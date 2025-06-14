use crate::util::authors::AUTHORS;
use crate::util::memory::LAYOUTS;
use crate::util::parser::split_word;
use crate::util::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let mut arg = msg.arg;
        let layout_name = split_word(&mut arg);
        let mut author = arg;

        if layout_name.is_empty() || author.is_empty() {
            return self.help();
        }
        if !LAYOUTS.contains(layout_name) {
            return format!("Error: `{layout_name}` does not exist");
        }
        let authors = AUTHORS.read().unwrap();

        let author_id = match author.parse::<u64>() {
            // Assign using ID
            Ok(author_id) => {
                match authors.get_name(author_id) {
                    None => return format!("Error: Author with ID `{author_id}` does not exist"),
                    Some(match_author_name) => author = match_author_name,
                }
                author_id
            }
            // Assign using name
            Err(_) => authors.get_id(author),
        };
        {
            let layout = &mut*LAYOUTS.get_mut(layout_name);  // always contains layout
            layout.user = author_id;
            format!("`{layout_name}` has been assigned to `{author}`")
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "assign <layout> <author>"
    }

    fn desc<'a>(&self) -> &'a str {
        "assign a layout to a new author"
    }

    fn mods_only(&self) -> bool {
        true
    }
}
