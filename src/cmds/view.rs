use crate::util::core::{Commandable, DynCommand};
use crate::util::{layout, memory};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, name: &str, id: u64) -> String {
        let ll = memory::find(name);
        layout::to_string(&ll, id)
    }

    fn usage<'a>(&self) -> &'a str {
        "view [layout name]"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the stats of a layout"
    }
}