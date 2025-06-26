use std::ops::Deref;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::IntoEnumIterator;
use crate::core::{conv, FxIndexMap, Metric, MetricMap, SyncIndexMap};

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
    pub sum: u64,
    pub stats: FxIndexMap<String, CachedStat>,
}