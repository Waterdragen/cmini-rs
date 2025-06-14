#![allow(unused)]
#![warn(unused_variables)]
#![warn(unused_mut)]
#![warn(unused_imports)]
#![warn(unused_must_use)]

mod cmds;
mod test;
mod util;

use once_cell::sync::Lazy;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message as DiscordMessage;
use serenity::model::gateway::{GatewayIntents, Ready};
use std::io::Write;
use std::sync::{Arc, RwLock};
use std::fs;
use tokio::signal;
use tokio::time::{self, Duration};

use crate::util::consts::{ADMINS, CMINI_CHANNEL, TRIGGERS};
use crate::util::{validate_json, Message};

static MAINTENANCE_MODE: Lazy<Arc<RwLock<bool>>> = Lazy::new(|| Arc::new(RwLock::new(false)));

fn maintenance_check(id: u64) -> bool {
    let mode = MAINTENANCE_MODE.read().unwrap();
    if *mode {
        return ADMINS.contains(&id);
    }
    !*mode
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: DiscordMessage) {
        let msg = Message::from(&msg);

        // Ignore other bots and empty messages
        if msg.author.bot || msg.content.is_empty() {
            return;
        }

        let id = msg.id;
        if !maintenance_check(id) {
            return;
        }

        // Is in a DM?
        let is_dm = msg.is_private();

        // Restricted command?
        let in_cmini_channel = msg.channel_id == CMINI_CHANNEL;

        let trigger = msg.trigger;
        if !is_dm && !TRIGGERS.contains(&trigger) {
            return;
        }

        let action = msg.action;

        let mut cmini_channel_only = false;
        let response = match action {
            "" => {
                "Try `!cmini help`".to_owned()
            }
            "akl" => {
                "Not yet implemented".to_owned()
            },
            "maintenance" | "1984" => {
                cmds::maintenance::Command.exec(msg.arg, id, Arc::clone(&MAINTENANCE_MODE))
            }
            _ => {
                match cmds::get_cmd(action) {
                    Some(cmd) => {
                        cmini_channel_only = cmd.cmini_channel_only();
                        cmd.try_exec(&msg)
                    },
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
    util::memory::sync_data();
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
    validate_json();

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