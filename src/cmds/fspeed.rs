use crate::util::corpora;
use crate::util::corpora::get_user_corpus_upper;
use crate::util::layout::header;
use crate::util::memory::LAYOUTS;
use crate::{Commandable, Message};
use cmini_core::{Col, FingerMap, Key, Layout, LayoutConfig, Position, Row};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        if msg.arg.is_empty() {
            return self.help();
        }
        let ll = &*LAYOUTS.find(msg.arg);
        let (unweighted_speeds, weighted_speeds) = FSpeed::new(ll).fingerspeed(msg.id);

        let mut used_fingers = FingerMap::<bool>::default();
        for pos in ll.keys.values() {
            used_fingers[pos.finger] = true;
        }

        let header = header(ll);
        let corpus_name = get_user_corpus_upper(msg.id);
        let mut output = format!("```\n\
                                         {header}\n\
                                         {corpus_name}:\n\
                                         Unweighted Speed\n");
        for ((finger, freq), _) in unweighted_speeds.iter()
            .zip(used_fingers.values())
            .filter(|(_, &used)| used) {
            let finger_str: &str = finger.into();
            output.push_str(&format!("    {finger_str}: {freq:.3}\n"))
        }
        output.push_str("\nWeighted Speed\n");
        for ((finger, freq), _) in weighted_speeds.iter()
            .zip(used_fingers.values())
            .filter(|(_, &used)| used) {
            let finger_str: &str = finger.into();
            output.push_str(&format!("    {finger_str}: {freq:.3}\n"))
        }
        output.push_str("```");

        output
    }

    fn usage<'a>(&self) -> &'a str {
        "fspeed <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "see the finger speed of a layout"
    }
}

struct FSpeed<'a> {
    stagger: bool,
    keys: &'a Layout,
}

impl<'a> FSpeed<'a> {
    const LATERAL: f64 = 1.4;
    const SFB: f64 = 1.0;
    const DSFB: f64 = 0.5;
    const KEY_TRAVEL: f64 = 0.01;
    const KPS: FingerMap<f64> = FingerMap::new([1.5, 3.6, 4.8, 5.5, 5.5, 5.5, 5.5, 4.8, 3.6, 1.5]);

    fn new(ll: &'a LayoutConfig) -> Self {
        let stagger = ll.board == "stagger";
        Self {
            stagger,
            keys: &ll.keys,
        }
    }

    fn fingerspeed(&self, id: u64) -> (FingerMap<f64>, FingerMap<f64>) {
        let bigrams = corpora::ngrams::<2>(id);
        let trigrams = corpora::ngrams::<3>(id);
        let bigram_total = bigrams.sum as f64;
        let trigram_total = trigrams.sum as f64;

        let mut sfb_speeds = FingerMap::<f64>::default();
        let mut dsfb_speeds = FingerMap::<f64>::default();

        let filter_fn = |first: Key, last: Key, freq: u64| {
            let pos0 = *self.keys.get(&first)?;
            let pos1 = *self.keys.get(&last)?;
            (pos0.finger == pos1.finger).then_some(())?;
            let dist = self.two_key_dist(pos0, pos1) + 2.0 * Self::KEY_TRAVEL;
            Some((pos1.finger, freq as f64 * dist))
        };

        bigrams.iter()
            .filter_map(|&(gram, freq)| {
                let [first, last] = gram;
                filter_fn(first, last, freq)
            })
            .for_each(|(finger, speed)| {
                sfb_speeds[finger] += speed;
            });
        trigrams.iter()
            .filter_map(|&(gram, freq)| {
                let [first, .., last] = gram;
                filter_fn(first, last, freq)
            })
            .for_each(|(finger, speed)| {
                dsfb_speeds[finger] += speed;
            });

        let mut speeds = FingerMap::<f64>::default();
        for (finger, freq) in speeds.iter_mut() {
            *freq = Self::SFB * sfb_speeds[finger] / bigram_total +
                Self::DSFB * dsfb_speeds[finger] / trigram_total;
        }
        let mut unweighted_speeds = speeds.clone();
        for speed in unweighted_speeds.values_mut() {
            *speed *= 800.0;
        }
        let mut weighted_speeds = speeds.clone();
        for (finger, speed) in weighted_speeds.iter_mut() {
            *speed /= Self::KPS[finger];
        }
        (unweighted_speeds, weighted_speeds)
    }

    fn two_key_dist(&self, p1: Position, p2: Position) -> f64 {
        let (r1, c1, r2, c2) = (p1.row, p1.col, p2.row, p2.col);
        let x = match self.stagger {
            true => staggered_x(r1, c1) - staggered_x(r2, c2),
            false => c1 as f64 - c2 as f64,
        };
        let y = r1 as f64 - r2 as f64;
        Self::LATERAL * x * x + y * y
    }
}

fn staggered_x(r: Row, c: Col) -> f64 {
    let c = c as f64;
    match r {
        0 => c - 0.25,
        2 => c + 0.5,
        _ => c,
    }
}
