use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use crate::util::consts::ADMINS;
use crate::util::{conv, Message};
use fxhash::{FxBuildHasher, FxHashMap};
use indexmap::IndexMap;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type Row = u8;
pub type Col = u8;
pub type Finger = u16;
pub type Key = char;
pub type Position = (Row, Col, Finger);

pub type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
pub type SyncFxMap<K, V> = Arc<RwLock<FxHashMap<K, Arc<V>>>>;
pub type SyncIndexMap<K, V> = Arc<RwLock<FxIndexMap<K, Arc<V>>>>;

pub type Layout = FxHashMap<Key, Position>;
pub type Stat = FxHashMap<Metric, f64>;
pub type FingerUsage = FxHashMap<Finger, f64>;
pub type CachedStats = FxHashMap<String, Arc<Stat>>;
pub type CachedStatConfig = Arc<RawCachedStatConfig>;

pub type RawCorpus<Gram> = [(Gram, u64)];
pub type Corpus<const N: usize> = RawCorpus<[Key; N]>;
pub type WordCorpus = RawCorpus<Vec<Key>>;
pub type RawServerCorpora<Gram> = SyncFxMap<String, RawCorpus<Gram>>;
pub type ServerCorpora<const N: usize> = SyncFxMap<String, Corpus<N>>;
pub type ServerWordCorpora = SyncFxMap<String, WordCorpus>;
pub type ServerCachedStats = SyncIndexMap<String, RawCachedStatConfig>;

// Trait: Commandable
// Struct: Command
// Instance Smart Pointer: DynCommand
pub type DynCommand = Box<dyn Commandable>;

#[repr(transparent)]
#[derive(Serialize)]
pub struct ServerLayouts(SyncIndexMap<String, LayoutConfig>);

impl Deref for ServerLayouts {
    type Target = SyncIndexMap<String, LayoutConfig>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ServerLayouts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_map(ServerLayoutsVisitor)
    }
}

struct ServerLayoutsVisitor;

impl<'de> Visitor<'de> for ServerLayoutsVisitor {
    type Value = ServerLayouts;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "struct ServerLayouts")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut map_inner = FxIndexMap::with_capacity_and_hasher(
            map.size_hint().unwrap_or(0),
            FxBuildHasher::default()
        );
        while let Some((key, ll)) = map.next_entry::<String, JsonLayoutConfig>()? {
            let name = key.clone();
            let layout_config = LayoutConfig {
                name: key,
                user: ll.user,
                board: ll.board.clone(),
                keys: conv::layout::unpack(&ll.keys),
                sum: conv::hash_keys(&ll.keys),
            };
            map_inner.insert(name, Arc::new(layout_config));
        }
        Ok(ServerLayouts(Arc::new(RwLock::new(map_inner))))
    }
}

#[derive(Debug, Deserialize)]
pub struct JsonLayoutConfig {
    pub user: u64,
    pub board: String,
    pub keys: String,
}

pub struct LayoutConfig {
    pub name: String,
    pub user: u64,
    pub board: String,
    pub keys: Layout,
    pub sum: u64,
}

impl LayoutConfig {
    pub fn new(name: String, user: u64, board: String, keys: Layout) -> Self {
        let packed = conv::layout::pack(&keys);
        let sum = conv::hash_keys(&packed);
        LayoutConfig {
            name,
            user,
            board,
            keys,
            sum,
        }
    }
}

impl Serialize for LayoutConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("user", &self.user)?;
        map.serialize_entry("board", &self.board)?;
        map.serialize_entry("keys", &conv::layout::pack(&self.keys))?;
        map.end()
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

    fn mods_only(&self) -> bool {
        false
    }

    fn try_exec(&self, msg: &Message) -> String {
        if !self.mods_only() || ADMINS.contains(&msg.id) {
            self.exec(msg)
        } else {
            "Unauthorized".to_owned()
        }
    }
}
