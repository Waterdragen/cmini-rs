use crate::util::consts::TABLE;
use crate::util::core::{ContainsMetric, Corpus, FingerCombo, FingerMap, FingerUsage, Key, Layout, LayoutConfig, Metric, MetricUnion, Stat};
use crate::util::metric_alias::SFS;

impl LayoutConfig {
    pub fn fingers_usage(&self, monograms: &Corpus<1>) -> FingerUsage {
        let mut usage = FingerMap::<u64>::default();

        for (gram, count) in monograms.iter() {
            let gram = gram[0];
            if !self.keys.contains_key(&gram) {
                continue;
            }
            let finger = self.keys.get(&gram).unwrap().finger;
            *usage.get_mut(finger) += count;
        }
        let total = usage.values().sum::<u64>() as f64;  // this total is zeroable
        // Try normalize counter or 0
        usage.iter()
            .map(|(finger, &freq)| {
                (finger, (freq as f64 / total).max(0.0))
            })
            .collect()
    }
    pub fn fingers_usage_of_metric(&self, trigrams: &Corpus<3>, metric_union: MetricUnion) -> FingerUsage {
        let mut usage = FingerMap::<f64>::default();
        iter_trigrams(trigrams, &self.keys)
            .filter_map(|(fingers_combo_cfg, freq)| {
                fingers_combo_cfg.map(|(finger_combo, metric)| (finger_combo, metric, freq))
            })
            .filter(|(_, metric, _)| metric_union.contains(*metric))
            .for_each(|(finger_combo, metric,  freq)| {
                // Only count the same-fingers for Sfb, Sfs, and Sfr
                let use_same_finger_or_hand_counting = if metric_union == SFS ||
                    matches!(metric,
                        Metric::Sfb | Metric::Sfr) {
                    let finger0 = finger_combo.inner[0];
                    let finger1 = finger_combo.inner[1];
                    let finger2 = finger_combo.inner[2];
                    Some([finger0 == finger1, finger1 == finger2])
                }
                // Only count the same-hands for rolls
                else if matches!(metric, Metric::InRoll | Metric::OutRoll) {
                    let hand0 = finger_combo.inner[0].is_right();
                    let hand1 = finger_combo.inner[1].is_right();
                    let hand2 = finger_combo.inner[2].is_right();
                    Some([hand0 == hand1, hand1 == hand2])
                } else {
                    None
                };
                let involved_fingers = match use_same_finger_or_hand_counting {
                    None | Some([true, true]) => [0usize, 1, 2].as_slice(),
                    Some([true, false]) => &[0, 1],
                    Some([false, true]) => &[1, 2],
                    Some([false, false]) => &[0, 2],
                };
                for &finger in involved_fingers {
                    *usage.get_mut(finger_combo.inner[finger]) += freq as f64 / involved_fingers.len() as f64;
                }
            });
        let total = trigrams.sum as f64;
        // Normalize counter
        usage.iter()
            .map(|(finger, &freq)| {
                (finger, freq / total)
            })
            .collect()
    }
    pub fn trigram_stats(&self, trigrams: &Corpus<3>) -> Stat {
        let mut counter = Metric::new_counter();
        for (finger_combo_cfg, freq) in iter_trigrams(trigrams, &self.keys) {
            let metric = match finger_combo_cfg {
                None => Metric::Unknown,
                Some((_, metric)) => metric,
            };
            *counter
                .get_mut(&metric)
                .unwrap_or_else(
                    || panic!("cannot get gram type {:?}", metric)
                ) += freq;
        }
        // Normalize counter
        let total = trigrams.sum as f64;
        counter.iter()
            .map(|(&metric, &freq)| (metric, freq as f64 / total))
            .collect()
    }
}

fn iter_trigrams<'a>(trigrams: &'a Corpus<3>, fingers: &'a Layout) -> impl Iterator<Item = (Option<(FingerCombo<3>, Metric)>, u64)> + 'a {
    const SPACE: Key = ' ';
    trigrams.iter().filter_map(|(gram, freq)| {
        let gram0 = gram[0];
        let gram1 = gram[1];
        let gram2 = gram[2];
        if gram0 == SPACE || gram1 == SPACE || gram2 == SPACE {
            return None;
        }
        let finger_combo = match FingerCombo::from_ngrams(fingers, gram) {
            None => return Some((None, *freq)),
            Some(finger_combo) => finger_combo,
        };
        if gram0 == gram1 || gram1 == gram2 || gram0 == gram2 {
            return Some((Some((finger_combo, Metric::Sfr)), *freq));
        }
        Some((Some((finger_combo, TABLE[finger_combo])), *freq))
    })
}
