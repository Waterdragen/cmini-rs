use crate::util::metric_alias::IN_ROLLTALS;
use crate::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        crate::cmds::cmd_for_top_trigrams_of_metric(msg, IN_ROLLTALS, "inrolltals")
            .unwrap_or_else(|| self.help())
    }

    fn usage<'a>(&self) -> &'a str {
        "inrolltals <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the highest inrolltals for a particular layout"
    }
}