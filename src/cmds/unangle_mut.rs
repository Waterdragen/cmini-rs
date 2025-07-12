use crate::cmds::unangle::impl_unangle;
use crate::{Commandable, Message};
use crate::util::memory::LAYOUTS;
use cmini_core::layout::RemoveError;
use crate::util::layout::to_pretty;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        if msg.arg.is_empty() {
            return self.help();
        }
        let mut ll = LAYOUTS.find_mut(msg.arg);
        if ll.user != msg.id {
            return RemoveError::NotOwner(&ll.name).to_string();
        }
        impl_unangle(&mut ll);
        ll.name.push_str(" (non angle modded)");
        to_pretty(&ll, msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "unangle! <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "view the non angle modded version of a layout"
    }
}