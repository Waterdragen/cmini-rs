use crate::{Commandable, Message};
use crate::util::memory::LAYOUTS;
use cmini_core::{Finger, LayoutConfig};
use crate::util::layout::to_pretty;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        if msg.arg.is_empty() {
            return self.help();
        }
        let mut ll = LAYOUTS.find(msg.arg).clone();
        impl_unangle(&mut ll);
        ll.name.push_str(" (non angle modded)");
        to_pretty(&ll, msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "unangle <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "view the non angle modded version of a layout"
    }
}

pub(super) fn impl_unangle(ll: &mut LayoutConfig) {
    if &ll.board != "angle" {
        return;
    }
    ll.board = "ortho".to_owned();
    for pos in ll.keys.values_mut() {
        if pos.row != 2 {
            continue;
        }
        let col = &mut pos.col;
        if *col >= 5 {
            continue;
        }
        if *col == 4 {
            *col = 0;
            pos.finger = Finger::LP;
        } else {
            *col += 1;
        }
    }
}