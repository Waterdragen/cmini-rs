use crate::util::{Commandable, Message};
use crate::util::memory::LIKES;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let id = msg.id;
        let name = &msg.author.name;
        let mut response = format!("```\n{name}'s liked layouts:\n");
        let likes = LIKES.read().unwrap();

        let mut liked_layouts = Vec::<&str>::new();
        for (layout, liked_users) in likes.iter() {
            if liked_users.contains(&id) {
                liked_layouts.push(layout);
            }
        }
        liked_layouts.sort();

        for layout in liked_layouts {
            response.push_str(layout);
            response.push('\n');
        }
        response.push_str("```");
        response
    }

    fn usage<'a>(&self) -> &'a str {
        "likes"
    }

    fn desc<'a>(&self) -> &'a str {
        "see a list of your liked layouts"
    }
}