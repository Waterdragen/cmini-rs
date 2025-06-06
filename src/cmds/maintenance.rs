use std::sync::{Arc, RwLock};
use crate::util::consts::ADMINS;
use crate::util::parser::get_args;

pub struct Command;

impl Command {
    pub fn exec(&self, arg: &str, id: u64, switch: Arc<RwLock<bool>>) -> String {
        if !ADMINS.contains(&id) {
            return "Unauthorized".to_owned();
        }
        let args = get_args(arg);
        if let Some(arg) = args.first() {
            match *arg {
                "on" | "enable" | "true" => {
                    let mut mode = switch.write().unwrap();
                    *mode = true;
                    return "Maintenance mode enabled".to_owned();
                }
                "off" | "disable" | "false" => {
                    let mut mode = switch.write().unwrap();
                    *mode = false;
                    return "Maintenance mode disabled".to_owned();
                }
                _ => {}
            }
        }
        {
            let mode = switch.read().unwrap();
            format!("Maintenance mode: {mode}")
        }
    }
}