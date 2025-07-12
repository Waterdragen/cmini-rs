use crate::util::memory::LAYOUTS;
use crate::{Commandable, Message};
use rand::prelude::{IteratorRandom, SeedableRng, StdRng};
use crate::util::layout::to_pretty;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let mut rng = StdRng::from_entropy();
        let layouts = LAYOUTS.read();
        let index = (0..layouts.len()).choose(&mut rng).unwrap();  // Always succeeds: LAYOUTS is not empty
        let (_, ll) = layouts.iter().nth(index).unwrap();  // Index is always valid
        to_pretty(ll, msg.id)
    }

    fn usage<'a>(&self) -> &'a str {
        "random [...]"
    }

    fn desc<'a>(&self) -> &'a str {
        "get a random layout"
    }
}