use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, BitOr, Deref, Shl};

use crate::prelude::*;
use crate::util::admins::ADMINS;
use crate::util::consts::{LH, RH};
use crate::util::{conv, Message};
use fxhash::{FxBuildHasher, FxHashMap};
use indexmap::IndexMap;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

pub type Row = u8;
pub type Col = u8;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, IntoStaticStr)]
#[repr(u8)]
pub enum Finger {
    LP = 0,
    LR = 1,
    LM = 2,
    LI = 3,
    LT = 4,
    RT = 5,
    RI = 6,
    RM = 7,
    RR = 8,
    RP = 9,
}

impl Finger {
    pub const MIN: u8 = Finger::LP.as_u8();
    pub const MAX: u8 = Finger::RP.as_u8();
    const MID: u8 = Finger::RT.as_u8();

    #[track_caller]
    pub fn from_str(s: &str) -> Self {
        match s {
            "LP" => Self::LP,
            "LR" => Self::LR,
            "LM" => Self::LM,
            "LI" => Self::LI,
            "LT" => Self::LT,
            "RT" => Self::RT,
            "RI" => Self::RI,
            "RM" => Self::RM,
            "RR" => Self::RR,
            "RP" => Self::RP,
            _ => panic!("Error in `Finger::from_str`: invalid finger {s}"),
        }
    }
    #[track_caller]
    pub fn from_u8(n: u8) -> Self {
        if n >= 10 {
            panic!("Error in `Finger::from_u8`: invalid finger index {n}");
        }
        // SAFETY: All enum variants are [0..=9]u8 (less than 10)
        // SAFETY: range has been checked above
        unsafe { std::mem::transmute::<u8, Finger>(n) }
    }
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
    pub fn mirror(self) -> Self {
        // Always succeeds: 9 - [0..=9] is always within range of 0..=9
        Self::from_u8(Self::MAX - self.as_u8())
    }
    pub fn is_left(self) -> bool {
        self.as_u8() < Self::MID
    }
    pub fn is_right(self) -> bool {
        self.as_u8() >= Self::MID
    }
    pub fn to_union(self) -> FingerUnion {
        FingerUnion(1 << self.as_u8())
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        (Self::MIN..=Self::MAX).map(|i| Self::from_u8(i))
    }
}

impl Shl<u8> for Finger {
    type Output = usize;

    fn shl(self, rhs: u8) -> Self::Output {
        usize::from(self.as_u8()) << rhs
    }
}

impl BitOr for Finger {
    type Output = FingerUnion;
    fn bitor(self, other: Self) -> Self::Output {
        self.to_union() | other
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
#[repr(transparent)]
pub struct FingerUnion(u16);

impl BitOr<Finger> for FingerUnion {
    type Output = Self;
    fn bitor(self, finger: Finger) -> Self::Output {
        Self(self.0 | 1u16 << finger.as_u8())
    }
}

impl FingerUnion {
    pub fn is_left(self) -> bool {
        self.0 & LH.0 != 0
    }
    pub fn is_right(self) -> bool {
        self.0 & RH.0 != 0
    }
    pub fn contains(self, finger: Finger) -> bool {
        self.0 & 1 << u16::from(finger.as_u8()) != 0
    }
    pub fn iter(self) -> impl Iterator<Item = Finger> {
        Finger::iter().filter(move |finger| self.contains(*finger))
    }
}

pub type Key = char;

#[derive(Copy, Clone)]
pub struct Position {
    pub row: Row,
    pub col: Col,
    pub finger: Finger,
}

impl Position {
    pub fn new(row: Row, col: Col, finger: Finger) -> Self {
        Position { row, col, finger }
    }
}

pub struct FingerMap<T>([T; 10]);

impl<T: Copy + Default> FromIterator<(Finger, T)> for FingerMap<T> {
    fn from_iter<I: IntoIterator<Item=(Finger, T)>>(iter: I) -> Self {
        let mut finger_map = FingerMap([T::default(); 10]);
        iter.into_iter()
            .for_each(|(finger, value)| {
                finger_map.0[usize::from(finger.as_u8())] = value;
            });
        finger_map
    }
}

impl<T> FingerMap<T> {
    pub fn get(&self, finger: Finger) -> &T {
        &self.0[usize::from(finger.as_u8())]
    }
    pub fn set(&mut self, finger: Finger, value: T) {
        self.0[usize::from(finger.as_u8())] = value;
    }
    pub fn get_mut(&mut self, finger: Finger) -> &mut T {
        &mut self.0[usize::from(finger.as_u8())]
    }
    pub fn sum(&self, finger_union: FingerUnion) -> T where T: for<'a> Add<&'a T, Output = T> + Default {
        finger_union.iter()
            .fold(T::default(),
            |acc, finger| {
                acc + self.get(finger)
            })
    }
    pub fn iter(&self) -> impl Iterator<Item = (Finger, &T)> {
        Finger::iter().zip(self.0.iter())
    }
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

pub type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;
pub type SyncFxIndexMap<K, V> = Arc<RwLock<FxHashMap<K, Arc<V>>>>;
pub type SyncIndexMap<K, V> = Arc<RwLock<FxIndexMap<K, Arc<V>>>>;

pub type Layout = FxHashMap<Key, Position>;
pub type Stat = FxHashMap<Metric, f64>;
pub type FingerUsage = FingerMap<f64>;
pub type CachedStats = FxHashMap<String, Arc<Stat>>;
pub type CachedStatConfig = Arc<RawCachedStatConfig>;

// pub type RawCorpus<Gram> = [(Gram, u64)];
pub type Corpus<const N: usize> = RawCorpus<[Key; N]>;
pub type WordCorpus = RawCorpus<Vec<Key>>;
pub type RawServerCorpora<Corpus> = Arc<RwLock<FxHashMap<String, Corpus>>>;
pub type ServerCorpora<const N: usize> = RawServerCorpora<Corpus<N>>;
pub type ServerWordCorpora = RawServerCorpora<WordCorpus>;
pub type ServerCachedStats = SyncIndexMap<String, RawCachedStatConfig>;

pub struct RawCorpus<Gram> {
    inner: Arc<[(Gram, u64)]>,
    pub sum: u64,
}

impl<Gram> RawCorpus<Gram> {
    pub fn arc_clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            sum: self.sum,
        }
    }
}

impl<Gram> Deref for RawCorpus<Gram> {
    type Target = [(Gram, u64)];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<Gram> FromIterator<(Gram, u64)> for RawCorpus<Gram> {
    fn from_iter<T: IntoIterator<Item=(Gram, u64)>>(iter: T) -> Self {
        let inner = iter.into_iter().collect::<Arc<[(Gram, u64)]>>();
        let sum = inner.iter().map(|(_, freq)| freq).sum::<u64>();
        Self {
            inner,
            sum,
        }
    }
}

// Trait: Commandable
// Struct: Command
// Instance Smart Pointer: DynCommand
pub type DynCommand = Box<dyn Commandable>;

#[derive(Debug, Deserialize)]
pub struct JsonLayoutConfig {
    pub user: u64,
    pub board: String,
    pub keys: String,
}

#[derive(Clone)]
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

impl BitOr for Metric {
    type Output = MetricUnion;

    fn bitor(self, rhs: Self) -> Self::Output {
        MetricUnion(1 << self.pack() | 1 << rhs.pack())
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct MetricUnion(u32);

impl BitOr<Metric> for MetricUnion {
    type Output = Self;

    fn bitor(self, metric: Metric) -> Self::Output {
        Self(self.0 | 1 << metric.pack())
    }
}

pub trait ContainsMetric: Copy {
    /// - If `self` is `Metric`:
    ///   - check if two `Metric`s are equal
    /// - If `self` is `MetricUnion`:
    ///   - check if it contains the `metric`
    fn contains(self, metric: Metric) -> bool;
}

impl ContainsMetric for Metric {
    fn contains(self, other: Metric) -> bool {
        self == other
    }
}

impl ContainsMetric for MetricUnion {
    fn contains(self, metric: Metric) -> bool {
        self.0 & 1 << metric.pack() != 0
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
        let mut help_message = "```\n".to_owned();
        help_message.push_str(self.usage());
        help_message.push('\n');
        help_message.push_str(self.desc());
        help_message.push_str("```");
        help_message
    }

    fn cmini_channel_only(&self) -> bool {
        false
    }

    fn public_channel_only(&self) -> bool {
        false
    }

    fn mods_only(&self) -> bool {
        false
    }

    fn try_exec(&self, msg: &Message) -> String {
        if self.mods_only() && !ADMINS.contains(msg.id) {
            return "Unauthorized".to_owned();
        }
        if self.public_channel_only() && msg.is_private() {
            return "Use this command in a public channel".to_owned();
        }
        self.exec(msg)
    }
}
