use crate::util::core::{Commandable, DynCommand};

const LINK: &str = "<https://github.com/waterdragen/cmini-rs>";

pub struct Command;

impl Commandable for Command {
    fn init() -> DynCommand where Self: Sized + 'static {
        Box::new(Command{})
    }

    fn exec(&self, _: &str, _: u64) -> String {
        LINK.to_string()
    }

    fn usage<'a>(&self) -> &'a str {
        "github"
    }

    fn desc<'a>(&self) -> &'a str {
        "get the link of the cmini github repository"
    }
}