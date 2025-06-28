use crate::cmds::cycle::impl_cycle;
use crate::util::memory::{RemoveError, LAYOUTS};
use crate::util::parser::split_words;
use crate::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let [name, cycles] = split_words(msg.arg);
        let cycles = cycles.split_whitespace().collect::<Vec<_>>();

        if name.is_empty() {
            return self.help();
        }
        let ll = &mut*LAYOUTS.find_mut(name);
        if ll.user != msg.id {
            return RemoveError::NotOwner(&ll.name).to_string();
        }
        match impl_cycle(ll, &cycles) {
            Ok(_) => ll.to_pretty(msg.id) + "Successfully updated!",
            Err(err) => err.to_string(),
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "cycle! | swap! <layout_name> <chars>"
    }

    fn desc<'a>(&self) -> &'a str {
        "cycle a layout's letters around and update the original layout"
    }
}