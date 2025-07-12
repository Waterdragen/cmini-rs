mod alias;
mod analyzer;
mod corpus;
mod finger;
mod key;
mod metric;
mod position;
mod stat;
mod table;

pub mod conv;
pub mod consts;
pub mod finger_alias;
pub mod get;
pub mod keysolve;
pub mod metric_alias;
pub mod layout;
pub mod lock;
pub mod response;

pub use alias::*;
pub use corpus::{Corpus, RawCorpus, WordCorpus};
pub use corpus::{RawServerCorpora, ServerCorpora, ServerWordCorpora};
pub use finger::{Finger, FingerCombo, FingerMap, FingerUnion, FingerUsage};
pub use key::{Key, KeyPat};
pub use layout::{Layout, LayoutConfig, ServerLayouts};
pub use metric::{ContainsMetric, Metric, MetricMap, MetricUnion};
pub use position::{Col, Position, Row};
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
