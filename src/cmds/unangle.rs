use crate::util::{Commandable, Message};
use crate::util::core::LayoutConfig;
use crate::util::memory::LAYOUTS;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        if msg.arg.is_empty() {
            return self.help();
        }
        let mut ll = LAYOUTS.find(msg.arg).clone();
        impl_unangle(&mut ll);
        ll.name.push_str(" (non angle modded)");
        ll.to_pretty(msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "unangle <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "view the non angle modded version of a layout"
    }
}

pub(super) fn impl_unangle(ll: &mut LayoutConfig) {
    const LP: u16 = 0;
    if &ll.board != "angle" {
        return;
    }
    ll.board = "ortho".to_owned();
    for key in ll.keys.values_mut() {
        if key.0 != 2 {
            continue;
        }
        let col = &mut key.1;
        if *col >= 5 {
            continue;
        }
        if *col == 4 {
            *col = 0;
            key.2 = LP;
        } else {
            *col += 1;
        }
    }
}