use crate::util::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, _: &Message) -> String {
        "<:woopwoop:1168675851592798288>".to_owned()
    }

    fn usage<'a>(&self) -> &'a str {
        "woopercat [...]"
    }

    fn desc<'a>(&self) -> &'a str {
        "get a woopercat"
    }
}