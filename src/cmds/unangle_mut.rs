use crate::cmds::unangle::impl_unangle;
use crate::util::{Commandable, Message};
use crate::util::memory::{RemoveError, LAYOUTS};

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
        ll.to_pretty(msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "unangle! <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "view the non angle modded version of a layout"
    }
}