use crate::util::core::LayoutConfig;
use crate::util::memory::LAYOUTS;
use crate::util::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        if msg.arg.is_empty() {
            return self.help();
        }
        let mut ll = LAYOUTS.find(msg.arg).clone();
        impl_mirror(&mut ll);
        ll.name.push_str(" (mirrored)");
        ll.to_pretty(msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "mirror <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "view the mirrored version of a layout"
    }
}

pub(super) fn impl_mirror(ll: &mut LayoutConfig) {
    let is_angle = &ll.board == "angle";

    for (row, col, finger) in ll.keys.values_mut() {
        if *col >= 10 {
            continue;
        }
        if *row != 3 {
            *col = 9 - *col;
        }
        *finger = 9 - *finger;
        if is_angle && *row == 2 {
            // We want to unangle the new right, angle the new left
            // but the columns were flipped earlier
            // original -> flipped -> angled/unangled
            // (L) xcvbz -> (R) zbvcx -> (R) bvcxz
            // (R) nm,./ -> (L) /.,mn -> (L) .,mn/
            match *col {
                0 => { *col = 4; *finger = 3; },
                5 => { *col = 9; *finger = 9; },
                _ => { *col -= 1; }
            }
        }
    }
}
