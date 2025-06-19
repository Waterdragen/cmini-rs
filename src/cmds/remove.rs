use crate::util::admins::ADMINS;
use crate::util::memory::{LAYOUTS, RemoveError};
use crate::util::parser::{get_kwargs, KwargType};
use crate::util::{Commandable, Message};
use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use std::borrow::ToOwned;

static KWARGS: Lazy<FxHashMap<String, KwargType>>
= Lazy::new(|| FxHashMap::from_iter([
    ("sudo".to_owned(), KwargType::Bool),
]));

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let name = msg.arg;
        if !ADMINS.contains(msg.id) {
            if name.is_empty() {
                return self.help();
            }
            return match LAYOUTS.remove(name, msg.id) {
                Ok(_) => format!("`{name}` has been removed"),
                Err(err) => err.to_string(),
            };
        }
        let kwargs = match get_kwargs(name, &KWARGS) {
            Ok(kwarg) => kwarg,
            Err(err) => return err.to_string(),
        };
        let name = &kwargs.arg;
        if name.is_empty() {
            return "```\nremove <layout_name> [--sudo]\ndelete cmini layouts```".to_owned();
        }
        let result = match kwargs["sudo"].unwrap_bool() {
            true => LAYOUTS.remove_as_admin(name, msg.id, !msg.is_private()),
            false => LAYOUTS.remove(name, msg.id),
        };
        match result {
            Ok(_) => format!("`{name}` has been removed"),
            Err(err @ RemoveError::NotOwner(_)) =>
                format!("{err}\nHelp: you may remove it with `remove {name} --sudo`"),
            Err(err) => err.to_string(),
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "remove <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "delete one of your layouts"
    }
}