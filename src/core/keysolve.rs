use std::ops::{Index, IndexMut};
use once_cell::sync::Lazy;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};
use crate::core::{Finger, FingerCombo, Position};

#[derive(Copy, Clone, EnumCount, EnumIter)]
pub enum KSBigramMetric {
    SameFinger = 0,
    Lateral,
    HScissor,
    FScissor,
    LateralHScissor,
    LateralFScissor,
}

impl KSBigramMetric {
    pub fn from_positions([p0, p1]: [Position; 2]) -> Option<KSBigramMetric> {
        if p0.finger == p1.finger {
            return ((p0.row, p0.col) != (p1.row, p1.col)).then_some(KSBigramMetric::SameFinger);
        }
        if p0.finger.is_right() != p0.finger.is_right() {
            return None;
        }
        let dx = p0.col.abs_diff(p1.col);
        let dy = p0.row.abs_diff(p1.row);
        let df = p0.finger.as_u8().abs_diff(p1.finger.as_u8());

        let is_lateral = df == 1 && dx == 2;
        let scissor = (dy > 0 && matches!(
            std::cmp::max_by_key(p0, p1, |pos| pos.row).finger,
            Finger::LR | Finger::LM | Finger::RM | Finger::RR
        )).then_some(dy);

        match (is_lateral, scissor) {
            (false, None) => None,
            (true, None) => Some(KSBigramMetric::Lateral),
            (false, Some(1)) => Some(KSBigramMetric::HScissor),
            (true, Some(1)) => Some(KSBigramMetric::LateralHScissor),
            (false, Some(_)) => Some(KSBigramMetric::FScissor),
            (true, Some(_)) => Some(KSBigramMetric::LateralFScissor),
        }
    }
}

#[derive(Copy, Clone, EnumCount, EnumIter)]
pub enum KSTrigramMetric {
    Alt,
    InRoll,
    OutRoll,
    Onehand,
    Redirect,
}

impl KSTrigramMetric {
    pub fn from_finger_combo(combo: [Finger; 3]) -> Option<KSTrigramMetric> {
        KS_TRIGRAM_TABLE[FingerCombo::from(combo).index()]
    }
}

static KS_TRIGRAM_TABLE: Lazy<[Option<KSTrigramMetric>; 1000]> = Lazy::new(|| {
    let mut table = [None; 1000];

    let mut assign = |combo: [Finger; 3], metric: KSTrigramMetric| {
        let index = FingerCombo::from(combo).index();
        table[index] = Some(metric);
    };

    let yield_finger_combo =
        Finger::iter().flat_map(|f0|
            Finger::iter().flat_map(move |f1|
                Finger::iter().map(move |f2| [f0, f1, f2])));

    for combo in yield_finger_combo {
        let [finger0, finger1, finger2] = combo;
        let (hand0, hand1, hand2) = (finger0.is_right(), finger1.is_right(), finger2.is_right());
        if hand0 != hand1 && hand1 != hand2 {
            assign(combo, KSTrigramMetric::Alt);
            continue;
        }
        if hand0 == hand1 && hand1 == hand2 {
            if finger0 > finger1 && finger1 > finger2 ||
                finger0 < finger1 && finger1 < finger2 {
                assign(combo, KSTrigramMetric::Onehand);
            } else {
                assign(combo, KSTrigramMetric::Redirect);
            }
            continue;
        }
        if finger0 == finger1 || finger1 == finger2 {
            continue;
        }
        let roll_to_left = if hand0 == hand1 { finger0 > finger1 } else { finger1 > finger2 };
        match roll_to_left == hand1 {
            true => assign(combo, KSTrigramMetric::InRoll),
            false => assign(combo, KSTrigramMetric::OutRoll)
        }
    }
    table
});

#[derive(Default)]
pub struct KSBigramMetricMap<T: Default>([T; KSBigramMetric::COUNT]);

impl<T: Default> KSBigramMetricMap<T> {
    pub fn iter(&self) -> impl Iterator<Item = (KSBigramMetric, &T)> {
        KSBigramMetric::iter().zip(self.0.iter())
    }
}

impl<T: Default> Index<KSBigramMetric> for KSBigramMetricMap<T> {
    type Output = T;

    fn index(&self, metric: KSBigramMetric) -> &Self::Output {
        &self.0[metric as usize]
    }
}

impl<T: Default> IndexMut<KSBigramMetric> for KSBigramMetricMap<T> {
    fn index_mut(&mut self, metric: KSBigramMetric) -> &mut Self::Output {
        &mut self.0[metric as usize]
    }
}

impl<T: Default> FromIterator<(KSBigramMetric, T)> for KSBigramMetricMap<T> {
    fn from_iter<I: IntoIterator<Item=(KSBigramMetric, T)>>(iter: I) -> Self {
        let mut map = Self::default();
        iter.into_iter().for_each(|(metric, value)| {
            map[metric] = value;
        });
        map
    }
}

#[derive(Default)]
pub struct KSTrigramMetricMap<T: Default>([T; KSTrigramMetric::COUNT]);

impl<T: Default> KSTrigramMetricMap<T> {
    pub fn iter(&self) -> impl Iterator<Item = (KSTrigramMetric, &T)> {
        KSTrigramMetric::iter().zip(self.0.iter())
    }
}

impl<T: Default> Index<KSTrigramMetric> for KSTrigramMetricMap<T> {
    type Output = T;

    fn index(&self, metric: KSTrigramMetric) -> &Self::Output {
        &self.0[metric as usize]
    }
}

impl<T: Default> IndexMut<KSTrigramMetric> for KSTrigramMetricMap<T> {
    fn index_mut(&mut self, metric: KSTrigramMetric) -> &mut Self::Output {
        &mut self.0[metric as usize]
    }
}

impl<T: Default> FromIterator<(KSTrigramMetric, T)> for KSTrigramMetricMap<T> {
    fn from_iter<I: IntoIterator<Item=(KSTrigramMetric, T)>>(iter: I) -> Self {
        let mut map = Self::default();
        iter.into_iter().for_each(|(metric, value)| {
            map[metric] = value;
        });
        map
    }
}

#[derive(Default)]
pub struct KSStat<T: Default> {
    pub bigrams: KSBigramMetricMap<T>,
    pub skipgrams: KSBigramMetricMap<T>,
    pub trigrams: KSTrigramMetricMap<T>,
}