use rand::prelude::{StdRng, SeedableRng, SliceRandom};
use crate::{Commandable, Message};

const CHOICES: [(&str, u8); 3] = [
    ("Heads", 49),
    ("Tails", 49),
    ("Mail", 1),
];

pub struct Command;

impl Commandable for Command {
    fn exec(&self, _: &Message) -> String {
        let mut rng = StdRng::from_entropy();
        CHOICES.choose_weighted(&mut rng, |choice| choice.1)
            .unwrap().0  // Always succeeds: CHOICES is not empty, all weights >= 0, sum of weights > 0
            .to_owned()
    }

    fn usage<'a>(&self) -> &'a str {
        "flip [...]"
    }

    fn desc<'a>(&self) -> &'a str {
        "flip a coin"
    }
}