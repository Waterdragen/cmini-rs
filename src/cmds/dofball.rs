use crate::util::{Commandable, Message};
use rand::prelude::{SeedableRng, SliceRandom, StdRng};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let guild = match msg.guild(&msg.context) {
            None => return "No dofs here :(".to_owned(),
            Some(guild) => guild,
        };

        let emojis = guild.emojis
            .values()
            .filter(|emoji|
                emoji.available &&
                    emoji.name.to_ascii_lowercase().contains("dof"))
            .collect::<Vec<_>>();
        if emojis.is_empty() {
            return "No dofs in here :(".to_owned();
        }

        let mut rng = StdRng::from_entropy();
        let dof = match emojis.choose(&mut rng) {
            None => return "No dofs in here :(".to_owned(),
            Some(dof) => dof,
        };

        let prefix = if dof.animated { "a" } else { "" };

        format!("<{prefix}:{}:{}>", dof.name, dof.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "dofball [...]"
    }

    fn desc<'a>(&self) -> &'a str {
        "get a dof"
    }
}