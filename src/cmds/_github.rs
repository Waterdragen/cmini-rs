use crate::util::core::Commandable;

const LINK: &'static str = "<https://github.com/waterdragen/cmini-rs>";

pub struct Command;

impl Commandable for Command {
    fn init() -> Box<dyn Commandable + Send + Sync + 'static> where Self: Sized + 'static {
        Box::new(Command{})
    }

    fn exec(&self, _args: &str) -> String {
        LINK.to_string()
    }

    fn usage<'a>(&self) -> &'a str {
        "github"
    }

    fn desc<'a>(&self) -> &'a str {
        "get the link of the cmini github repository"
    }
}