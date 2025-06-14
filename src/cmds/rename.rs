use crate::util::layout::check_name;
use crate::util::memory::LAYOUTS;
use crate::util::parser::split_word;
use crate::util::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let mut arg = msg.arg;
        let old = split_word(&mut arg);
        let new = arg;

        if let Err(err) = check_name(new) {
            return err;
        }
        match LAYOUTS.remove(old, msg.id) {
            Err(err) => err.to_string(),
            Ok(mut layout) => {
                if LAYOUTS.contains(new) {
                    return format!("Error: `{new}` already exists")
                }
                layout.name = new.to_owned();
                LAYOUTS.add(layout);  // Add always succeed
                format!("`{old}` has been renamed to `{new}`")
            }
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "rename <old_layout> <new_layout>"
    }

    fn desc<'a>(&self) -> &'a str {
        "rename one of your layouts"
    }
}