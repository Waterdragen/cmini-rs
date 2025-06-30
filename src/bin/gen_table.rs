use cmini_rs::core::Metric;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

const PATH: &str = "table.json";
const FINGERS: [&str; 10] = ["LP", "LR", "LM", "LI", "LT", "RT", "RI", "RM", "RR", "RP"];
const BAD_RED_MAP: [u8; 10] = [1, 1, 1, 0, 0, 0, 0, 1, 1, 1];

fn gen_table() -> Vec<(String, &'static str)> {
    let mut table = Vec::<(String, &'static str)>::with_capacity(1000);

    let mut add = |combo: [usize; 3], metric: Metric| {
        let mut s = "".to_owned();
        for finger in combo {
            s.push_str(FINGERS[finger]);
        }
        table.push((s, metric.into()));
    };

    let yield_finger_combo = (0..10).flat_map(|i| (0..10).flat_map(move |j| (0..10).map(move |k| [i, j, k])));
    for combo in yield_finger_combo {
        let [finger0, finger1, finger2] = combo;
        let (hand0, hand1, hand2) = (finger0 >= 5, finger1 >= 5, finger2 >= 5);
        if hand0 != hand1 && hand1 != hand2 {
            match finger0 != finger2 {
                true => add(combo, Metric::Alt),
                false => add(combo, Metric::AltSfs),
            }
            continue;
        }
        let sf_count = (finger0 == finger1) as u8 + (finger1 == finger2) as u8;
        if sf_count > 0 {
            match sf_count == 1 {
                true => add(combo, Metric::Sfb),
                false => add(combo, Metric::Sft),
            }
            continue;
        }
        if hand0 == hand1 && hand1 == hand2 {
            let roll_to_left = finger0 > finger1 && finger1 > finger2;
            if roll_to_left || finger0 < finger1 && finger1 < finger2 {
                match roll_to_left == hand0 {
                    true => add(combo, Metric::InOne),
                    false => add(combo, Metric::OutOne),
                }
            } else {
                let is_sfs = finger0 == finger2;
                let is_bad = (BAD_RED_MAP[finger0] + BAD_RED_MAP[finger1] + BAD_RED_MAP[finger2]) == 3;
                match (is_sfs, is_bad) {
                    (false, false) => add(combo, Metric::Red),
                    (false, true) => add(combo, Metric::BadRed),
                    (true, false) => add(combo, Metric::RedSfs),
                    (true, true) => add(combo, Metric::BadRedSfs),
                }
            }
            continue;
        }
        let roll_to_left = if hand0 == hand1 { finger0 > finger1 } else { finger1 > finger2 };
        match roll_to_left == hand1 {
            true => add(combo, Metric::InRoll),
            false => add(combo, Metric::OutRoll)
        }
    }
    table
}

fn write_table_to_json(table: &[(String, &str)]) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(PATH)?;
    writeln!(file, "{{")?;
    let last = table.len() - 1;
    for (combo, metric) in &table[..last] {
        writeln!(file, "    \"{combo}\": \"{metric}\",")?;
    }
    let (combo, metric) = &table[last];
    writeln!(file, "    \"{combo}\": \"{metric}\"")?;
    write!(file, "}}")?;
    file.flush()?;
    Ok(())
}

fn main_controller() -> Result<(), Box<dyn Error>> {
    let table = gen_table();
    write_table_to_json(&table)?;
    Ok(())
}

fn main() {
    match main_controller() {
        Ok(_) => println!("Successfully generated table at `{PATH}`"),
        Err(err) => println!("Error:\n{err}\n\nFailed to generate table")
    }
}