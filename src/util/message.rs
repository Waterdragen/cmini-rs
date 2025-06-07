use serenity::model::channel::Message as DiscordMessage;

pub struct Message<'a> {
    pub(crate) arg: String,
    pub(crate) id: u64,
    msg: &'a DiscordMessage,
}

impl<'a> Message<'a> {
    pub fn new(arg: String, id: u64, msg: &'a DiscordMessage) -> Self {
        Self { arg, id, msg }
    }
}