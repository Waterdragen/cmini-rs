use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use serenity::model::channel::Message as DiscordMessage;

pub struct Message<'a> {
    pub msg: &'a DiscordMessage,
    pub trigger: &'a str,
    pub action: &'a str,
    pub arg: &'a str,
    pub id: u64,
}

impl<'a> From<&'a DiscordMessage> for Message<'a> {
    fn from(msg: &'a DiscordMessage) -> Self {
        let id = *msg.author.id.as_u64();
        let is_dm = msg.is_private();

        let mut rest = &*msg.content;
        let mut trigger = "";
        if !is_dm {
            trigger = split_word(&mut rest);
        }
        let action = split_word(&mut rest);
        let arg = rest;

        Self {
            msg,
            trigger,
            action,
            arg,
            id,
        }
    }
}

impl<'a> Debug for Message<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("trigger", &self.trigger)
            .field("action", &self.action)
            .field("arg", &self.arg)
            .field("id", &self.id)
            .finish()
    }
}

fn split_word<'a>(s: &mut &'a str) -> &'a str {
    match s.split_once(char::is_whitespace) {
        None => {
            std::mem::take(s)
        },
        Some((first, rest)) => {
            let rest = rest.trim_start();
            *s = rest;
            first
        }
    }
}

impl<'a> Deref for Message<'a> {
    type Target = DiscordMessage;

    fn deref(&self) -> &Self::Target {
        self.msg
    }
}
