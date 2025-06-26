use crate::core::ServerLayouts;
use crate::core::authors::Authors;
use crate::prelude::*;
use crate::util::corpora::CORPORA_PREFS;
use crate::util::jsons::{read_json, write_json};
use crate::util::links::LINKS;
use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use std::fmt::Debug;
use thiserror::Error;
use crate::core::admins::Admins;

pub static AUTHORS: Lazy<Arc<RwLock<Authors>>> = Lazy::new(||
    Arc::new(RwLock::new(Authors::open("./authors.json").unwrap()))
);
pub static ADMINS: Lazy<Admins> = Lazy::new(|| Admins::open("admins.json"));

pub static LAYOUTS: Lazy<ServerLayouts> = Lazy::new(|| read_json("./layouts.json"));
pub static LIKES: Lazy<Arc<RwLock<FxHashMap<String, Vec<u64>>>>> = Lazy::new(|| read_json("./likes.json"));

#[derive(Debug, Error)]
pub enum RemoveError<'a> {
    #[error("Error: `{0}` does not exist")]
    NotFound(&'a str),
    #[error("Error: you don't own `{0}`")]
    NotOwner(&'a str),
    #[error("Use commands with `--sudo` in a public channel")]
    SudoInPrivateChannel,
}

type IsInPublicChannel = bool;

pub fn get_like_count(name: &str) -> usize {
    let likes = LIKES.read();
    match likes.get(name) {
        Some(liked_users) => liked_users.len(),
        None => 0,
    }
}

pub fn sync_data() {
    write_json("./admins.json", &*ADMINS);
    write_json("./authors.json", &*AUTHORS);
    write_json("./corpora.json", &*CORPORA_PREFS);
    write_json("./layouts.json", &*LAYOUTS);
    write_json("./likes.json", &*LIKES);
    write_json("./links.json", &*LINKS);
}
