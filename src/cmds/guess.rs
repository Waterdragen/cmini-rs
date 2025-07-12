use crate::{Commandable, Message, Lazy};
use crate::util::corpora;
use crate::util::minigames::{GuessPlayer, GUESS_PLAYERS};
use cmini_core::Key;
use rand::prelude::{SeedableRng, SliceRandom, StdRng};
use std::time::SystemTime;

pub static LETTERS: Lazy<Vec<Key>> = Lazy::new(|| ('a'..='z').collect());

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let mut players = GUESS_PLAYERS.lock();
        let player = match players.get_mut(&msg.id) {
            Some(player) if !should_reset(player.time) => player,
            _ => {
                let mut rng = StdRng::from_entropy();
                let mut chars = [' '; 3];
                for (char, rand_char) in chars.iter_mut().zip(LETTERS.choose_multiple(&mut rng, 3)) {
                    *char = *rand_char;
                }
                let bigrams = corpora::ngrams::<2>(0);
                let freq_percent = bigrams
                    .iter()
                    .filter_map(|([key0, key1], freq)| {
                        (chars.contains(key0) && chars.contains(key1)).then_some(*freq)
                    })
                    .sum::<u64>() as f64 / bigrams.sum as f64 * 100.0;
                players.insert(msg.id, GuessPlayer::new(chars, freq_percent));
                return format!("The column is `{}, {}, {}`, what is the total SF% (SFB + SFR)?", chars[0], chars[1], chars[2]);
            },
        };
        if msg.arg.is_empty() {
            let [key0, key1, key2] = player.chars;
            return format!("You need to make a guess! The column is `{key0}, {key1}, {key2}`");
        }
        let Ok(attempt) = msg.arg.trim_end_matches('%').parse::<f64>() else {
            return format!("Invalid number `{}`", msg.arg);
        };
        player.tries += 1;
        if (attempt - player.freq_percent).abs() < 0.01 {
            let tries = player.tries;
            let [key0, key1, key2] = player.chars;
            let freq = player.freq_percent;
            players.remove(&msg.id);
            return format!("You got it in {tries} tries! `{key0}, {key1}, {key2}` has {freq:.2}% total SF!");
        }
        match player.freq_percent > attempt {
            true => "higher".to_owned(),
            false => "lower".to_owned(),
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "guess <freq_percent>"
    }

    fn desc<'a>(&self) -> &'a str {
        "guess the total SF% of a given column"
    }
}

pub fn should_reset(earlier: SystemTime) -> bool {
    let now = SystemTime::now();
    match now.duration_since(earlier) {
        Ok(dur) => dur.as_secs() / 60 > 30,  // reset after 30 minutes
        Err(_) => false,  // encountered time drift caused by non-monotonic clock, that means `earlier` is very close to `now`
    }
}
