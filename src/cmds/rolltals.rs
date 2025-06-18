use crate::util::{Commandable, Message};
use crate::util::core::Metric;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        crate::cmds::cmd_for_top_trigrams_of_metric(
            msg,
            Metric::InRoll | Metric::OutRoll | Metric::InOne | Metric::OutOne,
            "rolltals"
        ).unwrap_or_else(|| self.help())
    }

    fn usage<'a>(&self) -> &'a str {
        "rolltals <layout>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the highest rolltals for a particular layout"
    }
}