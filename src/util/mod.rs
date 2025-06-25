pub mod admins;
pub mod analyzer;
pub mod authors;
pub mod cache;
pub mod consts;
pub mod core;
pub mod corpora;
pub mod jsons;
pub mod layout;
pub mod links;
pub mod memory;
pub mod parser;
mod conv;
mod message;
mod get;
pub mod metric_alias;

pub use core::Commandable;
pub use message::{Message, BoundedResponse, BoundedResponseVec};

pub fn validate_json() {
    let count = admins::ADMINS.count();
    assert_ne!(count, 0);
    let reader = authors::AUTHORS.read();
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
