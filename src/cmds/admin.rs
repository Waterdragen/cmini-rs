use crate::util::memory::{ADMINS, AUTHORS};
use crate::util::parser::split_words;
use crate::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let id = msg.id;
        let [action, target] = split_words(msg.arg);

        if target.is_empty() {
            return match ADMINS.list(id) {
                Ok(admin_names) => {
                    let mut s = "Admins:\n```\n".to_owned();
                    for admin_name in admin_names.iter() {
                        s.push_str(admin_name);
                        s.push('\n');
                    }
                    s.push_str("```");
                    s
                }
                Err(err) => err.to_string(),
            };
        }

        let target_id = id_from_name_or_str_id(target);
        match action {
            "add" => match ADMINS.add(id, target_id) {
                Ok(_) => {
                    let authors = AUTHORS.read();
                    let target_name = authors.get_name(target_id).unwrap();  // Checked by Admins::add
                    format!("Added `{target_name}` to admins") },
                Err(err) => err.to_string(),
            },
            "remove" => match ADMINS.remove(id, target_id) {
                Ok(_) => {
                    let authors = AUTHORS.read();
                    let target_name = authors.get_name(target_id).unwrap();  // Checked by Admins::add
                    format!("Removed `{target_name}` from admin")
                },
                Err(err) => err.to_string(),
            },
            _ => self.help(),
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "admin <add | remove> <name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "add or remove an admin"
    }

    fn mods_only(&self) -> bool {
        true
    }
}

fn id_from_name_or_str_id(target: &str) -> u64 {
    if target.chars().all(|c| c.is_ascii_digit()) {
        target.parse().unwrap()  // Always succeeds
    } else {
        let authors = AUTHORS.read();
        authors.get_id(target)
    }
}