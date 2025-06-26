mod alias;
mod commandable;
mod corpus;
mod finger;
mod layout;
mod metric;
mod position;
mod stat;
mod table;

pub mod admins;
pub mod authors;
pub mod conv;
pub mod get;

pub use alias::*;
pub use commandable::Commandable;
pub use corpus::{Corpus, RawCorpus, WordCorpus};
pub use corpus::{RawServerCorpora, ServerCorpora, ServerWordCorpora};
pub use finger::{Finger, FingerCombo, FingerMap, FingerUnion, FingerUsage};
pub use layout::{Layout, LayoutConfig, ServerLayouts};
pub use metric::{ContainsMetric, Metric, MetricMap, MetricUnion};
pub use position::{Row, Col, Key, Position};
pub use stat::{CachedStat, CachedStatConfig, ServerCachedStats, Stat};
pub use table::Table;

pub trait ClonableIterator<'a, Item>: Iterator<Item = Item> {
    fn clone_iter(&self) -> Box<dyn ClonableIterator<'a, Item> + 'a>;
}

impl<'a, T, Item> ClonableIterator<'a, Item> for T
where
    T: Iterator<Item = Item> + Clone + 'a,
{
    fn clone_iter(&self) -> Box<dyn ClonableIterator<'a, Item> + 'a> {
        Box::new(self.clone())
    }
}
