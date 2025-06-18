use crate::util::{Commandable, Message};
use crate::util::core::Metric;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        crate::cmds::cmd_for_top_trigrams_of_metric(msg, Metric::InRoll | Metric::InOne, "inrolltals")
            .unwrap_or_else(|| self.help())
    }

    fn usage<'a>(&self) -> &'a str {
        "inrolltals <layout>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the highest inrolltals for a particular layout"
    }
}