use crate::util::memory::ADMINS;
use crate::util::parser::get_args;
use crate::util::restart::{self, RESTART_FLAG};
use crate::util::{memory, shell};
use crate::{Message, Commandable};
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Command;

impl Command {
    pub async fn exec(&self, msg: &Message<'_>, active: &AtomicBool) -> String {
        let (arg, id) = (msg.arg, msg.id);
        if !ADMINS.contains(id) {
            return "Unauthorized".to_owned();
        }
        let args = get_args(arg);
        if let Some(arg) = args.first() {
            match *arg {
                "on" | "enable" | "true" => {
                    active.store(true, Ordering::Relaxed);
                    return "Maintenance mode enabled".to_owned();
                }
                "off" | "disable" | "false" => {
                    active.store(false, Ordering::Relaxed);
                    return "Maintenance mode disabled".to_owned();
                }
                "restart" => {
                    let _ = msg.msg.reply_ping(&msg.context.http, "Cmini is restarting soon. Caching files...").await;
                    active.store(true, Ordering::Relaxed);
                    if let Ok(output) = shell::git_pull() {
                        let _ = std::io::stdout().write_all(&output.stdout);
                        let _ = std::io::stderr().write_all(&output.stderr);
                    }
                    memory::sync_data_local();
                    let _ = RESTART_FLAG.get()
                        .unwrap()  // main() has initialized the sender, and runs before commands
                        .send(()).await;
                    restart::try_log_channel_id(msg.msg.id, msg.channel_id);

                    active.store(false, Ordering::Relaxed);
                    return "Caching finished, restarting cmini...".to_owned();
                }
                _ => {}
            }
        }
        {
            let is_active = active.load(Ordering::Relaxed);
            format!("Maintenance mode: {is_active}")
        }
    }
}

impl Commandable for Command {
    fn exec(&self, _: &Message) -> String {
        unimplemented!()
    }

    fn usage<'a>(&self) -> &'a str {
        "[maintenance | 1984] [on | off | restart]"
    }

    fn desc<'a>(&self) -> &'a str {
        "enable or disable maintenance mode, or restart cmini"
    }

    fn mods_only(&self) -> bool {
        true
    }
}