use crate::core::metric_alias::OUT_ROLLTALS;
use crate::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        crate::cmds::cmd_for_top_trigrams_of_metric(msg, OUT_ROLLTALS, "outrolltals")
            .unwrap_or_else(|| self.help())
    }

    fn usage<'a>(&self) -> &'a str {
        "outrolltals <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the highest outrolltals for a particular layout"
    }
}