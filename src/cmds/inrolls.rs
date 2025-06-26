use crate::{Commandable, Message};
use crate::core::Metric;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        crate::cmds::cmd_for_top_trigrams_of_metric(msg, Metric::InRoll, "Inrolls")
            .unwrap_or_else(|| self.help())
    }

    fn usage<'a>(&self) -> &'a str {
        "inrolls <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the highest inrolls for a particular layout"
    }
}