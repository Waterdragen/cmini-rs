use crate::util::{Commandable, Message};
use crate::util::memory::{LAYOUTS, LIKES};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let id = msg.id;
        let ll = &*LAYOUTS.find(msg.arg);
        {
            let mut likes = LIKES.write().unwrap();
            if let Some(liked_layouts) = likes.get_mut(&ll.name) {
                if let Some(idx) = liked_layouts
                    .iter()
                    .position(|liked_id| *liked_id == id) {
                    liked_layouts.remove(idx);
                    return format!("You unliked {}. (Now at {} likes)", ll.name, liked_layouts.len());
                }
            }
        }
        "You've already unliked this layout".to_owned()
    }

    fn usage<'a>(&self) -> &'a str {
        "unlike <layout>"
    }

    fn desc<'a>(&self) -> &'a str {
        "unlike a layout"
    }
}