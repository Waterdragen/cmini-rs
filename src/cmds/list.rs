use crate::message::Message;
use crate::prelude::{BoundedResponse, Commandable};
use crate::util::memory::{AUTHORS, LAYOUTS};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let authors = AUTHORS.read();
        let (id, name) = if msg.arg.is_empty() {
            let Some(name) = authors.get_name(msg.id) else {
                return format!("{}'s layouts:\n```\n```", msg.author.name);
            };
            (msg.id, name)
        } else {
            let id = authors.get_id(msg.arg);
            (id, authors.get_name(id).unwrap())  // id is always valid, so get_name is always Some(name)
        };
        let layouts = LAYOUTS.read();
        let mut owned_layouts = layouts.iter()
            .filter_map(|(name, ll)| (ll.user == id).then_some(name))
            .collect::<Vec<_>>();
        owned_layouts.sort_unstable();

        let mut output = BoundedResponse::from(format!("{name}'s layouts:\n```\n")).reserve(50);
        let mut hidden = 0;
        for layout in owned_layouts {
            let _ = output.push_str(layout);
            if output.push('\n').is_err() {
                hidden += 1;
            }
        }
        let mut output = output.finish();
        if hidden != 0 {
            output.push_str(&format!("(... {hidden} more)\n"));
        }
        output.push_str("```");
        output
    }

    fn usage<'a>(&self) -> &'a str {
        "list [user_name]"
    }

    fn desc<'a>(&self) -> &'a str {
        "see a list of a user's layouts"
    }
}