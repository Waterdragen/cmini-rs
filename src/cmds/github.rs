use crate::{Commandable, Message};

const LINK: &str = "<https://github.com/waterdragen/cmini-rs>";

pub struct Command;

impl Commandable for Command {
    fn exec(&self, _: &Message) -> String {
        LINK.to_string()
    }

    fn usage<'a>(&self) -> &'a str {
        "github"
    }

    fn desc<'a>(&self) -> &'a str {
        "get the link of the cmini github repository"
    }
}