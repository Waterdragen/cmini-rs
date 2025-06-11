use std::sync::Arc;
use crate::util::{layout, memory, Commandable, Message};
use crate::util::parser::get_layout;
use crate::util::layout::check_name;
use crate::util::consts::{FMAP_STANDARD, FMAP_ANGLE, FREE_CHAR};
use crate::util::core::{Layout, LayoutConfig};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let (name, matrix) = get_layout(msg.arg);
        dbg!(&name);
        dbg!(&matrix);
        if let Err(err) = check_name(&name) {
            return err;
        }
        let rows = matrix.lines().collect::<Vec<_>>();

        let row_count = rows.len();

        if row_count < 3 {
            return format!("Expected 3 lines, got {}", row_count);
        }

        // Calculate amount of leading whitespace for each line
        let spaces = rows.iter().map(|row| {
            row.chars().take_while(|c| c.is_whitespace()).count()
        })
            .collect::<Vec<_>>();

        let mut max_rows = 3;

        let board = if spaces[0] < spaces[1] && spaces[1] < spaces[2] {
            "stagger".to_owned()
        } else if spaces[0] == spaces[1] && spaces[2] > 1 {
            max_rows = 4;
            "mini".to_owned()
        } else if spaces[0] == spaces[1] && spaces[1] < spaces[2] {
            "angle".to_owned()
        } else if spaces[0] == spaces[1] && spaces[1] == spaces[2] {
            max_rows = 3;
            "ortho".to_owned()
        } else {
            return "Error: board shape is undefined".to_owned();
        };

        if row_count > max_rows {
            return format!("Error: board type `{board}` supports at most {max_rows} rows, got {row_count}");
        }

        let mut keymap: Layout = Layout::default();
        for (row_idx, row) in rows[..3].iter().enumerate() {
            for (col_idx, ch) in row
                .chars()
                .filter(|c| *c != ' ' && *c != FREE_CHAR)
                .enumerate() {

                let fmap = if row_idx == 2 && board == "angle" {
                    &FMAP_ANGLE
                } else {
                    &FMAP_STANDARD
                };

                let finger = fmap[col_idx.min(9)];

                if keymap.insert(ch, (row_idx as u8, col_idx as u8, finger)).is_some() {
                    return format!("Error: `{ch}` is defined twice");
                }
            }
        }
        if max_rows == 4 {
            if let Some(thumb_row) = rows.get(3) {
                let finger = if spaces[3] > 8 { 4 } else { 5 };
                for (i, ch) in thumb_row.chars().filter(|c| *c != ' ' && *c != FREE_CHAR).enumerate() {
                    if keymap.insert(ch, (3, i as u8, finger)).is_some() {
                        return format!("Error: `{ch}` is defined twice");
                    }
                }
            }
        }

        let data = Arc::new(LayoutConfig::new(name.clone(), msg.id, board, keymap));
        if memory::add(data) {
            format!("Success!\n{}", layout::to_string(&memory::get(&name).unwrap(), msg.id))
        } else {
            format!("Error: `{name}` already exists")
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "add <layout> ``\u{200b}`keys`\u{200b}``"
    }

    fn desc<'a>(&self) -> &'a str {
        "contribute a new layout"
    }
}
