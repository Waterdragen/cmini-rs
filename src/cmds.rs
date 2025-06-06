mod _8ball;
mod _github;
mod _view;

use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use crate::util::core::{Commandable, DynCommand};

pub static COMMANDS: Lazy<FxHashMap<String, DynCommand>> = Lazy::new(|| {
    FxHashMap::from_iter([
        ("8ball", _8ball::Command::init()),
        ("gh", _github::Command::init()),
        ("github", _github::Command::init()),
        ("view", _view::Command::init()),
    ].into_iter().map(|(name, obj)| (name.to_string(), obj)))
});

pub fn get_cmd(name: &str) -> Option<&DynCommand> {
    COMMANDS.get(name)
}
