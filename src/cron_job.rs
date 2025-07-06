use crate::util::{memory, shell};
use std::time::Duration;
use std::io::Write;
use tokio::time::interval;

fn sync_data_github() -> std::io::Result<()> {
    let outputs = [
        shell::git_add_jsons()?,
        shell::git_commit_sync_data()?,
        shell::git_pull()?,
        shell::git_push()?,
    ];
    for output in outputs {
        std::io::stdout().write_all(&output.stdout)?;
        std::io::stderr().write_all(&output.stderr)?;
    }
    Ok(())
}

pub async fn daily_cron_job() {
    let mut interval = interval(Duration::from_secs(86400));
    interval.tick().await;  // ticks immediately

    loop {
        interval.tick().await;
        memory::sync_data_local();
        let _ = sync_data_github();

        // You may enable this code to get a message from the bot

        // use serenity::model::id::UserId;
        // use crate::message::BOT_CONTEXT;
        // use crate::util::memory::ADMINS;
        // let http = &BOT_CONTEXT.get().unwrap().http;
        // let dm_channel = UserId(ADMINS.owner_id()).create_dm_channel(http).await.unwrap();
        // let _ = match shell::git_push() {
        //     Ok(_) => dm_channel.say(http, "Successfully synced data").await,
        //     Err(err) => dm_channel.say(http, err.to_string()).await,
        // };
    }
}