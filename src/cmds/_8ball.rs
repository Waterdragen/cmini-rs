use crate::util::Commandable;
use rand::SeedableRng;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use crate::util::Message;

const RESPONSES: [&str; 12] = [
    "Yes", "Count on it", "No doubt", "Absolutely", "Very likely",
    "Maybe", "Perhaps",
    "No", "No chance", "Unlikely", "Doubtful", "Probably not"
];

pub struct Command;

impl Commandable for Command {
    fn exec(&self, _: &Message) -> String {
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