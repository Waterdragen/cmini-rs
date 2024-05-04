use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};

use indexmap::IndexMap;
use fxhash::{FxBuildHasher, FxHashMap};
use nohash_hasher::NoHashHasher;
use num_enum::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::util::conv;

pub type Row = u8;
pub type Col = u8;
pub type Finger = u16;
pub type Key = char;
pub type Position = (Row, Col, Finger);
pub type RawCorpus = Vec<(Vec<Key>, u64)>;
pub type Corpus = Arc<RawCorpus>;

pub type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
pub type NoHashMap<K, V> = HashMap<K, V, NoHashHasher<K>>;
pub type SyncFxMap<K, V> = Arc<RwLock<FxHashMap<K, Arc<V>>>>;
pub type SyncIndexMap<K, V> = Arc<RwLock<FxIndexMap<K, Arc<V>>>>;

pub type Layout = FxHashMap<Key, Position>;
pub type LayoutConfig = Arc<RawLayoutConfig>;
pub type Stat = FxHashMap<Metric, f64>;
pub type FingerUsage = FxHashMap<Finger, f64>;
pub type CachedStats = FxHashMap<String, Arc<Stat>>;
pub type CachedStatConfig = Arc<RawCachedStatConfig>;

pub type ServerCorpora = SyncFxMap<String, RawCorpus>;
pub type ServerLayouts = SyncIndexMap<String, RawLayoutConfig>;
pub type ServerCachedStats = SyncIndexMap<String, RawCachedStatConfig>;

// Trait: Commandable
// Struct: Command
// Instance Smart Pointer: DynCommand
pub type DynCommand = Box<dyn Commandable + Send + Sync + 'static>;

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonLayoutConfig {
    pub user: u64,
    pub board: String,
    pub keys: String,
}

impl JsonLayoutConfig {
    pub fn from_raw(layout_config: &RawLayoutConfig) -> Self {
        JsonLayoutConfig {
            user: layout_config.user,
            board: layout_config.board.clone(),
            keys: conv::layout::pack(&layout_config.keys),
        }
    }
}

pub struct RawLayoutConfig {
    pub name: String,
    pub user: u64,
    pub board: String,
    pub keys: Layout,
    pub sum: u64,
}

impl RawLayoutConfig {
    pub fn from_json(name: &str, json_layout: JsonLayoutConfig) -> Self {
        RawLayoutConfig {
            name: name.to_string(),
            user: json_layout.user,
            board: json_layout.board.clone(),
            keys: conv::layout::unpack(&json_layout.keys),
            sum: conv::hash_keys(&json_layout.keys),
        }
    }
}

impl Serialize for RawLayoutConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        JsonLayoutConfig::from_raw(&self).serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonCachedStatConfig {
    pub sum: u64,
    pub stats: FxIndexMap<String, String>,
}

impl JsonCachedStatConfig {
    pub fn from_raw(cached_stat_config: &RawCachedStatConfig) -> Self {
        let mut stats: FxIndexMap<String, String> = FxIndexMap::from_iter(
            cached_stat_config.stats.iter().map(
                |(corpus, stat)| (corpus.clone(), conv::stats::pack(stat))
            ));
        stats.sort_unstable_keys();
        JsonCachedStatConfig {
            sum: cached_stat_config.sum,
            stats,
        }
    }
}

#[derive(Debug, Default)]
pub struct RawCachedStatConfig {
    pub sum: u64,
    pub stats: CachedStats,
}

impl RawCachedStatConfig {
    pub fn from_json(json: JsonCachedStatConfig) -> Self {
        RawCachedStatConfig {
            sum: json.sum,
            stats: json.stats.into_iter()
                .map(|(corpus, packed)| (corpus, Arc::new(conv::stats::unpack(&packed))))
                .collect()
        }
    }
}

impl Serialize for RawCachedStatConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        JsonCachedStatConfig::from_raw(self).serialize(serializer)
    }
}

#[derive(PartialEq)]
pub enum ArgType {
    Str,
    Vec,
}

#[derive(PartialEq)]
pub enum KwargType {
    Bool,
    Vec,
    Str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kwarg {
    Bool(bool),
    Vec(Vec<String>),
    Str(String),
    Default,
}

impl Kwarg {
    pub fn as_bool(&self) -> bool {
        match self {
            Kwarg::Bool(b) => *b,
            _ => panic!("{self:?} is not a bool")
        }
    }

    pub fn as_vec(&self) -> &Vec<String> {
        match self {
            Kwarg::Vec(v) => v,
            _ => panic!("{self:?} is not a vec")
        }
    }

    pub fn as_string(&self) -> &str {
        match self {
            Kwarg::Str(s) => s,
            _ => panic!("{self:?} is not a string")
        }
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumIter, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Metric {
    Sfb,
    Sft,
    Sfr,
    Alt,
    AltSfs,
    Red,
    BadRed,
    RedSfs,
    BadRedSfs,
    InOne,
    OutOne,
    InRoll,
    OutRoll,
    Unknown,
}

impl Metric {
    pub fn from_str(s: &str) -> Self {
        match s {
            "sfb" => Metric::Sfb,
            "sft" => Metric::Sft,
            "sfr" => Metric::Sfr,
            "alt" => Metric::Alt,
            "alt-sfs" => Metric::AltSfs,
            "red" => Metric::Red,
            "bad-red" => Metric::BadRed,
            "red-sfs" => Metric::RedSfs,
            "bad-red-sfs" => Metric::BadRedSfs,
            "inoneh" => Metric::InOne,
            "outoneh" => Metric::OutOne,
            "inroll" => Metric::InRoll,
            "outroll" => Metric::OutRoll,
            "unknown" => Metric::Unknown,
            _ => panic!("Invalid metric {s}")
        }
    }

    pub fn pack(self) -> char {
        let num: u8 = self.into();
        let hex = format!("{:01x}", num);
        hex.chars().next().unwrap()
    }

    pub fn unpack(c: char) -> Self {
        let s = String::from(c);
        let num = u8::from_str_radix(&s, 16).unwrap_or_else(|_| panic!("Failed to convert to u8. Unexpected value '{c}'"));
        Metric::try_from(num).unwrap_or_else(|_| panic!("Failed to convert to Metric. Unexpected value `{num}`"))
    }

    pub fn new_counter() -> FxHashMap<Metric, u64> {
        FxHashMap::from_iter(Metric::iter().map(|metric| {
            (metric, 0u64)
        }))
    }

    pub fn normalize_counter(counter: &FxHashMap<Metric, u64>) -> Stat {
        let total = counter.values().sum::<u64>() as f64;
        debug_assert_ne!(total, 0.0);
        FxHashMap::from_iter(counter.iter().map(|(metric, freq)| {
            (*metric, *freq as f64 / total)
        }))
    }
}

pub trait Commandable {
    fn init() -> DynCommand where Self: Sized + 'static;
    fn exec(&self, args: &str, id: u64) -> String;
    fn usage<'a>(&self) -> &'a str;
    fn desc<'a>(&self) -> &'a str;

    fn help(&self) -> String {
        let mut help_message = String::new();
        help_message.push_str(self.usage());
        help_message.push('\n');
        help_message.push_str(self.desc());
        help_message
    }

    fn cmini_channel_only(&self) -> bool {
        false
    }

    fn mod_only(&self) -> bool {
        false
    }
}
