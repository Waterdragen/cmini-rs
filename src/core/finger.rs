use std::ops::{Add, BitOr, Index, IndexMut, Shl};
use strum_macros::IntoStaticStr;
use crate::core::finger_alias::{LH, RH};
use crate::core::{Key, Layout};

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

    pub fn try_from_str(s: &str) -> Result<Self, ()> {
        Ok(match s {
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
            _ => return Err(()),
        })
    }
    #[track_caller]
    pub fn from_u8(n: u8) -> Self {
        if n >= 10 {
            panic!("Error in `Finger::from_u8`: invalid finger index {n}");
        }
        // SAFETY: All enum variants have memory layout of [0..=9]u8 (less than 10)
        // SAFETY: range has been checked above
        unsafe { std::mem::transmute::<u8, Finger>(n) }
    }
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
    pub fn as_digit_char(self) -> char {
        char::from_digit(self as u32, 10).unwrap()  // memory layout is < 10
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

impl BitOr<Self> for FingerUnion {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl From<Finger> for FingerUnion {
    fn from(finger: Finger) -> Self {
        Self(1u16 << finger.as_u8())
    }
}

impl FingerUnion {
    pub const fn from_raw(raw: u16) -> Self {
        FingerUnion(raw)
    }
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

#[derive(Debug, Default, Clone)]
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
    pub const fn new(map: [T; 10]) -> Self {
        Self(map)
    }
    pub fn sum(&self, finger_union: FingerUnion) -> T where T: for<'a> Add<&'a T, Output = T> + Default {
        finger_union.iter()
            .fold(T::default(),
                  |acc, finger| {
                      acc + &self[finger]
                  })
    }
    pub fn iter(&self) -> impl Iterator<Item = (Finger, &T)> {
        Finger::iter().zip(self.0.iter())
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Finger, &mut T)> {
        Finger::iter().zip(self.0.iter_mut())
    }
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.iter_mut()
    }
}

impl<T> Index<Finger> for FingerMap<T> {
    type Output = T;

    fn index(&self, finger: Finger) -> &Self::Output {
        &self.0[usize::from(finger.as_u8())]
    }
}

impl<T> IndexMut<Finger> for FingerMap<T> {
    fn index_mut(&mut self, finger: Finger) -> &mut Self::Output {
        &mut self.0[usize::from(finger.as_u8())]
    }
}

#[derive(Copy, Clone)]
pub struct FingerCombo<const N: usize> {
    pub inner: [Finger; N],
}

impl<const N: usize> From<[Finger; N]> for FingerCombo<N> {
    fn from(inner: [Finger; N]) -> Self {
        Self { inner }
    }
}

impl<const N: usize> FingerCombo<N> {
    pub fn from_ngrams(layout: &Layout, grams: &[Key; N]) -> Option<Self> {
        let mut inner = [Finger::LP; N];
        for i in 0..N {
            inner[i] = layout.get(&grams[i])?.finger;
        }
        Some(Self { inner })
    }
}

impl<const N: usize> FingerCombo<N> {
    pub fn index(self) -> usize {
        self.inner.iter()
            .fold(0usize, |acc, &finger| {
                10 * acc + usize::from(finger.as_u8())
            })
    }
}

pub type FingerUsage = FingerMap<f64>;
