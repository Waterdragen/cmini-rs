use crate::consts::{COL_LIMIT, LH, RH, ROW_LIMIT, TABLE};
use crate::core::{ContainsMetric, Finger, FingerCombo, FingerUnion, FingerUsage, Key, LayoutConfig, Metric, Position, Stat};
use crate::util::{corpora, links, memory};

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

impl LayoutConfig {
    pub fn matrix_str(&self) -> String {
        let mut keyboard = [vec![' '; 12], vec![' '; 12], vec![' '; 12]];
        let mut thumb_row = Vec::<(Key, Finger)>::new();
        self.keys.iter().for_each(|(&key, &pos)| {
            let Position { row, col, finger } = pos;
            if row > ROW_LIMIT || col > COL_LIMIT { return; }  // This should never happen as checked by add command and pos::unpack, but still we ignore these keys
            if row == 3 {
                thumb_row.push((key, finger));
                return;
            }
            let (row, col) = (usize::from(row), usize::from(col));
            while keyboard[row].len() <= col {
                keyboard[row].reserve_exact(5);
                for _ in 0..5 {
                    keyboard[row].push(' ');
                }
            }
            keyboard[row][col] = key;
        });
        let indents = match self.board.as_str() {
            "angle" => [2, 2, 3, 3],
            "stagger" => [2, 3, 4, 5],
            _ => [2, 2, 2, 2],
        };
        let mut rows = indents.iter()
            .take(if thumb_row.is_empty() { 3 } else { 4 })
            .map(|&indent| {
                let mut s = String::with_capacity(40);
                for _ in 0..indent {
                    s.push(' ');
                }
                s
            })
            .collect::<Vec<_>>();

        keyboard.iter().enumerate().for_each(|(row, row_keys)| {
            let left_hand = &row_keys[..5];
            let right_hand = &row_keys[5..];

            left_hand.iter().for_each(|key| {
                rows[row].push(*key);
                rows[row].push(' ');
            });
            rows[row].push(' ');
            right_hand.iter().for_each(|key| {
                rows[row].push(*key);
                rows[row].push(' ');
            });
        });

        if thumb_row.is_empty() {
            return rows[..3].join("\n");
        }
        let lt_rt_split = thumb_row.iter()
            .position(|(_, finger)| *finger != Finger::LT)
            .unwrap_or(thumb_row.len());
        let (left_thumbs, right_thumbs) = thumb_row.split_at(lt_rt_split);  // Always succeeds: index is at most slice.len()
        let pad_count = 5usize.checked_sub(left_thumbs.len()).unwrap();  // Never underflows: guaranteed by add command, left hand takes at most 5 thumb Keys
        std::iter::repeat_n(' ', pad_count)
            .chain(left_thumbs.iter().map(|(key, _)| *key))
            .for_each(|key| {
                rows[3].push(key);
                rows[3].push(' ');
            });
        rows[3].push(' ');
        for (key, _) in right_thumbs {
            rows[3].push(*key);
            rows[3].push(' ');
        }

        rows.join("\n")
    }

    pub fn to_pretty(&self, id: u64) -> String {
        let author_reader = memory::AUTHORS.read();
        let author = author_reader.get_name(self.user).unwrap_or("Unknown");
        let monograms = corpora::ngrams::<1>(id);
        let trigrams = corpora::ngrams::<3>(id);

        let matrix_str = self.matrix_str();

        let stats = self.trigram_stats(&trigrams);
        let finger_usage = self.fingers_usage(&monograms);
        let stats_str = get_stats_str(&stats, &finger_usage);

        let likes = memory::get_like_count(&self.name);
        let like_str = if likes == 1 {"like"} else {"likes"};
        let external_link = links::get_link(&self.name);

        let ll_name = self.name.as_str();
        let corpus_name = corpora::get_user_corpus(id).to_uppercase();
        format!("```\n\
             {ll_name} ({author}) ({likes} {like_str})\n\
             {matrix_str}\n\
             \n\
             {corpus_name}:\n\
             {stats_str}\
             ```\n\
             {external_link}\n")
    }

    pub fn top_trigrams_of_metric<M: ContainsMetric>(&self, id: u64, metric: M, top_n: usize) -> Vec<([Key; 3], f64)> {
        let trigrams = corpora::ngrams::<3>(id);
        let fingers = &self.keys;
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
}

pub fn get_stats_str(stats: &Stat, finger_usage: &FingerUsage) -> String {
    use Metric as M;

    // get percentage of metric
    let get = |metric: M| -> f64 {
        stats.get(metric) * 100.0
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

    let sfb = get(M::Sfb) / 2.0;

    let bad_red_sfs = get(M::BadRedSfs);
    let bad_red = get(M::BadRed) + bad_red_sfs;
    let red = get(M::Red) + bad_red;

    let alt_sfs = get(M::AltSfs);
    let red_sfs = get(M::RedSfs) + bad_red_sfs;
    let sfs = alt_sfs + red_sfs;

    let lh = get_hand(*LH);
    let rh = get_hand(*RH);

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
