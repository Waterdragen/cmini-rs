use crate::util::memory::DIRECT_WRITE_LOCK;
use crate::{Commandable, Message};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let _lock = DIRECT_WRITE_LOCK.lock();
        match append_suggestion(msg.arg) {
            Ok(_) => "Received :)".to_owned(),
            Err(_) => "Received ;)".to_owned(),
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "suggest <message>"
    }

    fn desc<'a>(&self) -> &'a str {
        "send me a suggestion for how to improve cmini :)"
    }
}

fn append_suggestion(suggestion: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .append(true)
        .open("./suggestions.txt")?;
    writeln!(file, "{}", suggestion.trim_end())?;
    Ok(())
}