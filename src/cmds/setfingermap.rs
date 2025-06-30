use crate::consts::{COL_LIMIT, FREE_CHAR, ROW_LIMIT};
use crate::core::finger_alias::THUMB;
use crate::core::Finger;
use crate::message::Message;
use crate::prelude::Commandable;
use crate::util::memory::{RemoveError, LAYOUTS};
use crate::util::parser::get_layout;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let (name, matrix) = get_layout(msg.arg);
        if name.is_empty() {
            return self.help();
        }
        let mut ll = LAYOUTS.find(&name).clone();
        if ll.user != msg.id {
            return RemoveError::NotOwner(&ll.name).to_string();
        }
        let rows = matrix.lines().collect::<Vec<_>>();
        let row_count = rows.len();
        if row_count < 3 {
            return format!("Error: expected at least 3 lines, got {}", row_count);
        }
        if row_count > ROW_LIMIT {
            return format!("Error: expected at most {ROW_LIMIT} rows, got {row_count}");
        }

        // Calculate amount of leading whitespace for each line
        let spaces = rows.iter().map(|row| {
            row.chars().take_while(|c| c.is_whitespace()).count()
        })
            .collect::<Vec<_>>();

        let board = if spaces[0] < spaces[1] && spaces[1] < spaces[2] {
            "stagger"
        } else if spaces[0] == spaces[1] && spaces[2].saturating_sub(spaces[1]) > 1 {
            "mini"
        } else if spaces[0] == spaces[1] && spaces[2].saturating_sub(spaces[1]) == 1 {
            "angle"
        } else if spaces[0] == spaces[1] && spaces[1] == spaces[2] {
            "ortho"
        } else {
            return "Error: board shape is undefined".to_owned();
        };

        if ll.board != board {
            return format!("Error: board shape ({}) is different from original ({})", board, ll.board);
        }
        let matrix = ll.matrix();
        for (row_idx, row) in rows.iter().enumerate() {
            let mut row_iter = row
                .chars()
                .filter(|c| *c != ' ')
                .enumerate()
                .filter(|(_, c)| *c != FREE_CHAR);
            for (col_idx, ch) in row_iter.by_ref().take(COL_LIMIT) {
                if col_idx >= COL_LIMIT {
                    return format!("Error: expected at most {COL_LIMIT} columns, got {}", col_idx + row_iter.count());
                }
                let finger = match ch.to_digit(10) {
                    None => return format!("Error: invalid finger `{ch}`, expected 0 to 9"),
                    Some(finger_num) => Finger::from_u8(finger_num as u8),
                };
                match ((row_idx == 3), THUMB.contains(finger)) {
                    (true, false) => return "Error: only thumb values are allowed on row 3".to_owned(),
                    (false, true) => return "Error: no thumb values are allowed on rows 0-2".to_owned(),
                    _ => {}
                }
                let key = matrix[row_idx][col_idx];  // key may be ' '
                let Some(pos) =  ll.keys.get_mut(&key) else {
                    return format!("Error: `{}` does not have a key at position (row {row_idx}, column {col_idx})", ll.name);
                };
                pos.finger = finger;  // row and col bounds has been checked
            }
        }

        let name = ll.name.to_owned();
        let mut new_ll = LAYOUTS.get_mut(&name);
        *new_ll = ll;
        let header = new_ll.header();
        let matrix = new_ll.matrix_str();
        let finger_matrix = new_ll.finger_matrix_str();
        format!("Success!\n\
                ```\n\
                {header}\n\
                {matrix}\
                \n\
                {finger_matrix}\
                ```")
    }

    fn usage<'a>(&self) -> &'a str {
        "setfingermap <layout_name> <``\u{200b}`finger_matrix`\u{200b}``>"
    }

    fn desc<'a>(&self) -> &'a str {
        "set the fingermap of a layout"
    }
}