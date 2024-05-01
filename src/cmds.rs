mod _8ball;
mod _github;

use fxhash::FxHashMap;
use lazy_static::lazy_static;
use crate::util::core::{Commandable, DynCommand};

lazy_static!(
    pub static ref COMMANDS: FxHashMap<String, DynCommand> = FxHashMap::from_iter([
        ("8ball", _8ball::Command::init()),
        ("gh", _github::Command::init()),
        ("github", _github::Command::init()),
    ].into_iter().map(|(name, obj)| (name.to_string(), obj)));
);

pub fn get_cmd(name: &str) -> Option<&DynCommand> {
    Some(COMMANDS.get(name)?)
}
