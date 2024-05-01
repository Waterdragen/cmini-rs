use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::{BuildHasherDefault, Hash, Hasher};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

use indexmap::IndexMap;
use fxhash::{FxBuildHasher, FxHashMap};
use nohash_hasher::NoHashHasher;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
pub type SyncIndexMap<K, V> = Arc<RwLock<IndexMap<K, Arc<V>>>>;

pub type Layout = FxHashMap<char, Position>;
pub type LayoutConfig = Arc<RawLayoutConfig>;
pub type CachedStat = FxHashMap<String, FxHashMap<Metric, f64>>;

pub type ServerCorpora = SyncFxMap<String, RawCorpus>;
pub type ServerLayouts = SyncIndexMap<String, RawLayoutConfig>;

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
            keys: pack_layout(&layout_config.keys),
        }
    }
}

pub struct RawLayoutConfig {
    pub name: String,
    pub user: u64,
    pub board: String,
    pub keys: Layout,
}

impl RawLayoutConfig {
    pub fn from_json(name: &str, json_layout: JsonLayoutConfig) -> Self {
        RawLayoutConfig {
            name: name.to_string(),
            user: json_layout.user,
            board: json_layout.board.clone(),
            keys: unpack_layout(&json_layout.keys),
        }
    }
}

impl Serialize for RawLayoutConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        JsonLayoutConfig::from_raw(&self).serialize(serializer)
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


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumIter)]
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
    pub fn from_string(s: &str) -> Self {
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

    pub const fn to_str(&self) -> &str {
        match self {
            Metric::Sfb => "sfb",
            Metric::Sft => "sft",
            Metric::Sfr => "sfr",
            Metric::Alt => "alt",
            Metric::AltSfs => "alt-sfs",
            Metric::Red => "red",
            Metric::BadRed => "bad-red",
            Metric::RedSfs => "red-sfs",
            Metric::BadRedSfs => "bad-red-sfs",
            Metric::InOne => "inoneh",
            Metric::OutOne => "outoneh",
            Metric::InRoll => "inroll",
            Metric::OutRoll => "outroll",
            Metric::Unknown => "unknown",
        }
    }

    pub fn new_counter() -> FxHashMap<Metric, u64> {
        FxHashMap::from_iter(Metric::iter().map(|metric| {
            (metric, 0u64)
        }))
    }

    pub fn normalize_counter(counter: &FxHashMap<Metric, u64>) -> FxHashMap<Metric, f64> {
        let total = counter.values().sum::<u64>() as f64;
        assert_ne!(total, 0.0);
        FxHashMap::from_iter(counter.iter().map(|(metric, freq)| {
            (*metric, *freq as f64 / total)
        }))
    }
}

impl Serialize for Metric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.to_str())
    }
}

pub trait Commandable {
    fn init() -> DynCommand where Self: Sized + 'static;
    fn exec(&self, args: &str) -> String;
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

fn pack_pos((row, col, finger): &Position) -> String {
    let mut packed = (u16::from(*row) & 0xf) << 8;
    packed |= (u16::from(*col) & 0xf) << 4;
    packed |= finger & 0xf;
    format!("{:03x}", packed)
}

fn unpack_pos(packed_str: &str) -> Position {
    let packed = u16::from_str_radix(packed_str, 16).unwrap();
    let row = (packed >> 8 & 0xf) as u8;
    let col = (packed >> 4 & 0xf) as u8;
    let finger = packed & 0xf;
    (row, col, finger)
}

fn pack_layout(layout: &Layout) -> String {
    let mut layout_packed_ordered: Vec<(String, u32)> = layout.iter().map(|(key, pos)| {
        let mut packed_keypos = String::with_capacity(4);
        packed_keypos.push(*key);
        let packed_pos = pack_pos(pos);
        packed_keypos.push_str(&packed_pos);
        let order = ((pos.0 as u32) << 8) + (pos.1 as u32);
        (packed_keypos, order)
    }).collect();
    layout_packed_ordered.sort_by(|item0, item1| {
        item0.1.cmp(&item1.1)
    });
    let layout_packed: String = layout_packed_ordered.into_iter().map(|(keypos, _)| {
        keypos
    }).collect();

    layout_packed
}

fn unpack_layout(layout_packed: &str) -> Layout {
    let mut layout = Layout::default();
    let unpacked_chars: Vec<char> = layout_packed.chars().collect();

    for start in (0..unpacked_chars.len()).step_by(4) {
        let key = unpacked_chars[start];
        let mut chunk = String::with_capacity(3);
        (start + 1 .. start + 4).for_each(|index| {
            chunk.push(unpacked_chars[index])
        });
        let pos = unpack_pos(&chunk);
        layout.insert(key, pos);
    }
    layout
}
