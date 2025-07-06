use std::ops::Index;
use crate::core::{Finger, FingerCombo, Metric};

#[derive(Debug)]
pub struct Table([Metric; 1000]);

impl Table {
    pub fn from_inner(inner: [Metric; 1000]) -> Self {
        Self(inner)
    }
}

impl Index<FingerCombo<3>> for Table {
    type Output = Metric;

    fn index(&self, finger_combo: FingerCombo<3>) -> &Self::Output {
        &self.0[finger_combo.index()]
    }
}

impl Table {
    pub fn new() -> Self {
        let mut table = Table([Metric::Unknown; 1000]);
        const BAD_RED_MAP: [u8; 10] = [1, 1, 1, 0, 0, 0, 0, 1, 1, 1];

        let mut assign = |combo: [Finger; 3], metric: Metric| {
            let index = FingerCombo::from(combo).index();
            table.0[index] = metric;
        };

        let yield_finger_combo =
            Finger::iter().flat_map(|f0|
                Finger::iter().flat_map(move |f1|
                    Finger::iter().map(move |f2| [f0, f1, f2])));

        for combo in yield_finger_combo {
            let [finger0, finger1, finger2] = combo;
            let (hand0, hand1, hand2) = (finger0.is_right(), finger1.is_right(), finger2.is_right());
            if hand0 != hand1 && hand1 != hand2 {
                match finger0 != finger2 {
                    true => assign(combo, Metric::Alt),
                    false => assign(combo, Metric::AltSfs),
                }
                continue;
            }
            let sf_count = (finger0 == finger1) as u8 + (finger1 == finger2) as u8;
            match sf_count {
                1 => { assign(combo, Metric::Sfb); continue },
                2 => { assign(combo, Metric::Sft); continue },
                _ => {},
            }
            if hand0 == hand1 && hand1 == hand2 {
                let roll_to_left = finger0 > finger1 && finger1 > finger2;
                if roll_to_left || finger0 < finger1 && finger1 < finger2 {
                    match roll_to_left == hand0 {
                        true => assign(combo, Metric::InOne),
                        false => assign(combo, Metric::OutOne),
                    }
                } else {
                    let is_sfs = finger0 == finger2;
                    let is_bad = (BAD_RED_MAP[finger0.as_usize()] + BAD_RED_MAP[finger1.as_usize()] + BAD_RED_MAP[finger2.as_usize()]) == 3;
                    match (is_sfs, is_bad) {
                        (false, false) => assign(combo, Metric::Red),
                        (false, true) => assign(combo, Metric::BadRed),
                        (true, false) => assign(combo, Metric::RedSfs),
                        (true, true) => assign(combo, Metric::BadRedSfs),
                    }
                }
                continue;
            }
            let roll_to_left = if hand0 == hand1 { finger0 > finger1 } else { finger1 > finger2 };
            match roll_to_left == hand1 {
                true => assign(combo, Metric::InRoll),
                false => assign(combo, Metric::OutRoll)
            }
        }
        table
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}