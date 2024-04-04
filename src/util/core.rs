use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub row: u8,
    pub col: u8,
    pub finger: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Layout {
    pub name: String,
    pub user: u64,
    pub board: String,
    pub keys: HashMap<String, Position>,
    #[serde(default)]
    pub free: Vec<Position>,
}

#[derive(PartialEq)]
pub enum ArgType {
    Str,
    Vec,
}

#[derive(PartialEq)]
pub enum KwargType {
    Bool,
    Vec,
    Str,
}

pub struct KwargValue {
    bool_: Option<bool>,
    vec_: Option<Vec<String>>,
    str_: Option<String>,
}

impl KwargValue {
    pub fn from_bool(bool_: bool) -> Self {
        KwargValue{bool_: Some(bool_), vec_: None, str_: None}
    }
    pub fn from_vec(vec_: Vec<String>) -> Self {
        KwargValue{bool_: None, vec_: Some(vec_), str_: None}
    }
    pub fn from_str(str_: String) -> Self {
        KwargValue{bool_: None, vec_: None, str_: Some(str_)}
    }
    pub fn from_none() -> Self {
        KwargValue{bool_: None, vec_: None, str_: None}
    }
    pub fn get_bool(self) -> bool {
        self.bool_.unwrap_or_else(|| false)
    }
    pub fn get_vec(self) -> Vec<String> {
        self.vec_.unwrap_or_else(move || Vec::new())
    }
    pub fn get_str(self) -> String {
        self.str_.unwrap_or_else(|| String::new())
    }
}

impl Debug for KwargValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match (self.bool_, self.vec_.as_ref(), self.str_.as_ref()) {
            (Some(bool_), _, _) => bool_.to_string(),
            (_, Some(vec_), _) => format!("{:?}", vec_),
            (_, _, Some(str_)) => str_.to_string(),
            _ => String::from("<None>"),
        };
        write!(f, "{}", s)
    }
}

pub type Corpus = Arc<Vec<(Vec<char>, u64)>>;
