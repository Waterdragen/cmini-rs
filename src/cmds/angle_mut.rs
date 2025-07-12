use crate::cmds::angle::impl_angle;
use crate::util::memory::LAYOUTS;
use crate::{Commandable, Message};
use cmini_core::layout::RemoveError;
use crate::util::layout::to_pretty;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        if msg.arg.is_empty() {
            return self.help();
        }
        let ll = &mut *LAYOUTS.find_mut(msg.arg);
        if ll.user != msg.id {
            return RemoveError::NotOwner(&ll.name).to_string();
        }
        match impl_angle(ll) {
            Ok(_) => to_pretty(ll, msg.id) + "Successfully updated!",
            Err(err) => err.to_string(),
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "angle! <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "angle mod and update the original layout"
    }
}