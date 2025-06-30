use crate::core::Position;
use crate::util::memory::LAYOUTS;
use crate::{Message, Lazy, Commandable};
use fxhash::FxHashMap;
use itertools::Itertools;
use std::borrow::Cow;
use std::iter::{IntoIterator, Iterator};

static NAME_MAP: Lazy<FxHashMap<char, (&str, &str)>> = Lazy::new(|| [
    ('`', ("grave", "asciitilde")),
    ('1', ("1",     "exclam")),
    ('2', ("2",     "at")),
    ('3', ("3",     "numbersign")),
    ('4', ("4",     "dollar")),
    ('5', ("5",     "percent")),
    ('6', ("6",     "asciicircum")),
    ('7', ("7",     "ampersand")),
    ('8', ("8",     "asterisk")),
    ('9', ("9",     "parenleft")),
    ('0', ("0",     "parenright")),
    
    ('[', ("bracketleft",  "braceleft")),
    (']', ("bracketright", "braceright")),
    ('\\', ("backslash",   "bar")),
    ('-', ("minus",        "underscore")),
    ('=', ("equal",        "plus")),
    (';', ("semicolon",    "colon")),
    ('\'', ("apostrophe",  "quotedbl")),
    (',', ("comma",        "less")),
    ('.', ("period",       "greater")),
    ('/', ("slash",        "question")),
].into_iter().collect());
const ROW_NAMES: [&str; 3] = ["AD", "AC", "AB"];

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        if msg.arg.is_empty() {
            return self.help();
        }
        let ll = &*LAYOUTS.find(msg.arg);
        let ll_name = &ll.name;
        let mut xkb = format!("```\ndefault partial alphanumeric_keys modifier_keys\n\
                                  xkb_symbols \"basic\" {{\n\n    name[Group1]= \"{ll_name}\";\n\n");
        xkb.push_str(&format_xkb_item_with_names("TLDE", "grave".into(), "asciitilde".into()));  // Tilde key (` ~)
        for (idx, key) in "1234567890-=".chars().enumerate() {
            let key_id = format!("AE{:0>2}", idx + 1);
            xkb.push_str(&format_xkb_item(&key_id, key));
        }
        // Other rows
        // It's too late to implement this in a better way, sorry
        // This also assumes ANSI enter key (\ above enter?)
        // since that's how qwerty.json is defined
        let mut warn_has_thumb = false;
        let mut current_row = u8::MAX;
        let sorted_ll = ll.sorted_layout();
        for &(key, Position {row, col, ..}) in sorted_ll.iter() {
            if row == 3 {  // layout is sorted by (row, col), subsequent rows are also 3
                warn_has_thumb = true;
                break;
            }
            if current_row != row {
                current_row = row;
                xkb.push('\n');
            }
            let row_name = ROW_NAMES[usize::from(row)];
            let key_id = if row > 1 || col < 12 {
                format!("{row_name}{:0>2}", col + 1)
            } else if col == 12 {
                "BKSL".to_owned()
            } else {
                format!("{row_name}{:0>2}", col)
            };
            xkb.push_str(&format_xkb_item(&key_id, key));
        }
        xkb.push_str("};\n```");
        if warn_has_thumb {
            xkb.push_str(&format!("\nWarning: `{ll_name}` has thumb keys that are excluded in the xkb:\n"));
            xkb.push_str(&sorted_ll.iter()
                .skip_while(|(_, pos)| pos.row < 3)
                .map(|(key, _)| key)
                .join(", "));
        }
        xkb
    }

    fn usage<'a>(&self) -> &'a str {
        "xkb <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "generate an xkb symbol file for a layout"
    }

    fn cmini_channel_only(&self) -> bool {
        true
    }
}

fn format_xkb_item(key_id: &str, key: char) -> String {
    let (lowercase, uppercase) = match NAME_MAP.get(&key) {
        None => (Cow::Owned(key.to_string()), Cow::Owned(key.to_uppercase().to_string())),
        Some((unshifted, shifted)) => (Cow::Borrowed(*unshifted), Cow::Borrowed(*shifted)),
    };
    format_xkb_item_with_names(key_id, lowercase, uppercase)
}

fn format_xkb_item_with_names(key_id: &str, lowercase: Cow<str>, uppercase: Cow<str>) -> String {
    format!("    key <{key_id}>\t {{[\t  {lowercase},  {uppercase}\t ]}};\n")
}