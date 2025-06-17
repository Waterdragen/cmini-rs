mod _8ball;
mod github;
mod help;
mod view;
pub mod maintenance;
mod suggest;
mod corpus;
mod add;
mod remove;
mod assign;
mod rename;
mod like;
mod likes;
mod unlike;
mod authors;
mod admin;
mod catball;
mod dofball;
mod wooperball;
mod random;

use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use crate::util::core::{Commandable, DynCommand};

pub static COMMANDS: Lazy<FxHashMap<String, DynCommand>> = Lazy::new(|| {
    FxHashMap::from_iter([
        ("8ball", _8ball::Command.init()),
        ("add", add::Command.init()),
        ("admin", admin::Command.init()),
        ("assign", assign::Command.init()),
        ("authors", authors::Command.init()),
        ("catball", catball::Command.init()),
        ("corpus", corpus::Command.init()),
        ("dofball", dofball::Command.init()),
        ("gh", github::Command.init()),
        ("github", github::Command.init()),
        ("help", help::Command.init()),
        ("like", like::Command.init()),
        ("likes", likes::Command.init()),
        ("random", random::Command.init()),
        ("remove", remove::Command.init()),
        ("rename", rename::Command.init()),
        ("suggest", suggest::Command.init()),
        ("unlike", unlike::Command.init()),
        ("wooperball", wooperball::Command.init()),
        ("view", view::Command.init()),
    ].into_iter().map(|(name, obj)| (name.to_string(), obj)))
});

pub fn get_cmd(name: &str) -> Option<&DynCommand> {
    COMMANDS.get(name)
}
