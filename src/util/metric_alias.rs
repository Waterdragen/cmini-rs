use std::borrow::ToOwned;
use std::iter::IntoIterator;
use itertools::Itertools;
use once_cell::sync::Lazy;
use crate::core::{FxIndexMap, MetricUnion, Metric as M};

const fn raw_bit(metric: M) -> u32 {
    1 << metric as u32
}

const ALT_SFS: u32 = raw_bit(M::AltSfs);
const RED: u32 = raw_bit(M::Red);
const BAD_RED: u32 = raw_bit(M::BadRed);
const RED_SFS: u32 = raw_bit(M::RedSfs);
const BAD_RED_SFS: u32 = raw_bit(M::BadRedSfs);
const IN_ONE: u32 = raw_bit(M::InOne);
const OUT_ONE: u32 = raw_bit(M::OutOne);
const IN_ROLL: u32 = raw_bit(M::InRoll);
const OUT_ROLL: u32 = raw_bit(M::OutRoll);

pub const SFS: MetricUnion = MetricUnion(ALT_SFS | RED_SFS | BAD_RED_SFS);
pub const ONEHANDS: MetricUnion = MetricUnion(IN_ONE | OUT_ONE);
pub const REDIRECTS: MetricUnion = MetricUnion(RED | BAD_RED | RED_SFS | BAD_RED_SFS);
pub const ROLLS: MetricUnion = MetricUnion(IN_ROLL | OUT_ROLL);
pub const IN_ROLLTALS: MetricUnion = MetricUnion(IN_ROLL | IN_ONE);
pub const OUT_ROLLTALS: MetricUnion = MetricUnion(OUT_ROLL | OUT_ONE);
pub const ROLLTALS: MetricUnion = MetricUnion(IN_ROLL | OUT_ROLL | IN_ONE | OUT_ONE);

// Use IndexMap to list metric with dedup_by
pub static METRIC_NAMES: Lazy<FxIndexMap<String, MetricUnion>> = Lazy::new(|| FxIndexMap::from_iter([
    ("alt", M::Alt.into()),
    ("alts", M::Alt.into()),
    ("alternate", M::Alt.into()),
    ("alternates", M::Alt.into()),
    ("sfb", M::Sfb.into()),
    ("sfbs", M::Sfb.into()),
    ("sft", M::Sft.into()),
    ("sfs", SFS),
    ("dsfb", SFS),
    ("sfr", M::Sfr.into()),
    ("red", REDIRECTS),
    ("redirect", REDIRECTS),
    ("redirects", REDIRECTS),
    ("oneh", ONEHANDS),
    ("onehand", ONEHANDS),
    ("onehands", ONEHANDS),
    ("inroll", M::InRoll.into()),
    ("inrolls", M::InRoll.into()),
    ("roll-in", M::InRoll.into()),
    ("outroll", M::OutRoll.into()),
    ("outrolls", M::OutRoll.into()),
    ("roll-out", M::OutRoll.into()),
    ("inrolltal", IN_ROLLTALS),
    ("inrolltals", IN_ROLLTALS),
    ("outrolltal", OUT_ROLLTALS),
    ("outrolltals", OUT_ROLLTALS),
    ("rolls", ROLLS),
    ("roll", ROLLS),
    ("roll-total", ROLLS),
    ("rolltal", ROLLTALS),
    ("rolltals", ROLLTALS),
    ("rolltotal", ROLLTALS),
].into_iter().map(|(s, metric)| (s.to_owned(), metric))));

pub fn list_metric_union_names<'a>() -> impl Iterator<Item = &'a str> {
    METRIC_NAMES.iter()
        .dedup_by(|(_, &metric0), (_, &metric1)| {
        metric0 == metric1
        })
        .map(|(name, _)| name.as_str())
}