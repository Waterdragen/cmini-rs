use crate::util::{Commandable, Message};
use crate::util::links::LINKS;
use crate::util::memory::LAYOUTS;
use crate::util::parser::split_word;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let mut arg = msg.arg;
        let name = split_word(&mut arg);
        let new_link = arg;

        if name.is_empty() {
            return self.help();
        }
        if new_link.is_empty() {
            return "Error: link cannot be empty\n\n\
                    Help: use `link <layout> <link>` to add a link\n\
                    Help: use `unlink <layout>` to remove a link".to_owned()
        }
        let ll = &*LAYOUTS.find(name);
        let mut links = LINKS.write().unwrap();
        match links.insert(ll.name.to_owned(), new_link.to_owned()) {
            None => format!("Link added for {}.", ll.name),
            Some(old_link) => {
                format!("Changed link for {}.\nPrevious link: {old_link}\nCurrent link: {new_link}",
                        ll.name)
            }
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "link <layout> <link>"
    }

    fn desc<'a>(&self) -> &'a str {
        "add a link to a layout"
    }

    fn public_channel_only(&self) -> bool {
        true
    }

    fn mods_only(&self) -> bool {
        true
    }
}