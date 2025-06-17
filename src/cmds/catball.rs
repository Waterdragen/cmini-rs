use crate::util::{Commandable, Message};
use rand::prelude::{SeedableRng, SliceRandom, StdRng};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let guild = match msg.guild(&msg.context) {
            None => return "No cats in here :(".to_owned(),
            Some(guild) => guild,
        };

        let emojis = guild.emojis
            .values()
            .filter(|emoji|
                emoji.available &&
                emoji.name.to_ascii_lowercase().contains("cat"))
            .collect::<Vec<_>>();
        if emojis.is_empty() {
            return "No cats in here :(".to_owned();
        }

        let mut rng = StdRng::from_entropy();
        let cat = match emojis.choose(&mut rng) {
            None => return "No cats in here :(".to_owned(),
            Some(cat) => cat,
        };

        let prefix = if cat.animated { "a" } else { "" };
        
        format!("<{prefix}:{}:{}>", cat.name, cat.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "catball [...]"
    }

    fn desc<'a>(&self) -> &'a str {
        "get a cat"
    }
}