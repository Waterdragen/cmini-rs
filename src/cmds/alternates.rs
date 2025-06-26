use crate::core::Metric;
use crate::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        crate::cmds::cmd_for_top_trigrams_of_metric(msg, Metric::Alt, "Alternates")
            .unwrap_or_else(|| self.help())
    }

    fn usage<'a>(&self) -> &'a str {
        "alternates <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the highest alternates for a particular layout"
    }
}