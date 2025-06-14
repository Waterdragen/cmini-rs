use crate::util::{Commandable, Message};
use crate::util::memory::{LAYOUTS, LIKES};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let name = msg.arg;
        if name.is_empty() {
            return self.help();
        }
        let ll = &*LAYOUTS.find(name);
        if ll.name == "qwerty" {
            return "You can't like Qwerty :yellow_circle:".to_owned();
        }
        let has_liked = {
            // Must drop or else deadlock
            let likes = LIKES.read().unwrap();
            match likes.get(&ll.name) {
                None => false,
                Some(liked_users) => liked_users.contains(&msg.id),
            }
        };
        if has_liked {
            return "You've already liked this layout".to_owned();
        }
        {
            let mut likes = LIKES.write().unwrap();
            let like_count = match likes.get_mut(&ll.name) {
                None => {
                    likes.insert(ll.name.clone(), vec![msg.id]);
                    1usize
                }
                Some(liked_users) => {
                    liked_users.push(msg.id);
                    liked_users.len()
                }
            };
            let s = if like_count == 1 { "" } else { "s" };
            format!("You liked {}. (Now at {like_count} like{s})", ll.name)
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "like <layout>"
    }

    fn desc<'a>(&self) -> &'a str {
        "like a layout"
    }
}