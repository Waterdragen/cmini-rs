use crate::util::memory::{AUTHORS, LAYOUTS};
use crate::util::parser::split_words;
use crate::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let [layout_name, mut author] = split_words(msg.arg);
        if layout_name.is_empty() || author.is_empty() {
            return self.help();
        }
        let mut layout = LAYOUTS.raw_get_mut(layout_name);
        let Some(layout) = layout.checked() else {
            return format!("Error: `{layout_name}` does not exist");
        };
        let authors = AUTHORS.read();

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
        layout.user = author_id;
        format!("`{layout_name}` has been assigned to `{author}`")
    }

    fn usage<'a>(&self) -> &'a str {
        "assign <layout_name> <author_name_or_id>"
    }

    fn desc<'a>(&self) -> &'a str {
        "assign a layout to a new author"
    }

    fn public_channel_only(&self) -> bool {
        true
    }

    fn mods_only(&self) -> bool {
        true
    }
}
