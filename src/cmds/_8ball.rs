use crate::util::core::{Commandable, DynCommand};
use rand::SeedableRng;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;

const RESPONSES: [&'static str; 12] = [
    "Yes", "Count on it", "No doubt", "Absolutely", "Very likely",
    "Maybe", "Perhaps",
    "No", "No chance", "Unlikely", "Doubtful", "Probably not"
];

pub struct Command;

impl Commandable for Command {
    fn init() -> DynCommand where Self: Sized + 'static {
        Box::new(Command {})
    }

    fn exec(&self, _: &str, _: u64) -> String {
        let mut rng = StdRng::from_entropy();
        RESPONSES.choose(&mut rng).unwrap().to_string()
    }

    fn usage<'a>(&self) -> &'a str {
        "8ball [anything]"
    }

    fn desc<'a>(&self) -> &'a str {
        "let cmini decide the likelihood of [anything]"
    }
}