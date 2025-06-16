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

pub use core::Commandable;
pub use message::{Message, BoundedResponse};

pub fn validate_json() {
    let count = admins::ADMINS.count();
    assert_ne!(count, 0);
    let reader = authors::AUTHORS.read().unwrap();
    assert!(!reader.is_empty());
    let reader = cache::CACHED_STATS.read().unwrap();
    assert!(!reader.is_empty());
    let reader = corpora::CORPORA.as_slice();
    assert!(!reader.is_empty());
    let reader=  corpora::CORPORA_PREFS.read().unwrap();
    assert!(!reader.is_empty());
    let reader = consts::TABLE.as_ref();
    assert!(!reader.is_empty());
    let reader = links::LINKS.read().unwrap();
    assert!(!reader.is_empty());
    let reader = memory::LAYOUTS.read().unwrap();
    assert!(!reader.is_empty());
    let reader = memory::LIKES.read().unwrap();
    assert!(!reader.is_empty());
}
