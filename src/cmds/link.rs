use crate::util::links::LINKS;
use crate::util::memory::LAYOUTS;
use crate::util::parser::split_words;
use crate::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let [name, new_link] = split_words(msg.arg);
        if name.is_empty() {
            return self.help();
        }
        if new_link.is_empty() {
            return "Error: link cannot be empty\n\n\
                    Help: use `link <layout_name> <link>` to add a link\n\
                    Help: use `unlink <layout_name>` to remove a link".to_owned()
        }
        let ll = &*LAYOUTS.find(name);
        let mut links = LINKS.write();
        match links.insert(ll.name.to_owned(), new_link.to_owned()) {
            None => format!("Link added for {}.", ll.name),
            Some(old_link) => {
                format!("Changed link for {}.\nPrevious link: {old_link}\nCurrent link: {new_link}",
                        ll.name)
            }
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "link <layout_name> <link>"
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