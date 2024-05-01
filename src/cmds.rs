mod _8ball;
mod _github;

use fxhash::FxHashMap;
use lazy_static::lazy_static;
use crate::util::core::Commandable;

lazy_static!(
    pub static ref COMMANDS: FxHashMap<String, Box<dyn Commandable + Send + Sync + 'static>> = FxHashMap::from_iter([
        ("8ball", _8ball::Command::init()),
        ("gh", _github::Command::init()),
        ("github", _github::Command::init()),
    ].into_iter().map(|(name, obj)| (name.to_string(), obj)));
);

pub fn get_cmd(name: &str) -> Option<&Box<dyn Commandable + Send + Sync>> {
    Some(COMMANDS.get(name)?)
}
