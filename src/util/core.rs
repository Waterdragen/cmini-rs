use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Key {
    row: u8,
    col: u8,
    finger: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Layout {
    name: String,
    user: u64,
    board: String,
    keys: HashMap<String, Key>,
    free: Vec<Key>,
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