use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message as DiscordMessage;
use serenity::model::gateway::{GatewayIntents, Ready};
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::signal;
use tokio::sync::mpsc::channel;

use cmini_rs::cmds;
use cmini_rs::consts::TRIGGERS;
use cmini_rs::cron_job::daily_cron_job;
use cmini_rs::error::{BotError, Signal};
use cmini_rs::message::BOT_CONTEXT;
use cmini_rs::prelude::*;
use cmini_rs::util::memory::{self, ADMINS};
use cmini_rs::util::restart::RESTART_FLAG;
use cmini_rs::util::{restart, validate_json};

static MAINTENANCE_FLAG: AtomicBool = AtomicBool::new(false);

fn maintenance_check(id: u64) -> bool {
    let active = MAINTENANCE_FLAG.load(Ordering::Relaxed);
    !active || ADMINS.contains(id)
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: DiscordMessage) {
        let msg = Message::from_msg_ctx(&msg, &ctx);

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
        let in_cmini_channel = msg.in_cmini_channel();

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
                cmds::akl::Command.exec(&msg).await
            }
            "maintenance" | "1984" => {
                cmds::maintenance::Command.exec(&msg, &MAINTENANCE_FLAG).await
            }
            "question" => {
                cmds::question::Command.exec().await
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
            false => { let _ = msg.reply_ping(&ctx.http, &response).await; }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        // Not cmini's first run?
        if let Some((message_id, channel_id)) = restart::try_get_channel_id() {
            if let Ok(msg) = channel_id.message(&ctx.http, message_id).await {
                let _ = msg.reply(&ctx.http, "Cmini successfully restarted!").await;
            }
        }

        println!("{} is connected!", ready.user.name);
        BOT_CONTEXT.set(ctx).unwrap_or_else(|_| panic!("Cannot set bot context"));
    }
}

async fn start_discord_bot() {
    let token = fs::read_to_string("token.txt")
        .expect("Expected a token in the token.txt file");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILDS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(err) = client.start_autosharded().await {
        println!("Client error: {:?}", err);
    }
}

#[tokio::main]
async fn main() -> Result<(), BotError> {
    validate_json();

    let args: Vec<String> = std::env::args().collect();
    // FIXME: write a proper cmd line parser if more flags are used in the future
    let always_cache = args.iter().any(|s| s == "-y");

    tokio::spawn(daily_cron_job());
    tokio::spawn(start_discord_bot());

    let (tx, mut rx) = channel(1);
    let _ = RESTART_FLAG.set(tx);

    let signal = tokio::select! {
        _ = rx.recv() => Signal::AdminRestart,
        _ = signal::ctrl_c() => Signal::ForceEnd,
    };
    if always_cache {
        println!("Caching cmini...");
        memory::sync_data_local();
    } else {
        println!("Aborting cmini. Warning: cmini might have unsaved changes!");
        print!("Sync data? [Y/n]: ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "n" {
            memory::sync_data_local()
        }
    }
    match signal {
        Signal::AdminRestart => Ok(()),  // shell script auto-restarts on OK
        Signal::ForceEnd => Err(BotError::CtrlC),
    }
}