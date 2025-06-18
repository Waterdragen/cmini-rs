use crate::util::{Commandable, Message};
use crate::util::links::LINKS;
use crate::util::memory::LAYOUTS;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let name = msg.arg;
        if name.is_empty() {
            return self.help();
        }
        let ll = LAYOUTS.find(name);
        let mut links = LINKS.write().unwrap();
        match links.remove(&ll.name) {
            None => format!("Error: {name} does not have a link"),
            Some(old_link) => format!("Link removed for {name}, previous link: {old_link}"),
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "unlink <layout>"
    }

    fn desc<'a>(&self) -> &'a str {
        "remove a link from a layout"
    }

    fn public_channel_only(&self) -> bool {
        true
    }

    fn mods_only(&self) -> bool {
        true
    }
}