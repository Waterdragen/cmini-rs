use crate::core::metric_alias::ROLLTALS;
use crate::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        crate::cmds::cmd_for_top_trigrams_of_metric(msg, ROLLTALS, "rolltals")
            .unwrap_or_else(|| self.help())
    }

    fn usage<'a>(&self) -> &'a str {
        "rolltals <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the highest rolltals for a particular layout"
    }
}