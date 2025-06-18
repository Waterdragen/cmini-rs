use crate::util::{Commandable, Message};
use crate::util::core::Metric;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        crate::cmds::cmd_for_top_trigrams_of_metric(
            msg,
            Metric::AltSfs | Metric::RedSfs | Metric::BadRedSfs,
            "sfs"
        ).unwrap_or_else(|| self.help())
    }

    fn usage<'a>(&self) -> &'a str {
        "sfs <layout>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the highest sfs for a particular layout"
    }
}