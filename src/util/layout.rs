use regex::Regex;
use crate::util::{analyzer, authors, corpora, links, memory};
use crate::util::core::{FingerUsage, LayoutConfig, Metric, Stat};

pub fn check_name(name: &str) -> Result<(), String> {
    let first_char = name.chars().next();
    if let Some(first_char) = first_char {
        if first_char == '_' {
            return Err(String::from("Error: names cannot start with an underscore"));
        }
    }
    if name.len() < 3 {
        return Err(String::from("Error: names must be at least 3 characters long"));
    }
    let re = Regex::new(r"^[a-zA-Z0-9_'\-():~]+$").unwrap();
    for c in name.chars() {
        if !re.is_match(&c.to_string()) {
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
        true => (&rows[..3]).join("\n"),
        false => rows.join("")
    }
}

pub fn get_stats_str(stats: &Stat, finger_usage: &FingerUsage) -> String {
    const LH: &u16 = &10;
    const RH: &u16 = &11;

    // get percentage of metric
    let get = |metric: &Metric| -> f64 {
        stats.get(metric).unwrap() * 100.0
    };
    let get_hand = |hand: &u16| -> f64 {
        finger_usage.get(hand).unwrap() * 100.0
    };

    let inroll = get(&Metric::InRoll);
    let outroll = get(&Metric::OutRoll);
    let inone = get(&Metric::InOne);
    let outone = get(&Metric::OutOne);

    let roll = inroll + outroll;
    let one = inone + outone;
    let inrolltal = inroll + inone;
    let outrolltal = outroll + outone;
    let rolltal = roll + one;

    let bad_red_sfs = get(&Metric::BadRedSfs);
    let bad_red = get(&Metric::BadRed) + bad_red_sfs;
    let red = get(&Metric::Red) + bad_red;

    let alt_sfs = get(&Metric::AltSfs);
    let red_sfs = get(&Metric::RedSfs) + bad_red_sfs;
    let sfs = alt_sfs + red_sfs;

    let stats_strs = [
        format!("  Alt: {:>5.2}%", get(&Metric::Alt)),
        format!("  Rol: {:>5.2}%   (In/Out: {:>5.2}% | {:>5.2}%)", roll, inroll, outroll),
        format!("  One: {:>5.2}%   (In/Out: {:>5.2}% | {:>5.2}%)", one, inone, outone),
        format!("  Rtl: {:>5.2}%   (In/Out: {:>5.2}% | {:>5.2}%)", rolltal, inrolltal, outrolltal),
        format!("  Red: {:>5.2}%   (Bad:    {:>5.2}%)", red, bad_red),
        String::new(),
        format!("  SFB: {:>5.2}%", get(&Metric::Sfb)),
        format!("  SFS: {:>5.2}%   (Red/Alt: {:>5.2}% | {:>5.2}%)", sfs, red_sfs, alt_sfs),
        String::new(),
        format!("  LH/RH: {:>5.2}% | {:>5.2}%", get_hand(LH), get_hand(RH)),
    ];
    stats_strs.join("\n")
}

pub fn to_string(ll: &LayoutConfig, id: u64) -> String {
    let author = authors::get_name(ll.user);
    let monograms = corpora::ngrams(1, id);
    let trigrams = corpora::ngrams(3, id);

    let matrix_str = get_matrix_str(ll);

    let stats = analyzer::trigrams(ll, &trigrams);
    let finger_usage = analyzer::fingers_usage(ll, &monograms);
    let stats_str = get_stats_str(&stats, &finger_usage);

    let likes = memory::get_like_count(&ll.name);
    let like_str = if likes == 1 {"like"} else {"likes"};
    let external_link = links::get_link(&ll.name);

    format!("```\n\
             {} ({}) ({} {})\n\
             {}\n
             \n\
             {}:\n\
             {}\
             ```\n\
             {}\n",
    ll.name, author, likes, like_str,
    matrix_str,
    corpora::get_user_corpus(id).to_uppercase(),
    stats_str,
    external_link)
}