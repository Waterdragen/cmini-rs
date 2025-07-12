use crate::util::{corpora, links, memory};
use cmini_core::consts::TABLE;
use cmini_core::finger_alias::{LH, RH};
use cmini_core::{ContainsMetric, FingerCombo, FingerUnion, FingerUsage, Key, LayoutConfig, Metric, Stat};

fn is_char_allowed_in_name(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' |
                '_' | '\'' | '-' | '(' | ')' | ':' | '~')
}

pub fn check_name(name: &str) -> Result<(), String> {
    if name.starts_with('_') {
        return Err("Error: names cannot start with an underscore".to_owned());
    }
    if name.len() < 3 {
        return Err("Error: names must be at least 3 characters long".to_owned());
    }
    for c in name.chars() {
        if !is_char_allowed_in_name(c) {
            return Err(format!("names cannot contain `{c}`"));
        }
    }
    Ok(())
}

pub fn header(layout: &LayoutConfig) -> String {
    let ll_name = &layout.name;
    let author_reader = memory::AUTHORS.read();
    let author = author_reader.get_name(layout.user).unwrap_or("Unknown");
    let likes = memory::get_like_count(&layout.name);
    let like_str = if likes == 1 {"like"} else {"likes"};
    format!("{ll_name} ({author}) ({likes} {like_str})")
}

pub fn to_pretty(layout: &LayoutConfig, id: u64) -> String {
    let header = header(layout);
    let matrix_str = layout.matrix_str();
    let corpus_name = corpora::get_user_corpus(id).to_uppercase();

    let monograms = corpora::ngrams::<1>(id);
    let trigrams = corpora::ngrams::<3>(id);

    let stats = layout.trigram_stats(&trigrams);
    let finger_usage = layout.fingers_usage(&monograms);
    let stats_str = get_stats_str(&stats, &finger_usage);
    let external_link = links::get_link(&layout.name);
    format!("```\n\
             {header}\n\
             {matrix_str}\
             \n\
             {corpus_name}:\n\
             {stats_str}\
             ```\n\
             {external_link}\n")
}

pub fn top_trigrams_of_metric<M: ContainsMetric>(layout: &LayoutConfig, id: u64, metric: M, top_n: usize) -> Vec<([Key; 3], f64)> {
    let trigrams = corpora::ngrams::<3>(id);
    let fingers = &layout.keys;
    let sum = trigrams.sum as f64;

    trigrams
        .iter()
        .filter_map(|(gram, freq)| {
            let gram_metric = if gram[0] == gram[1] || gram[1] == gram[2] || gram[0] == gram[2] {
                Metric::Sfr
            } else {
                match FingerCombo::from_ngrams(fingers, gram) {
                    None => Metric::Unknown,
                    Some(finger_combo) => TABLE[finger_combo],
                }
            };
            match metric.contains(gram_metric) {
                true => Some((*gram, *freq as f64 / sum)),
                false => None,
            }
        })
        // We only need the first n elements,
        // because all corpora are sorted in descending order
        .take(top_n)
        .collect()  // Already sorted by freq
}

pub fn get_stats_str(stats: &Stat, finger_usage: &FingerUsage) -> String {
    use Metric as M;

    // get percentage of metric
    let get = |metric: M| -> f64 {
        stats[metric] * 100.0
    };
    let get_hand = |hand: FingerUnion| -> f64 {
        finger_usage.sum(hand) * 100.0
    };

    let alt = get(M::Alt);

    let inroll = get(M::InRoll);
    let outroll = get(M::OutRoll);
    let inone = get(M::InOne);
    let outone = get(M::OutOne);

    let roll = inroll + outroll;
    let one = inone + outone;
    let inrolltal = inroll + inone;
    let outrolltal = outroll + outone;
    let rolltal = roll + one;

    let sfb = get(M::Sfb) / 2.0 + get(M::Sft);

    let bad_red_sfs = get(M::BadRedSfs);
    let bad_red = get(M::BadRed) + bad_red_sfs;
    let red = get(M::Red) + bad_red;

    let alt_sfs = get(M::AltSfs);
    let red_sfs = get(M::RedSfs) + bad_red_sfs;
    let sfs = alt_sfs + red_sfs;

    let lh = get_hand(LH);
    let rh = get_hand(RH);

    format!(
        "  Alt: {alt:>5.2}%
  Rol: {roll:>5.2}%   (In/Out: {inroll:>5.2}% | {outroll:>5.2}%)
  One: {one:>5.2}%   (In/Out: {inone:>5.2}% | {outone:>5.2}%)
  Rtl: {rolltal:>5.2}%   (In/Out: {inrolltal:>5.2}% | {outrolltal:>5.2}%)
  Red: {red:>5.2}%   (Bad:    {bad_red:>5.2}%)

  SFB: {sfb:>5.2}%
  SFS: {sfs:>5.2}%   (Red/Alt: {red_sfs:>5.2}% | {alt_sfs:>5.2}%)

  LH/RH: {lh:>5.2}% | {rh:>5.2}%\n")
}
