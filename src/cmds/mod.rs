mod _8ball;
mod github;
mod help;
mod view;
pub mod maintenance;
mod suggest;
mod corpus;
mod add;

use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use crate::util::core::{Commandable, DynCommand};

pub static COMMANDS: Lazy<FxHashMap<String, DynCommand>> = Lazy::new(|| {
    FxHashMap::from_iter([
        ("8ball", _8ball::Command.init()),
        ("add", add::Command.init()),
        ("corpus", corpus::Command.init()),
        ("gh", github::Command.init()),
        ("github", github::Command.init()),
        ("help", help::Command.init()),
        ("suggest", suggest::Command.init()),
        ("view", view::Command.init()),
    ].into_iter().map(|(name, obj)| (name.to_string(), obj)))
});

pub fn get_cmd(name: &str) -> Option<&DynCommand> {
    COMMANDS.get(name)
}
