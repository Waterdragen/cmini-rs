use crate::util::consts::TABLE;
use crate::util::core::{Finger, FingerUsage, Key, Layout, LayoutConfig, Metric, Stat};
use fxhash::FxHashMap;

impl LayoutConfig {
    pub fn fingers_usage(&self, grams: &[([Key; 1], u64)]) -> FingerUsage {
        let mut fingers: FxHashMap<Finger, u64> = FxHashMap::default();

        for (gram, count) in grams.iter() {
            let gram = gram[0];
            if !self.keys.contains_key(&gram) {
                continue;
            }
            let finger = self.keys.get(&gram).unwrap().finger;
            match fingers.contains_key(&finger) {
                true => { *fingers.get_mut(&finger).unwrap() += count; },
                false => { fingers.insert(finger, *count); },
            };
        }
        let total = fingers.values().sum::<u64>() as f64;

        fingers.into_iter()
            .map(|(finger, freq)| {
                (finger, freq as f64 / total)
            })
            .collect()
    }
    pub fn trigram_stats(&self, grams: &[([Key; 3], u64)]) -> Stat {
            let mut counter = Metric::new_counter();
            let fingers = &self.keys;
            const SFR: &Metric = &Metric::Sfr;
            const UNKNOWN: &Metric = &Metric::Unknown;
            const SPACE: Key = ' ';

        grams.iter().for_each(|(gram, count)| {
            let gram0 = gram[0];
            let gram1 = gram[1];
            let gram2 = gram[2];
            if gram0 == SPACE || gram1 == SPACE || gram2 == SPACE {
                return;
            }
            if gram0 == gram1 || gram1 == gram2 || gram0 == gram2 {
                *counter.get_mut(SFR).unwrap_or_else(|| panic!("cannot get sfr")) += count;
                return;
            }
            let finger_hash = match get_finger_hash(fingers, gram0, gram1, gram2) {
                None => {
                    *counter.get_mut(UNKNOWN).unwrap_or_else(|| panic!("cannot get unknown")) += count;
                    return;
                }
                Some(finger_hash) => finger_hash,
            };

            let gram_type = TABLE[finger_hash];

            *counter
                .get_mut(&gram_type)
                .unwrap_or_else(|| panic!("cannot get gram type {:?}", gram_type)
                ) += count;
        });

        Metric::normalize_counter(&counter)
    }
}

#[inline]
pub(crate) fn get_finger_hash(layout: &Layout, gram0: Key, gram1: Key, gram2: Key) -> Option<usize> {
    let finger0 = layout.get(&gram0)?.finger;
    let finger1 = layout.get(&gram1)?.finger;
    let finger2 = layout.get(&gram2)?.finger;
    Some((finger0 << 8) | (finger1 << 4) | finger2.as_u8() as usize)
}
