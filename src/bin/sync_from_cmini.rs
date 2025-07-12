use cmini_rs::util::jsons::JsonError;
use cmini_rs::util::jsons::{read_json_checked, write_json_checked};
use cmini_core::{Finger, FxIndexMap};
use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{IntoIterator, Iterator};

const FOLDER: &str = "./cmini/";
const INPUT: &str = "./cmini/input/";
const OUTPUT: &str = "./cmini/output/";

const AUTHORS: &str = "authors.json";
const LAYOUTS: &str = "layouts/";
const OUTPUT_LAYOUTS: &str = "./cmini/output/layouts.json";
const LIKES: &str = "likes.json";

fn sync_authors() -> Result<(), JsonError> {
    let authors: FxIndexMap<String, u64> = read_json_checked(&format!("{INPUT}{AUTHORS}"))?;
    let mut new_authors = FxIndexMap::<u64, Vec<String>>::default();
    for (name, id) in authors {
        let names = new_authors.entry(id).or_default();
        if !names.contains(&name) {
            names.push(name);
        }
    }
    write_json_checked(&format!("{OUTPUT}{AUTHORS}"), &new_authors)?;
    Ok(())
}

fn sync_likes() -> Result<(), JsonError> {
    let likes: FxIndexMap<String, Vec<u64>> = read_json_checked(&format!("{INPUT}{LIKES}"))?;
    let likes = likes.into_iter()
        .map(|(name, liked_users)| (name.to_lowercase(), liked_users))
        .collect::<FxIndexMap<_, _>>();
    write_json_checked(&format!("{OUTPUT}{LIKES}"), &likes)?;
    Ok(())
}

#[derive(Deserialize)]
struct CminiLayoutConfig {
    name: String,
    user: u64,
    board: String,
    keys: FxHashMap<char, CminiPosition>,
}

#[derive(Deserialize)]
struct CminiPosition {
    row: usize,
    col: usize,
    finger: String,
}

#[derive(Serialize)]
struct JsonLayoutConfig {
    pub user: u64,
    pub board: String,
    pub keys: String,
}

fn sync_layouts() -> Result<(), Box<dyn Error>> {
    let path = format!("{INPUT}{LAYOUTS}");
    if !matches!(fs::exists(&path), Ok(true)) {
        return Ok(());
    }
    let mut map = FxIndexMap::default();
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        if matches!(reader.fill_buf(), Ok(&[])) {
            continue;
        }
        let ll: CminiLayoutConfig = serde_json::from_reader(reader)?;
        let mut keyboard = [[(' ', '0'); 36]; 4];
        for (key, pos) in ll.keys.iter() {
            let (col, mut finger) = match Finger::try_from_str(&pos.finger) {
                Ok(finger) => (pos.col, finger),
                Err(_) => (6, Finger::RT)  // Encounter TB
            };
            if pos.row == 3 && pos.finger != "TB" {
                // Convert column to corresponding thumb
                finger = if col < 5 { Finger::LT } else { Finger::RT };
            }
            keyboard[pos.row][col] = (*key, finger.as_digit_char());
        }
        // Right-justify left thumbs
        if let Some(idx) = keyboard[3].iter().rposition(|(_, finger)| *finger == '4') {
            let fill = 4 - idx;
            keyboard[3][0..5].rotate_right(fill);
        }
        let mut keys_packed = "".to_owned();

        #[allow(clippy::needless_range_loop)]
        for row_idx in 0..4 {
            for col_idx in 0..36 {
                let (key, finger) = keyboard[row_idx][col_idx];
                if key == ' ' { continue; }
                keys_packed.push(key);
                keys_packed.push(char::from_digit(row_idx as u32, 4).unwrap());
                keys_packed.push(char::from_digit(col_idx as u32, 36).unwrap());
                keys_packed.push(finger);
            }
        }
        let CminiLayoutConfig { name, user, board, .. } = ll;
        let name = name.to_lowercase();
        map.insert(name, JsonLayoutConfig {
            user,
            board,
            keys: keys_packed,
        });
    }
    map.sort_unstable_keys();
    write_json_checked(OUTPUT_LAYOUTS, &map)?;
    Ok(())
}

fn main_controller() -> Result<(), Box<dyn Error>> {
    let _ = fs::create_dir(OUTPUT);
    match sync_authors() {
        Ok(_) | Err(JsonError::FileNotFound) => {},
        err => err?,
    }
    match sync_likes() {
        Ok(_) | Err(JsonError::FileNotFound) => {},
        err => err?,
    }
    sync_layouts()?;
    Ok(())
}

fn main() {
    if !matches!(fs::exists(FOLDER), Ok(true)) {
        println!("Creating a folder `{FOLDER}` in root");
        fs::create_dir(FOLDER).unwrap();
        return;
    }
    if fs::create_dir(INPUT).is_ok() {
        println!("Creating input folder");
    }
    if fs::create_dir(OUTPUT).is_ok() {
        println!("Creating output folder");
    }
    if let Err(err) = main_controller() {
        println!("Error: {err}");
    } else {
        println!("Successfully converted cmini jsons to `{OUTPUT}`");
    }
}