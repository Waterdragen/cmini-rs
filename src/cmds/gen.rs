use crate::message::Message;
use crate::prelude::Commandable;
use crate::util::layout::to_pretty;
use crate::util::memory::{PAIRS, PLACES};
use cmini_core::consts::FREE_CHAR;
use cmini_core::Finger::{self, *};
use cmini_core::{Col, LayoutConfig, Position, Row};
use fxhash::{FxHashMap, FxHashSet};
use rand::prelude::{SeedableRng, StdRng};
use rand::Rng;

const FINGER_MAP: [Finger; 30] = [
    LP, LR, LM, LI, LI, RI, RI, RM, RR, RP,
    LP, LR, LM, LI, LI, RI, RI, RM, RR, RP,
    LP, LR, LM, LI, LI, RI, RI, RM, RR, RP,
];

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let mut ll_gen = Generator::new();
        let mut poss = init_positions();

        while !poss.is_empty() {
            let (&letter, rand_indices) = poss.iter()
                .min_by_key(|(_, rand_indices)| rand_indices.len())
                .unwrap();  // poss is not empty, checked by while loop

            if rand_indices.is_empty() {
                return "Unknown error generating layout".to_owned();
            }

            let idx = ll_gen.set_letter(letter, rand_indices);
            let collisions = ll_gen.collisions(idx);

            poss.remove(&letter).unwrap();  // letter is from poss min_by_key

            for (&ch, index_set) in poss.iter_mut() {
                index_set.remove(&idx);  // remove if idx in set

                if PAIRS.contains(&[ch, letter]) ||
                    PAIRS.contains(&[letter, ch]) {
                    continue;
                }

                for collision_idx in collisions.iter() {
                    index_set.remove(collision_idx);  // remove if idx in set
                }
            }
        }
        let keys = ll_gen.matrix.iter()
            .enumerate()
            .filter_map(|(idx, &key)| {
                (key != FREE_CHAR).then_some(())?;
                let row = (idx / 10) as Row;
                let col = (idx % 10) as Col;
                let finger = FINGER_MAP[idx];
                Some((key, Position::new(row, col, finger)))
            })
            .collect::<FxHashMap<_, _>>();

        let res = LayoutConfig::new(
            "generated".to_owned(),
            1085579430623199292,
            "ortho".to_owned(),
            keys,
        );
        to_pretty(&res, msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "gen"
    }

    fn desc<'a>(&self) -> &'a str {
        "generate a random layout"
    }
}
struct Generator {
    matrix: [char; 30],
    rng: StdRng,
}

impl Generator {
    fn new() -> Self {
        Self {
            matrix: [FREE_CHAR; 30],
            rng: StdRng::from_entropy(),
        }
    }
    fn set_letter(&mut self, letter: char, rand_indices: &FxHashSet<usize>) -> usize {
        let nth = self.rng.gen_range(0..rand_indices.len());  // checked by caller: rand_indices must not be empty
        let idx = *rand_indices.iter().nth(nth).unwrap();  // nth is checked within range
        self.matrix[idx] = letter;
        idx
    }

    fn collisions(&self, index: usize) -> Vec<usize> {
        let target_finger = FINGER_MAP[index];
        FINGER_MAP.iter()
            .enumerate()
            .filter_map(move |(idx, &finger)| {
                (finger == target_finger && self.matrix[idx] == FREE_CHAR).then_some(idx)
            })
            .collect()
    }
}

fn init_positions() -> FxHashMap<char, FxHashSet<usize>> {
    let default_pos = FxHashSet::<usize>::from_iter(0..30);
    let mut poss = ('a'..='z')
        .map(|c| (c, default_pos.clone()))
        .collect::<FxHashMap<_, _>>();

    for (idx, letter_in_places) in PLACES.iter()
        .flat_map(|s| s.chars().enumerate())
        .filter(|&(_, c)| c != FREE_CHAR)
    {
        let pos = poss.get_mut(&letter_in_places)
            .unwrap();  // all chars in places.json are a..=z
        if pos.len() == 30 {
            *pos = FxHashSet::from_iter([idx]);
        } else {
            pos.insert(idx);
        }
    }
    poss
}