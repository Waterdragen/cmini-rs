extern crate core;

mod util;
mod test;
mod cmds;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::gateway::{Ready, GatewayIntents};
use serenity::model::channel::Message;
use std::fs;
use tokio::time::{self, Duration};

use crate::util::consts::{CMINI_CHANNEL, TRIGGERS};

fn split_action_args(is_dm: bool, words: &Vec<&str>) -> (String, String) {
    match is_dm {
        true => (words.get(0).unwrap_or(&"").to_lowercase(), words[1..].join(" ")),
        false => (words.get(1).unwrap_or(&"").to_lowercase(), words[2..].join(" "))
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore other bots and empty messages
        if msg.author.bot || msg.content.is_empty() {
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
        let (action, args) = split_action_args(is_dm, &words);
        let mut cmini_channel_only = false;
        let response = match cmds::get_cmd(&action) {
            Some(cmd) => { cmini_channel_only = cmd.cmini_channel_only(); cmd.exec(&args) },
            None => format!("Error: {} is not an available command", &action),
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

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !args.is_empty() && args.contains(&String::from("cache")) {
        util::cache::cache_main();
        return;
    }

    tokio::spawn(daily_cron_job());

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