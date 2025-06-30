use crate::util::memory::AUTHORS;
use crate::consts::{COL_LIMIT, FMAP_ANGLE, FMAP_STANDARD, FREE_CHAR, ROW_LIMIT};
use crate::core::{Layout, LayoutConfig, Position};
use crate::util::layout::check_name;
use crate::util::memory::LAYOUTS;
use crate::util::parser::get_layout;
use crate::{Commandable, Message};
use crate::core::Finger;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let (name, matrix) = get_layout(msg.arg);
        let (name, matrix) = (name.to_lowercase(), matrix.to_lowercase());
        if name.is_empty() {
            return self.help();
        }
        if let Err(err) = check_name(&name) {
            return err;
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
            "stagger".to_owned()
        } else if spaces[0] == spaces[1] && spaces[2].saturating_sub(spaces[1]) > 1 {
            "mini".to_owned()
        } else if spaces[0] == spaces[1] && spaces[2].saturating_sub(spaces[1]) == 1 {
            "angle".to_owned()
        } else if spaces[0] == spaces[1] && spaces[1] == spaces[2] {
            "ortho".to_owned()
        } else {
            return "Error: board shape is undefined".to_owned();
        };

        let mut keymap: Layout = Layout::default();
        for (row_idx, row) in rows[..3].iter().enumerate() {
            let mut row_iter = row
                .chars()
                .filter(|c| *c != ' ')
                .enumerate()
                .filter(|(_, c)| *c != FREE_CHAR);
            for (col_idx, ch) in row_iter.by_ref().take(COL_LIMIT) {
                if col_idx >= COL_LIMIT {
                    return format!("Error: expected at most {COL_LIMIT} columns, got {}", col_idx + row_iter.count());
                }
                let fmap = match row_idx == 2 && board == "angle" {
                    true => &FMAP_ANGLE,
                    false => &FMAP_STANDARD,
                };
                let finger = fmap[col_idx.min(9)];

                if keymap.insert(ch, Position::new(row_idx as u8, col_idx as u8, finger)).is_some() {
                    return format!("Error: `{ch}` is defined twice");
                }
            }
        }
        if let Some(thumb_row) = rows.get(3) {
            let mut thumb_iter = thumb_row.chars()
                .filter(|&c| c != ' ')
                .enumerate()
                .filter(|(_, c)| *c != FREE_CHAR);
            for (col_idx, ch) in thumb_iter.by_ref() {
                let finger = if col_idx < 5 { Finger::LT } else { Finger::RT };
                if col_idx >= COL_LIMIT {
                    return format!("Error: expected at most {COL_LIMIT} columns, got {}", col_idx + thumb_iter.count());
                }
                if keymap.insert(ch, Position::new(3, col_idx as u8, finger)).is_some() {
                    return format!("Error: `{ch}` is defined twice");
                }
            }
        }

        let data = LayoutConfig::new(name.clone(), msg.id, board, keymap);
        if LAYOUTS.add(data) {
            {
                // Must drop or else deadlock
                let mut authors = AUTHORS.write();
                authors.update(msg.id, &msg.author.name);
            }
            format!("Success!\n{}", LAYOUTS.get(&name).to_pretty(msg.id))
        } else {
            format!("Error: `{name}` already exists")
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "add <layout_name> ``\u{200b}`keys`\u{200b}``"
    }

    fn desc<'a>(&self) -> &'a str {
        "contribute a new layout"
    }
}
