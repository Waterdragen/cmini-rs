use std::ops::{Add, BitOr};
use fxhash::FxHashMap;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, EnumIter, EnumCount)]
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
    pub fn get(&self, metric: Metric) -> &T {
        &self.0[metric.as_usize()]
    }
    pub fn get_mut(&mut self, metric: Metric) -> &mut T {
        &mut self.0[metric.as_usize()]
    }
    pub fn set(&mut self, metric: Metric, value: T) {
        self.0[metric.as_usize()] = value;
    }
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
    pub const fn len(&self) -> usize {
        Metric::COUNT
    }
}