use crate::util::memory::LAYOUTS;
use crate::util::Commandable;
use crate::util::Message;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let name = &msg.arg;
        if name.is_empty() {
            return self.help();
        }
        let ll = &*LAYOUTS.find(name);
        ll.to_pretty(msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "view <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the stats of a layout"
    }
}