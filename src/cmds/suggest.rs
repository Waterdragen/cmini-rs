use crate::util::Commandable;
use crate::util::Message;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, _: &Message) -> String {
        "Received :)".to_owned()
    }

    fn usage<'a>(&self) -> &'a str {
        "suggest <message>"
    }

    fn desc<'a>(&self) -> &'a str {
        "send me a suggestion for how to improve cmini :)"
    }
}