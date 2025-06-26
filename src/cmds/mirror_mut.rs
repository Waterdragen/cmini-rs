use crate::cmds::mirror::impl_mirror;
use crate::{Commandable, Message};
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
        impl_mirror(&mut ll);
        ll.name.push_str(" (mirrored)");
        ll.to_pretty(msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "mirror! <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "mirror and update the original layout"
    }
}