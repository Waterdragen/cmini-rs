use fxhash::FxHashMap;
use thiserror::Error;
use std::ops::{Deref, DerefMut};
use std::slice::SliceIndex;

#[derive(Debug, Error)]
pub enum ParseKwargError {
    #[error("Invalid kwarg: {0}")]
    Invalid(String),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KwargType {
    Bool,
    Vec,
    Str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kwarg {
    Bool(bool),
    Vec(Option<Vec<String>>),
    Str(Option<String>),
}

impl Kwarg {
    pub fn default_from_type(kt: KwargType) -> Self {
        match kt {
            KwargType::Bool => Self::Bool(false),
            KwargType::Vec => Self::Vec(None),
            KwargType::Str => Self::Str(None),
        }
    }
}

impl Kwarg {
    pub fn unwrap_bool(&self) -> bool {
        match self {
            Kwarg::Bool(b) => *b,
            _ => panic!("{self:?} is not a bool")
        }
    }

    pub fn unwrap_vec(&self) -> Option<&[String]> {
        match self {
            Kwarg::Vec(v) => Some(v.as_ref()?),
            _ => panic!("{self:?} is not a vec")
        }
    }

    pub fn unwrap_str(&self) -> Option<&str> {
        match self {
            Kwarg::Str(s) => Some(s.as_ref()?),
            _ => panic!("{self:?} is not a string")
        }
    }
}

#[derive(Debug)]
pub struct KwargData {
    pub arg: String,
    kwargs: FxHashMap<String, Kwarg>,
}

impl Deref for KwargData {
    type Target = FxHashMap<String, Kwarg>;

    fn deref(&self) -> &Self::Target {
        &self.kwargs
    }
}

impl DerefMut for KwargData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kwargs
    }
}

pub fn get_args(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

pub fn get_layout(s: &str) -> (String, String) {
    let parts: Vec<&str> = s.split("```").collect();

    let name = parts.first().unwrap_or(&"").trim().to_lowercase();
    let matrix = parts.get(1).unwrap_or(&"").trim().to_lowercase();

    (name, matrix)
}

pub fn get_kwargs(s: &str, cmd_kwargs: &FxHashMap<String, KwargType>)
                  -> Result<KwargData, ParseKwargError> {
    let words: Vec<&str> = s.split_whitespace().collect();

    let is_kwargs = words.iter()
        .map(|word| is_kwarg(cmd_kwargs, word))
        .collect::<Result<Vec<_>, _>>()?;
    let arg_index = is_kwargs.iter().position(|x| *x).unwrap_or(words.len());

    // Make default hashmap
    let arg = words[..arg_index].join(" ");
    let mut kwarg_data = KwargData {
        arg,
        kwargs: FxHashMap::from_iter(
            cmd_kwargs
                .iter()
                .map(|(kw_name, kw_type)|
                    (kw_name.clone(), Kwarg::default_from_type(*kw_type)))
        ),
    };

    let words: &[&str] = &words[arg_index..];
    let mut last_in_vec = 0usize;
    let mut last_kwarg_type = KwargType::Vec;
    let mut last_vec_kwarg = String::new();
    let mut in_vec = false;
    for (index, word) in words.iter().enumerate() {
        if !is_kwargs[arg_index + index] {
            continue;
        }
        let word = remove_kw_prefix(word);
        let kw_type = *cmd_kwargs.get(&word).unwrap();

        // Encountered next keyword, stops previous vec
        if in_vec {
            kwarg_data.insert(last_vec_kwarg.clone(),
                              slice_kwarg(last_kwarg_type, &words[last_in_vec..index]),
            );
        }
        in_vec = kw_type == KwargType::Vec || kw_type == KwargType::Str;
        if !in_vec {
            kwarg_data.insert(word.clone(), Kwarg::Bool(true));
        }

        // Starts a new list after kwarg
        if in_vec {
            last_kwarg_type = kw_type;
            last_vec_kwarg = word;
            last_in_vec = index + 1;
        }
    }

    // Close the last vec
    if in_vec {
        kwarg_data.insert(last_vec_kwarg,
                          slice_kwarg(last_kwarg_type, &words[last_in_vec..]),
        );
    }

    Ok(kwarg_data)
}

pub fn slice_kwarg(last_kwarg_type: KwargType, words: &[&str]) -> Kwarg {
    if last_kwarg_type == KwargType::Vec {
        Kwarg::Vec(Some(
            words.iter()
                .map(|s| s.to_string())
                .collect()
        ))
    } else {
        Kwarg::Str(Some(words.join(" ")))
    }
}

pub fn map_from_vec(v: Vec<(&str, KwargType)>) -> FxHashMap<String, KwargType> {
    v.into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect()
}

fn starts_with_kw_prefix(word: &str) -> bool {
    ["--", "—", "––"].iter().any(|prefix| word.starts_with(prefix))
}

fn remove_kw_prefix(word: &str) -> String {
    let mut word = word.to_string();
    for prefix in vec!["--", "—", "––"].into_iter() {
        if let Some(new_word) = word.strip_prefix(prefix) {
            word = new_word.to_string();
            break;
        }
    }
    word.to_lowercase()
}

/// Check if word is a kwarg, while checking if it is valid.
/// If the word is valid, returns `Ok`(`true`/`false`), else `Err`.
fn is_kwarg(kwargs: &FxHashMap<String, KwargType>, word: &str) -> Result<bool, ParseKwargError> {
    if !starts_with_kw_prefix(word) {
        return Ok(false);
    }
    let word = remove_kw_prefix(word);
    match kwargs.contains_key(&word) {
        true => Ok(true),
        false => Err(ParseKwargError::Invalid(word.to_owned())),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::parser::KwargType as KT;

    macro_rules! str {
        ($expr:expr) => {
            String::from($expr)
        };
    }

    #[test]
    fn test_kwarg() {
        let cmd_kwargs = map_from_vec(vec![
            ("vec", KT::Vec),
            ("bool", KT::Bool),
            ("str", KT::Str),
        ]);
        let kwargs = get_kwargs("", &cmd_kwargs).unwrap();
        assert_eq!(kwargs.arg, "");
        dbg!(&kwargs);
        let kwargs = get_kwargs("hello vec --vec 1 2 3", &cmd_kwargs).unwrap();
        assert_eq!(kwargs.arg, "hello vec");
        assert_eq!(kwargs["vec"].unwrap_vec(), Some(&*vec![str!("1"), str!("2"), str!("3")]));
        let kwargs = get_kwargs("hello str --str bogos binted", &cmd_kwargs).unwrap();
        assert_eq!(kwargs.arg, "hello str");
        assert_eq!(kwargs["str"].unwrap_str(), Some("bogos binted"));
        let kwargs = get_kwargs("hello bool --bool", &cmd_kwargs).unwrap();
        assert_eq!(kwargs.arg, "hello bool");
        assert!(kwargs["bool"].unwrap_bool());
        let kwargs = get_kwargs("hello all --vec a b --str c d --bool", &cmd_kwargs).unwrap();
        assert_eq!(kwargs.arg, "hello all");
        assert_eq!(kwargs["vec"].unwrap_vec(), Some(&*vec![str!("a"), str!("b")]));
        assert_eq!(kwargs["str"].unwrap_str(), Some("c d"));
        assert!(kwargs["bool"].unwrap_bool());
        let not_kwargs = get_kwargs("hello all --other --flag", &cmd_kwargs);
        assert!(not_kwargs.is_err());
    }
}

