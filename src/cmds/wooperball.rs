use crate::util::{Commandable, Message};
use rand::prelude::{SeedableRng, SliceRandom, StdRng};

const CHOICES: [(&str, u8); 3] = [
    ("<a:wooperno:1085582045121613874>", 49),
    ("<a:wooper:1081033714043207771>", 49),
    ("<:woopwoop:1168675851592798288>", 1),
];

pub struct Command;

impl Commandable for Command {
    fn exec(&self, _: &Message) -> String {
        let mut rng = StdRng::from_entropy();
        CHOICES.choose_weighted(&mut rng,
                                |choice| choice.1)
            .unwrap().0  // Always succeeds: CHOICES is not empty, all weights >= 0, sum of weights > 0
            .to_owned()
    }

    fn usage<'a>(&self) -> &'a str {
        "wooperball [...]"
    }

    fn desc<'a>(&self) -> &'a str {
        "get a wooper"
    }
}