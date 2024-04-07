use std::collections::HashMap;
use lazy_static::lazy_static;
use tokio::time::Instant;
use crate::util::analyzer_util::{MetricName, ThreeFingerCombo};
use crate::util::core::{Corpus, Layout};
use crate::util::jsons::{get_map_str_str, get_table};

lazy_static!(
    static ref TABLE: HashMap<ThreeFingerCombo, String> = get_table("./table.json");
    static ref DEFAULT_COUNTER: HashMap<MetricName<'static>, f64> = HashMap::from_iter(
        ["alternate", "bad-redirect", "dsfb", "dsfb-alt",
            "dsfb-red", "oneh-in", "oneh-out", "redirect",
            "roll-in", "roll-out", "sfR", "sfT",
            "sfb", "sft", "unknown"]
        .into_iter()
        .map(|metric| (MetricName::new(metric), 0.0))
    );
);

pub fn fingers_usage(ll: &Layout, grams: &Corpus) -> HashMap<String, f64> {
    let mut fingers: HashMap<String, f64> = HashMap::new();

    for (gram, count) in grams.iter() {
        let gram = gram[0].to_string();
        if !ll.keys.contains_key(&gram) {
            continue;
        }
        let finger = &ll.keys.get(&gram).unwrap().finger;
        match fingers.contains_key(finger) {
            true => { *fingers.get_mut(finger).unwrap() += count; },
            false => { fingers.insert(finger.to_string(), count.clone()); },
        };
    }
    let total: f64 = fingers.values().sum();
    for (_, value) in fingers.iter_mut() {
        *value /= total;
    }
    fingers.insert(String::from("LH"),
                   fingers.iter().filter_map(|item| {
                       let (finger, count) = item;
                       if &finger[..1] == "L" {Some(count)}
                       else {None}
                   }).sum()
    );
    fingers.insert(String::from("RH"),
                   fingers.iter().filter_map(|item| {
                       let (finger, count) = item;
                       let c = &finger[..1];
                       if c == "R" || c == "T" {Some(count)}
                       else {None}
                   }).sum()
    );
    fingers
}


pub fn trigrams(ll: &Layout, grams: &Corpus) -> HashMap<String, f64> {
    let analyzer_start = Instant::now();
    let mut counts = DEFAULT_COUNTER.clone();
    let fingers: HashMap<char, &str> = ll.keys.iter().map(|(key, pos)| {
        (key.chars().next().unwrap(), pos.finger.as_str())
    }).collect();
    let sfr = &MetricName::new("sfR");
    let unknown = &MetricName::new("unknown");
    let space = ' ';

    let mut total_iters = 0;
    // let tolerance: u64 = grams.iter().map(|item| item.1).sum::<u64>() / 10_000_000;

    grams.iter().for_each(|(gram, count)| {
        // if count < &tolerance {
        //     return;
        // }
        let gram0 = gram[0];
        let gram1 = gram[1];
        let gram2 = gram[2];
        if gram0 == space || gram1 == space || gram2 == space {
            return;
        }
        if gram0 == gram1 || gram1 == gram2 || gram0 == gram2 {
            *counts.get_mut(sfr).unwrap() += count;
            return;
        }
        let mut finger_combo = String::with_capacity(6);
        for c in gram.iter() {
            if let Some(finger) = fingers.get(c) {
                finger_combo.push_str(finger);
            } else { break; }
        };
        if finger_combo.len() < 6 {
            *counts.get_mut(unknown).unwrap() += count;
            return;
        }
        let gram_type = TABLE.get(&ThreeFingerCombo::new(finger_combo));

        total_iters += 1;

        *counts.get_mut(&MetricName::new(gram_type.unwrap())).unwrap() += count;
    });

    let total: f64 = counts.values().sum();
    counts.values_mut().for_each(|value| {
        *value /= total;
    });
    println!("analyzer::trigrams(): {:?} gram length: {}", analyzer_start.elapsed(), grams.len());

    counts
        .into_iter()
        .map(|(k, v)| {
            (k.to_string(), v)
        })
        .collect()
}
