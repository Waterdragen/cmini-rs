use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

use indexmap::IndexMap;
use fxhash::{FxBuildHasher, FxHashMap};
use serde::{Deserialize, Serialize, Serializer};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::util::{conv, Message};

pub type Row = u8;
pub type Col = u8;
pub type Finger = u16;
pub type Key = char;
pub type Position = (Row, Col, Finger);
pub type RawCorpus = Vec<(Vec<Key>, u64)>;
pub type Corpus = Arc<RawCorpus>;

pub type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
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
pub type DynCommand = Box<dyn Commandable>;

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
        JsonLayoutConfig::from_raw(self).serialize(serializer)
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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumIter)]
#[repr(u8)]
pub enum Metric {
    Sfb = 0,
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

    #[inline]
    pub fn pack(self) -> u8 {
        self as u8
    }

    #[inline]
    pub fn unpack(num: u8) -> Self {
        match num {
            0 => Self::Sfb,
            1 => Self::Sft,
            2 => Self::Sfr,
            3 => Self::Alt,
            4 => Self::AltSfs,
            5 => Self::Red,
            6 => Self::BadRed,
            7 => Self::RedSfs,
            8 => Self::BadRedSfs,
            9 => Self::InOne,
            10 => Self::OutOne,
            11 => Self::InRoll,
            12 => Self::OutRoll,
            13 => Self::Unknown,
            _ => panic!("Failed to convert to Metric. Unexpected value `{num}`")
        }
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

pub trait Commandable: Send + Sync {
    fn exec(&self, msg: &Message) -> String;
    fn usage<'a>(&self) -> &'a str;
    fn desc<'a>(&self) -> &'a str;

    fn init(self) -> Box<dyn Commandable> where Self: Sized + 'static {
        Box::new(self)
    }

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
