use crate::util::consts::ADMINS;
use crate::util::memory::{remove, remove_as_admin, RemoveError};
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
        if !ADMINS.contains(&msg.id) {
            return match remove(name, msg.id) {
                Ok(_) => format!("`{name}` has been removed"),
                Err(err) => err.to_string(),
            };
        }
        let kwarg = match get_kwargs(&name, &KWARGS) {
            Ok(kwarg) => kwarg,
            Err(err) => return err.to_string(),
        };
        let result = match kwarg["sudo"].unwrap_bool() {
            true => remove_as_admin(&kwarg.arg, msg.id),
            false => remove(&kwarg.arg, msg.id),
        };
        match result {
            Ok(_) => format!("`{}` has been removed", kwarg.arg),
            Err(err @ RemoveError::NotFound(_)) => err.to_string(),
            Err(err @ RemoveError::NotOwner(_)) =>
                format!("{}\nHelp: you may remove it with `remove {} --sudo`",
                        err.to_string(), kwarg.arg),
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "remove <layout>"
    }

    fn desc<'a>(&self) -> &'a str {
        "delete one of your layouts"
    }
}