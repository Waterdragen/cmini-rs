use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use crate::util::core::{Corpus, Layout, RawLayoutConfig, Metric, Stat, FingerUsage, Finger, Key};
use crate::util::jsons::{get_table};

static TABLE: Lazy<[Metric; 4096]> = Lazy::new(|| get_table("./table.json"));

pub fn fingers_usage(ll: &RawLayoutConfig, grams: &Corpus) -> FingerUsage {
    let mut fingers: FxHashMap<Finger, u64> = FxHashMap::default();

    for (gram, count) in grams.iter() {
        let gram = gram[0];
        if !ll.keys.contains_key(&gram) {
            continue;
        }
        let finger = &ll.keys.get(&gram).unwrap().2;
        match fingers.contains_key(finger) {
            true => { *fingers.get_mut(finger).unwrap() += count; },
            false => { fingers.insert(*finger, *count); },
        };
    }
    let total = fingers.values().sum::<u64>() as f64;

    let mut fingers: FingerUsage = fingers.into_iter()
        .map(|(finger, freq)| {
            (finger, freq as f64 / total)
        })
        .collect();

    let total = fingers.values().sum::<f64>();
    let lh_usage = fingers.iter()
        .filter_map(|(finger, count)| { if *finger < 5 {Some(count)} else {None} })
        .sum::<f64>();

    fingers.insert(10, lh_usage);
    fingers.insert(11, total - lh_usage);
    fingers
}


pub fn trigrams(ll: &RawLayoutConfig, grams: &Corpus) -> Stat {
    let mut counter = Metric::new_counter();
    let fingers = &ll.keys;
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
        let finger_hash = get_finger_hash(fingers, gram0, gram1, gram2);
        if finger_hash.is_none() {
            *counter.get_mut(UNKNOWN).unwrap_or_else(|| panic!("cannot get unknown")) += count;
            return;
        }
        let finger_hash = finger_hash.unwrap();

        let gram_type = &TABLE[usize::from(finger_hash)];

        *counter
            .get_mut(gram_type)
            .unwrap_or_else(|| panic!("cannot get gram type {:?}", gram_type)
            ) += count;
    });

    Metric::normalize_counter(&counter)
}

#[inline]
fn get_finger_hash(layout: &Layout, gram0: Key, gram1: Key, gram2: Key) -> Option<Finger> {
    let finger0 = layout.get(&gram0)?.2;
    let finger1 = layout.get(&gram1)?.2;
    let finger2 = layout.get(&gram2)?.2;
    Some((finger0 << 8) | (finger1 << 4) | finger2)
}
