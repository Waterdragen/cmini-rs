use crate::util::Commandable;
use crate::util::{layout, Message};
use crate::util::memory::LAYOUTS;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let name = &msg.arg;
        if name.is_empty() {
            return self.help();
        }
        let ll = &*LAYOUTS.find(name);
        layout::to_string(ll, msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "view <layout name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the stats of a layout"
    }
}