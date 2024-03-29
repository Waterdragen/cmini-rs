use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::util::core::Layout;
use crate::util::jsons::get_map_str_str;

lazy_static!(
    static ref TABLE: HashMap<String, String> = get_map_str_str("./table.json");
    static ref DEFAULT_COUNTER: HashMap<String, f64> = HashMap::from_iter(
        vec!["alternate", "bad-redirect", "dsfb", "dsfb-alt",
            "dsfb-red", "oneh-in", "oneh-out", "redirect",
            "roll-in", "roll-out", "sfR", "sfT",
            "sfb", "sft", "unknown"]
        .into_iter()
        .map(|metric| (metric.to_string(), 0.0))
    );
);

pub fn fingers_usage(ll: &Layout, grams: &HashMap<String, u64>) -> HashMap<String, f64> {
    let mut fingers: HashMap<String, f64> = HashMap::new();

    for (gram, count) in grams {
        if !ll.keys.contains_key(gram) {
            continue;
        }
        let finger = &ll.keys.get(gram).unwrap().finger;
        match fingers.contains_key(finger) {
            true => { *fingers.get_mut(finger).unwrap() += count.clone() as f64; },
            false => { fingers.insert(finger.to_string(), count.clone() as f64); },
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

pub fn trigrams(ll: &Layout, grams: &HashMap<String, u64>) -> HashMap<String, f64> {
    let mut counts = DEFAULT_COUNTER.clone();
    let fingers: HashMap<char, String> = ll.keys.iter().map(|item| {
        (item.0.chars().next().unwrap(), item.1.finger.to_string())
    }).collect();

    for (gram, count) in grams {
        if gram.contains(" ") {
            continue;
        }
        let mut gram_iter = gram.chars();
        let gram0 = gram_iter.next().unwrap();
        let gram1 = gram_iter.next().unwrap();
        let gram2 = gram_iter.next().unwrap();
        if gram0 == gram1 || gram1 == gram2 || gram0 == gram2 {
            *counts.get_mut("sfR").unwrap() += count.clone() as f64;
            continue;
        }

        let finger_combo = gram.chars().filter_map(|c| {
            match fingers.contains_key(&c) {
                true => Some(fingers.get(&c).unwrap().to_string()),
                false => None,
            }
        }).collect::<Vec<String>>().join("-");
        let finger_combo = finger_combo.replace("TB", "RT");
        let gram_type = TABLE.get(&finger_combo).unwrap_or(&String::from("unknown")).to_string();

        *counts.get_mut(&gram_type).unwrap() += count.clone() as f64;
    }
    let total: f64 = counts.values().sum();
    for (_, value) in counts.iter_mut() {
        *value /= total;
    }

    counts
}
