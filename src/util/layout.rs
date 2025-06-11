use crate::util::{analyzer, authors, corpora, links, memory};
use crate::util::core::{FingerUsage, LayoutConfig, Metric, Stat};

fn is_char_allowed_in_name(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' |
                '_' | '\'' | '-' | '(' | ')' | ':' | '~')
}

pub fn check_name(name: &str) -> Result<(), String> {
    let first_char = name.chars().next();
    if let Some(first_char) = first_char {
        if first_char == '_' {
            return Err("Error: names cannot start with an underscore".to_owned());
        }
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

pub fn get_matrix_str(ll: &LayoutConfig) -> String {
    let mut keyboard: [[char; 16]; 4]= [[' '; 16]; 4];
    ll.keys.iter().for_each(|(key, pos)| {
        let (row, col, _) = pos;
        if *row > 4 || *col > 16 { return; }
        keyboard[usize::from(*row)][usize::from(*col)] = *key;
    });

    // maximum length possible = 2(initial) + 2 + 5 * 2 + 2 + 11 * 2
    let mut rows: Vec<String> = std::iter::repeat_with(
        || String::with_capacity(38))
        .take(4)
        .collect();
    rows.iter_mut().for_each(|row| { row.push_str("  ") });

    match ll.board.as_str() {
        "angle" => { rows[2].push(' '); }
        "stagger" => { rows[1].push(' '); rows[2].push_str("  "); }
        _ => ()
    }
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

    match rows[3].chars().all(|c| c == ' ') {
        true => rows[..3].join("\n"),
        false => rows.join("\n")
    }
}

pub fn get_stats_str(stats: &Stat, finger_usage: &FingerUsage) -> String {
    use Metric as M;
    const LH: u16 = 10;
    const RH: u16 = 11;

    // get percentage of metric
    let get = |metric: M| -> f64 {
        stats.get(&metric).unwrap() * 100.0
    };
    let get_hand = |hand: u16| -> f64 {
        finger_usage.get(&hand).unwrap() * 100.0
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

    let lh = get_hand(LH);
    let rh = get_hand(RH);

    format!(
        "\
  Alt: {alt:>5.2}%\n\
  Rol: {roll:>5.2}%   (In/Out: {inroll:>5.2}% | {outroll:>5.2}%)\n\
  One: {one:>5.2}%   (In/Out: {inone:>5.2}% | {outone:>5.2}%)\n\
  Rtl: {rolltal:>5.2}%   (In/Out: {inrolltal:>5.2}% | {outrolltal:>5.2}%)\n\
  Red: {red:>5.2}%   (Bad:    {bad_red:>5.2}%)\n\
\n\
  SFB: {sfb:>5.2}%\n\
  SFS: {sfs:>5.2}%   (Red/Alt: {red_sfs:>5.2}% | {alt_sfs:>5.2}%)\n\
\n\
  LH/RH: {lh:>5.2}% | {rh:>5.2}%\n\
    ")
}

pub fn to_string(ll: &LayoutConfig, id: u64) -> String {
    let author_reader = authors::AUTHORS.read().unwrap();
    let author = author_reader.get_name(ll.user).unwrap_or("Unknown");
    let monograms = corpora::ngrams::<1>(id);
    let trigrams = corpora::ngrams::<3>(id);

    let matrix_str = get_matrix_str(ll);

    let stats = analyzer::trigrams(ll, &trigrams);
    let finger_usage = analyzer::fingers_usage(ll, &monograms);
    let stats_str = get_stats_str(&stats, &finger_usage);

    let likes = memory::get_like_count(&ll.name);
    let like_str = if likes == 1 {"like"} else {"likes"};
    let external_link = links::get_link(&ll.name);

    let ll_name = ll.name.as_str();
    let corpus_name = corpora::get_user_corpus(id).to_uppercase();
    format!("```\n\
             {ll_name} ({author}) ({likes} {like_str})\n\
             {matrix_str}\n
             \n\
             {corpus_name}:\n\
             {stats_str}\
             ```\n\
             {external_link}\n")
}