use std::ops::{Add, BitOr, Index, IndexMut};
use fxhash::FxHashMap;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter, EnumString, IntoStaticStr};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumIter, EnumCount, EnumString, IntoStaticStr)]
#[repr(u8)]
pub enum Metric {
    #[strum(serialize = "sfb")]
    Sfb = 0,
    #[strum(serialize = "sft")]
    Sft,
    #[strum(serialize = "sfr")]
    Sfr,
    #[strum(serialize = "alt")]
    Alt,
    #[strum(serialize = "alt-sfs")]
    AltSfs,
    #[strum(serialize = "red")]
    Red,
    #[strum(serialize = "bad-red")]
    BadRed,
    #[strum(serialize = "red-sfs")]
    RedSfs,
    #[strum(serialize = "bad-red-sfs")]
    BadRedSfs,
    #[strum(serialize = "inoneh")]
    InOne,
    #[strum(serialize = "outoneh")]
    OutOne,
    #[strum(serialize = "inroll")]
    InRoll,
    #[strum(serialize = "outroll")]
    OutRoll,
    #[strum(serialize = "unknown")]
    Unknown,
}

impl Metric {
    #[inline]
    pub fn as_u8(self) -> u8 {
        self as u8
    }
    #[inline]
    pub fn as_usize(self) -> usize {
        usize::from(self as u8)
    }

    pub fn new_counter() -> FxHashMap<Metric, u64> {
        FxHashMap::from_iter(Metric::iter().map(|metric| {
            (metric, 0u64)
        }))
    }
}

impl BitOr for Metric {
    type Output = MetricUnion;

    fn bitor(self, rhs: Self) -> Self::Output {
        MetricUnion(1 << self.as_u8() | 1 << rhs.as_u8())
    }
}

#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct MetricUnion(u32);

impl MetricUnion {
    pub const fn from_raw(raw: u32) -> Self {
        MetricUnion(raw)
    }
}

impl From<Metric> for MetricUnion {
    fn from(metric: Metric) -> Self {
        Self(1 << metric.as_u8())
    }
}

impl BitOr<Metric> for MetricUnion {
    type Output = Self;

    fn bitor(self, metric: Metric) -> Self::Output {
        Self(self.0 | 1 << metric.as_u8())
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
        self.0 & 1 << metric.as_u8() != 0
    }
}

#[derive(Debug, Default)]
pub struct MetricMap<T>([T; Metric::COUNT]);

impl<T: Copy + Default> FromIterator<(Metric, T)> for MetricMap<T> {
    fn from_iter<I: IntoIterator<Item=(Metric, T)>>(iter: I) -> Self {
        let mut map = [T::default(); Metric::COUNT];
        iter.into_iter()
            .for_each(|(metric, t)| {
                map[metric.as_usize()] = t;
            });
        Self(map)
    }
}

impl<T> MetricMap<T> {
    pub fn sum(&self, metric_union: MetricUnion) -> T where T: for<'a> Add<&'a T, Output = T> + Default {
        Metric::iter().enumerate()
            .filter_map(|(idx, metric)| metric_union.contains(metric).then_some(&self.0[idx]))
            .fold(T::default(), |acc, b| acc + b)
    }
    pub fn iter(&self) -> impl Iterator<Item = (Metric, &T)> {
        Metric::iter()
            .zip(self.0.iter())
    }
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.iter_mut()
    }
}

impl<T> Index<Metric> for MetricMap<T> {
    type Output = T;

    fn index(&self, metric: Metric) -> &Self::Output {
        &self.0[metric.as_usize()]
    }
}

impl<T> IndexMut<Metric> for MetricMap<T> {
    fn index_mut(&mut self, metric: Metric) -> &mut Self::Output {
        &mut self.0[metric.as_usize()]
    }
}