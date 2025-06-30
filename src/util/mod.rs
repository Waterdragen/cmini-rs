use crate::consts;

mod commandable;

pub mod cache;
pub mod corpora;
pub mod cmp;
pub mod jsons;
pub mod layout;
pub mod links;
pub mod memory;
pub mod minigames;
pub mod parser;
mod admins;
mod analyzer;

pub use commandable::Commandable;

pub fn validate_json() {
    let count = memory::ADMINS.count();
    assert_ne!(count, 0);
    let reader = memory::AUTHORS.read();
    assert!(!reader.is_empty());
    let reader = cache::CACHED_STATS.read();
    assert!(!reader.is_empty());
    let reader = &corpora::CORPORA;
    assert!(!reader.is_empty());
    let reader=  corpora::CORPORA_PREFS.read();
    assert!(!reader.is_empty());
    let _reader = &*consts::TABLE;  // Table is never empty
    let reader = links::LINKS.read();
    assert!(!reader.is_empty());
    let reader = memory::LAYOUTS.read();
    assert!(!reader.is_empty());
    let reader = memory::LIKES.read();
    assert!(!reader.is_empty());
}
