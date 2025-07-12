use crate::util::{memory, shell};
use std::time::Duration;
use time::{UtcDateTime, Time};
use tokio::time::{sleep, interval};

const ONE_DAY: Duration = Duration::from_secs(86400);

async fn wait_until_utc_midnight() {
    let now = UtcDateTime::now();
    let dur_until_midnight = Duration::try_from(Time::MAX - now.time()).unwrap();  // Time::MAX - now is always non-negative, can safely cast back to StdDuration
    sleep(dur_until_midnight).await;
}

pub async fn daily_cron_job() -> ! {
    wait_until_utc_midnight().await;

    let mut interval = interval(ONE_DAY);
    loop {
        interval.tick().await;
        memory::sync_data_local();
        let _ = shell::sync_github();

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