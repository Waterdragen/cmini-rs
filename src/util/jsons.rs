use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde_json::Value;
use serenity::futures::StreamExt;
use crate::util::core::Layout;

fn read_json(path: String) -> Value {
    let mut file = File::open(&path).expect(
        format!("Failed to open file {}", &path).as_str()
    );
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let json: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");
    json
}

pub fn get_map_str_str(path: String) -> HashMap<String, String> {
    let json = read_json(path);
    let mut hashmap: HashMap<String, String> = HashMap::new();

    if let Value::Object(obj) = json {
        for (key, value) in obj {
            if let Value::String(str_) = value {
                hashmap.insert(key, str_);
            }
        }
    }
    hashmap
}

pub fn get_vec_str(path: String) -> Vec<String> {
    let json = read_json(path);
    match json {
        Value::Array(arr) => {
            arr.into_iter()
                .filter_map(|val| Some(val.to_string()))
                .collect()
        }
        _ => Vec::new()
    }
}

pub fn get_map_str_vec_u64(path: String) -> HashMap<String, Vec<u64>> {
    let json = read_json(path);
    let mut hashmap: HashMap<String, Vec<u64>> = HashMap::new();

    if let Value::Object(obj) = json {
        for (key, value) in obj {
            if let Value::Array(arr) = value {
                let vec_ = arr
                    .into_iter()
                    .filter_map(|val| val.as_u64())
                    .collect();
                hashmap.insert(key, vec_);
            }
        }
    }
    hashmap
}

pub fn get_map_str_u64(path: String) -> HashMap<String, u64> {
    let json = read_json(path);
    let mut hashmap: HashMap<String, u64> = HashMap::new();

    if let Value::Object(obj) = json {
        for (key, value) in obj {
            if let Some(u64_) = value.as_u64() {
                hashmap.insert(key, u64_);
            }
        }
    }
    hashmap
}

pub fn get_layout(path: String) -> Layout {
    let json = read_json(path);
    serde_json::from_value(json).expect("Failed to parse layout")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_get_map_str_str() {
        let path = String::from("./links.json");
        let map = get_map_str_str(path);
        for (key, value) in map.into_iter() {
            println!("{}, {}", key, value);
        }
    }

    fn test_get_vec_str() {
        let path = String::from("./pairs.json");
        let vec = get_vec_str(path);
        for value in vec.into_iter() {
            println!("{}", value);
        }
    }

    fn test_get_map_str_vec_u64() {
        let path = String::from("./likes.json");
        let map = get_map_str_vec_u64(path);
        for (key, vec_) in map.into_iter() {
            println!("{}, {:?}", key, vec_);
        }
    }

    fn test_get_map_str_u64() {
        let path = String::from("./authors.json");
        let map = get_map_str_u64(path);
        for (key, value) in map.into_iter() {
            println!("{}, {}", key, value);
        }
    }

    fn test_get_layout() {
        let path = String::from("./layouts/a02.json");
        let layout = get_layout(path);
        println!("{:?}", layout);
    }
}
