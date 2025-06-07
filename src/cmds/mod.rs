mod _8ball;
mod github;
mod help;
mod view;
pub mod maintenance;
mod suggest;
mod corpus;

use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use crate::util::core::{Commandable, DynCommand};

macro_rules! cmd {
    ($key:literal $module:ident) => {
        ($key, $module::Command.init())
    };
}

pub static COMMANDS: Lazy<FxHashMap<String, DynCommand>> = Lazy::new(|| {
    FxHashMap::from_iter([
        cmd!("8ball" _8ball),
        cmd!("corpus" corpus),
        cmd!("gh" github),
        cmd!("github" github),
        cmd!("help" help),
        cmd!("suggest" suggest),
        cmd!("view" view),
    ].into_iter().map(|(name, obj)| (name.to_string(), obj)))
});

pub fn get_cmd(name: &str) -> Option<&DynCommand> {
    COMMANDS.get(name)
}
