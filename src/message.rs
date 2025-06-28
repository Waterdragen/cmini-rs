use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use serenity::model::channel::Message as DiscordMessage;
use serenity::prelude::Context;
use crate::consts::CMINI_CHANNEL;
use crate::util::parser::{split_word, split_words};

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
        let id = *msg.author.id.as_u64();
        let is_dm = msg.is_private();

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

pub struct BoundedResponse {
    inner: String,
    len: usize,
    /// Number of characters before hard limit
    reserved: usize,
}

impl From<String> for BoundedResponse {
    fn from(string: String) -> Self {
        let len = string.chars().count();
        Self {
            inner: string,
            len,
            reserved: 0,
        }
    }
}

impl BoundedResponse {
    const LIMIT: usize = 2000;

    pub fn reserve(mut self, reserved: usize) -> Self {
        assert!(reserved < Self::LIMIT);
        self.reserved = reserved;
        self
    }

    fn add_len(&mut self, inc: usize) -> Result<(), ()> {
        self.len += inc;
        if self.len > Self::LIMIT - self.reserved {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn push_str(&mut self, s: &str) -> Result<(), ()> {
        let inc = s.chars().count();
        self.add_len(inc)?;
        self.inner.push_str(s);
        Ok(())
    }

    pub fn push_line(&mut self, s: &str) -> Result<(), ()> {
        self.push_str(s)?;
        self.push('\n')?;
        Ok(())
    }

    pub fn push(&mut self, c: char) -> Result<(), ()> {
        self.add_len(1)?;
        self.inner.push(c);
        Ok(())
    }

    // pub fn try_remove_line(&mut self) -> usize {
    //     if let Some(suffix) = self.inner.rsplit('\n').next() {
    //         let freed = suffix.chars().count() + 1;
    //         for _ in 0..freed {
    //             self.inner.pop();  // Pop suffix + newline character
    //         }
    //         self.len -= freed;
    //         freed
    //     } else {
    //         0
    //     }
    // }

    pub fn finish(self) -> String {
        self.inner
    }
}

pub struct BoundedResponseVec {
    inner: Vec<String>,
    len: usize,
    /// Number of characters before hard limit
    reserved: usize,
}

impl From<Vec<String>> for BoundedResponseVec {
    fn from(value: Vec<String>) -> Self {
        let len = value.iter().fold(0usize, |acc, s| acc + s.chars().count());
        Self {
            inner: value,
            len,
            reserved: 0,
        }
    }
}

impl BoundedResponseVec {
    const LIMIT: usize = 2000;

    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            len: 0,
            reserved: 0,
        }
    }

    pub fn reserve(mut self, reserved: usize) -> Self {
        assert!(reserved < Self::LIMIT);
        self.reserved = reserved;
        self
    }

    fn add_len(&mut self, inc: usize) -> Result<(), ()> {
        self.len += inc;
        if self.len > Self::LIMIT - self.reserved {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn push(&mut self, string: String) -> Result<(), ()> {
        let inc = string.chars().count();
        self.add_len(inc)?;
        self.inner.push(string);
        Ok(())
    }

    pub fn finish(self) -> Vec<String> {
        self.inner
    }
}

