use serenity::model::id::{ChannelId, MessageId};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::OnceLock;
use tokio::sync::mpsc::Sender;

pub static RESTART_FLAG: OnceLock<Sender<()>> = OnceLock::new();

const PATH: &str = "./restarted_by.txt";

pub fn try_log_channel_id(message_id: MessageId, channel_id: ChannelId) {
    impl_try_log_channel_id(message_id, channel_id);
}

fn impl_try_log_channel_id(message_id: MessageId, channel_id: ChannelId) -> Option<()> {
    let mut file = File::create(PATH).ok()?;
    writeln!(file, "{}", message_id.get()).ok()?;
    writeln!(file, "{}", channel_id.get()).ok()?;
    Some(())
}

pub fn try_get_channel_id() -> Option<(MessageId, ChannelId)> {
    let reader = BufReader::new(File::open(PATH).ok()?);
    let mut lines = reader.lines();
    let message_id = MessageId::new(lines.next()?.ok()?.parse::<u64>().ok()?);
    let channel_id = ChannelId::new(lines.next()?.ok()?.parse::<u64>().ok()?);
    Some((message_id, channel_id))
}
