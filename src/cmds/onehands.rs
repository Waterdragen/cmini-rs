use crate::util::metric_alias::ONEHANDS;
use crate::{Commandable, Message};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        crate::cmds::cmd_for_top_trigrams_of_metric(msg, ONEHANDS, "onehands")
            .unwrap_or_else(|| self.help())
    }

    fn usage<'a>(&self) -> &'a str {
        "onehands <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the highest onehands for a particular layout"
    }
}