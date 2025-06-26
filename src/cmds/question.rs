use crate::{Commandable, Message};
use fxhash::FxHashMap;

static LINK: &str = "https://story-shack-cdn-v2.glitch.me/generators/random-question-generator";

pub struct Command;

impl Command {
    pub async fn exec(&self) -> String {
        get_random_question().await.unwrap_or_else(|| "?".to_owned())
    }
}

async fn get_random_question() -> Option<String> {
    let response: FxHashMap<String, FxHashMap<String, String>> = reqwest::get(LINK).await.ok()?.json().await.ok()?;
    Some(response.get("data")?.get("name")?.to_owned())
}

impl Commandable for Command {
    fn exec(&self, _: &Message) -> String {
        unimplemented!()
    }

    fn usage<'a>(&self) -> &'a str {
        "question [...]"
    }

    fn desc<'a>(&self) -> &'a str {
        "get a random question"
    }
}
