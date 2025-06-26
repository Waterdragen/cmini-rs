use crate::prelude::*;
use crate::util::memory::ADMINS;
use crate::util::parser::get_args;

pub struct Command;

impl Command {
    pub fn exec(&self, arg: &str, id: u64, switch: Arc<RwLock<bool>>) -> String {
        if !ADMINS.contains(id) {
            return "Unauthorized".to_owned();
        }
        let args = get_args(arg);
        if let Some(arg) = args.first() {
            match *arg {
                "on" | "enable" | "true" => {
                    let mut mode = switch.write();
                    *mode = true;
                    return "Maintenance mode enabled".to_owned();
                }
                "off" | "disable" | "false" => {
                    let mut mode = switch.write();
                    *mode = false;
                    return "Maintenance mode disabled".to_owned();
                }
                _ => {}
            }
        }
        {
            let mode = *switch.read();
            format!("Maintenance mode: {mode}")
        }
    }
}

impl Commandable for Command {
    fn exec(&self, _: &Message) -> String {
        unimplemented!()
    }

    fn usage<'a>(&self) -> &'a str {
        "[maintenance | 1984] [on | off]"
    }

    fn desc<'a>(&self) -> &'a str {
        "enable or disable maintenance mode"
    }

    fn mods_only(&self) -> bool {
        true
    }
}