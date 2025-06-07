#![allow(unused)]

mod cmds;
mod test;
mod util;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::gateway::{Ready, GatewayIntents};
use serenity::model::channel::Message;
use std::fs;
use std::io::Write;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use tokio::time::{self, Duration};
use tokio::signal;

use crate::util::consts::{CMINI_CHANNEL, TRIGGERS, ADMINS};

static MAINTENANCE_MODE: Lazy<Arc<RwLock<bool>>> = Lazy::new(|| Arc::new(RwLock::new(false)));

fn maintenance_check(id: u64) -> bool {
    let mode = MAINTENANCE_MODE.read().unwrap();
    if *mode {
        return ADMINS.contains(&id);
    }
    !*mode
}

fn split_action_args(is_dm: bool, words: &[&str]) -> (String, String) {
    let (first, rest) = match is_dm {
        true => (0, 1..),
        false => (1, 2..),
    };
    (
        words.get(first).unwrap_or(&"").to_lowercase(),
        match words.get(rest) {
            None => String::new(),
            Some(s) => s.join(" "),
        }
    )
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore other bots and empty messages
        if msg.author.bot || msg.content.is_empty() {
            return;
        }

        let id = *msg.author.id.as_u64();
        if !maintenance_check(id) {
            return;
        }

        // Is in a DM?
        let is_dm = msg.is_private();

        // Restricted command?
        let in_cmini_channel = msg.channel_id == CMINI_CHANNEL;

        let first_word = msg.content.split_whitespace().next().unwrap_or_default();
        if !is_dm && !TRIGGERS.contains(&first_word) {
            return;
        }

        let words: Vec<&str> = msg.content.split_whitespace().collect();
        let (action, arg) = split_action_args(is_dm, &words);
        let wrap_msg = util::Message::new(arg, id, &msg);

        let mut cmini_channel_only = false;
        let response = match action.as_ref() {
            "" => {
                "Try `!cmini help`".to_owned()
            }
            "akl" => {
                "Not yet implemented".to_owned()
            },
            "maintenance" | "1984" => {
                cmds::maintenance::Command.exec(&wrap_msg.arg, id, Arc::clone(&MAINTENANCE_MODE))
            }
            _ => {
                match cmds::get_cmd(&action) {
                    Some(cmd) => { cmini_channel_only = cmd.cmini_channel_only(); cmd.exec(&wrap_msg) },
                    None => format!("Error: {} is not an available command", &action),
                }
            }
        };

        // DM required?
        match !in_cmini_channel && cmini_channel_only {
            true => if let Ok(dm_channel) = msg.author.create_dm_channel(&ctx.http).await {
                let _ = dm_channel.say(&ctx.http, &response).await;
            }
            false => { let _ = msg.channel_id.say(&ctx.http, &response).await; }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn sync_data() {
    util::cache::cache_main();
    util::memory::sync_layouts();
}

async fn daily_cron_job() {
    let mut interval = time::interval(Duration::from_secs(86400));
    interval.tick().await;  // ticks immediately

    loop {
        interval.tick().await;
        sync_data();
    }
}

async fn start_discord_bot() {
    let token = fs::read_to_string("token.txt")
        .expect("Expected a token in the token.txt file");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !args.is_empty() && args.contains(&String::from("cache")) {
        util::cache::cache_main();
        return;
    }

    tokio::spawn(daily_cron_job());
    tokio::spawn(start_discord_bot());

    let _ = signal::ctrl_c().await;
    let mut input = String::new();
    println!("Aborting cmini. Warning: cmini might have unsaved changes!");
    print!("Sync data? [Y/n]: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() != "n" {
        sync_data()
    }
}