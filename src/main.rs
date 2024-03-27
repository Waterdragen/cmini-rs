use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::gateway::{Ready, GatewayIntents};
use serenity::model::channel::Message;
use std::fs;

mod util {
    pub mod consts;
}

use util::consts::{CMINI_CHANNEL, TRIGGERS};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore other bots
        if msg.author.bot {
            return;
        }

        // Empty message
        if msg.content.is_empty() {
            return;
        }

        // Is in a DM?
        let is_dm = msg.is_private();

        // Restricted command?
        let restricted = msg.channel_id != CMINI_CHANNEL && !is_dm;

        let first_word = msg.content.split_whitespace().next().unwrap_or_default();
        if !is_dm && !TRIGGERS.contains(&first_word) {
            return;
        }

        let words: Vec<&str> = msg.content.split_whitespace().collect();
        let (action, command) = if is_dm {
                (words.get(0).unwrap_or(&"").to_string(), words[1..].join(" "))
            }
            else {
                (words.get(1).unwrap_or(&"").to_string(), words[2..].join(" "))
            };

        let _ = msg.channel_id.say(&ctx.http, format!("Action: {action} Command: {command}")).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
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