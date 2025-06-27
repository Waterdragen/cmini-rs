use crate::core::{conv, Finger, FingerUnion, FxIndexMap, Metric, MetricMap, SyncIndexMap};
use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;
use strum::IntoEnumIterator;

pub type Stat = MetricMap<f64>;
pub type ServerCachedStats = SyncIndexMap<String, CachedStatConfig>;

#[derive(Debug)]
pub struct CachedStat(pub Stat);

impl Deref for CachedStat {
    type Target = MetricMap<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for CachedStat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut output = "".to_owned();
        for freq in self.0.values() {
            for c in conv::freq::pack(*freq) {
                output.push(c);
            }
        }
        serializer.serialize_str(&output)
    }
}

impl<'de> Deserialize<'de> for CachedStat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let mut stats = Stat::default();
        let s = String::deserialize(deserializer)?;
        let chars = s.chars().collect::<Vec<_>>();
        Metric::iter().zip(chars.windows(3).step_by(3))
            .for_each(|(metric, packed)| {
                let freq = conv::freq::unpack(packed);
                stats.set(metric, freq);
            });
        Ok(CachedStat(stats))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedStatConfig {
    pub keys: String,
    pub user: u64,
    pub sum: u64,
    pub stats: FxIndexMap<String, CachedStat>,
}

impl CachedStatConfig {
    pub fn iter_packed_key<'a>(&'a self) -> impl Iterator<Item = [char; 4]> + 'a {
        self.keys.chars()
            .batching(|it|
                Some([it.next()?, it.next()?, it.next()?, it.next()?])
            )
    }
    pub fn is_total_layout(&self) -> bool {
        let mut a_to_z_map = [false; 26];  // should involve all a-z keys
        let mut row_map = [[0u8; 10]; 3];  // every row should involve >=6 different fingers

        for packed_key in self.iter_packed_key() {
            let [key, row, _col, finger] = packed_key;
            if matches!(key, 'a'..='z') {
                a_to_z_map[key as usize - 'a' as usize] = true;
            }
            if let Some(row) = row.to_digit(3) {
                if let Some(finger) = finger.to_digit(10) {
                    row_map[row as usize][finger as usize] = 1;
                }
            }
        }
        a_to_z_map.iter().all(|&b| b) &&
            row_map.iter().all(|rows| rows.iter().sum::<u8>() >= 6)
    }
    pub fn contains_vowel_in_one_hand(&self, vowels: &str) -> bool {
        let mut hand = [false, false];
        for packed_key in self.iter_packed_key() {
            let [key, _row, _col, finger] = packed_key;
            if vowels.contains(key) {
                hand[
                    Finger::from_u8(
                        finger.to_digit(10).unwrap() as u8
                    ).is_left() as usize
                    ] = true
            }
        }
        hand[0] ^ hand[1]
    }
    pub fn contains_full_punc(&self) -> bool {
        let mut flags = [false, false, false];
        for key in self.keys.chars() {
            match key {
                '.' => { flags[0] = true; },
                ',' => { flags[1] = true; },
                '\'' => { flags[2] = true; },
                _ => {}
            }
        }
        flags.iter().all(|b| *b)
    }
    pub fn contains_thumb_keys(&self) -> bool {
        self.iter_packed_key()
            .any(|[_key, _row, _col, finger]| {
                finger == '4' || finger == '5'
            })
    }
    pub fn is_sfb(&self, sfb: &str, fingers: Option<FingerUnion>) -> bool {
        let count = sfb.chars().count();
        let mut matched = 0;
        self.iter_packed_key()
            .filter_map(|[key, _row, _col, finger]| {
                if let Some(fingers) = fingers {
                    let finger = Finger::from_u8(finger.to_digit(10)? as u8);
                    fingers.contains(finger).then_some(())?;
                }
                sfb.contains(key).then_some(())?;
                matched += 1;
                Some(finger)
            })
            .dedup()
            .count() == 1 && matched == count
    }
    pub fn get_homerow(&self) -> String {
        let mut chars = self.iter_packed_key()
            .filter_map(|[key, row, col, _finger]| {
                (row == '1').then_some((key, col))
            })
            .collect_vec();
        chars.sort_unstable_by_key(|(_, col)| *col);
        chars.into_iter()
            .map(|(key, _)| key)
            .collect()
    }
}