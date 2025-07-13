use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use serenity::model::channel::Message as DiscordMessage;
use serenity::prelude::Context;
use std::sync::{Arc, OnceLock};
use serenity::all::Http;
use crate::consts::CMINI_CHANNEL;
use crate::util::parser::{split_word, split_words};

pub static BOT_CONTEXT: OnceLock<Context> = OnceLock::new();
pub static BOT_CLIENT_HTTP: OnceLock<Arc<Http>> = OnceLock::new();

pub struct Message<'a> {
    pub msg: &'a DiscordMessage,
    pub trigger: &'a str,
    pub action: &'a str,
    pub arg: &'a str,
    pub id: u64,
    pub context: &'a Context,
}

impl<'a> Message<'a> {
    pub fn in_cmini_channel(&self) -> bool {
        self.msg.channel_id == CMINI_CHANNEL
    }
}

impl<'a> Message<'a> {
    pub fn from_msg_ctx(msg: &'a DiscordMessage, context: &'a Context) -> Self {
        let id = msg.author.id.get();
        let is_dm = msg.guild_id.is_none();

        let mut rest = &*msg.content;
        let mut trigger = "";
        if !is_dm {
            trigger = split_word(&mut rest);
        }
        let [action, arg] = split_words(rest);

        Self {
            msg,
            trigger,
            action,
            arg,
            id,
            context,
        }
    }
    pub fn is_private(&self) -> bool {
        self.msg.guild_id.is_none()
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

impl<'a> Deref for Message<'a> {
    type Target = DiscordMessage;

    fn deref(&self) -> &Self::Target {
        self.msg
    }
}