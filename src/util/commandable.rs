use crate::message::Message;
use crate::util::memory::ADMINS;

pub trait Commandable: Send + Sync {
    fn exec(&self, msg: &Message) -> String;
    fn usage<'a>(&self) -> &'a str;
    fn desc<'a>(&self) -> &'a str;

    fn init(self) -> Box<dyn Commandable> where Self: Sized + 'static {
        Box::new(self)
    }

    fn help(&self) -> String {
        let mut help_message = "```\n".to_owned();
        help_message.push_str(self.usage());
        help_message.push('\n');
        help_message.push_str(self.desc());
        help_message.push_str("```");
        help_message
    }

    fn cmini_channel_only(&self) -> bool {
        false
    }

    fn public_channel_only(&self) -> bool {
        false
    }

    fn mods_only(&self) -> bool {
        false
    }

    fn try_exec(&self, msg: &Message) -> String {
        if self.mods_only() && !ADMINS.contains(msg.id) {
            return "Unauthorized".to_owned();
        }
        if self.public_channel_only() && msg.is_private() {
            return "Use this command in a public channel".to_owned();
        }
        self.exec(msg)
    }
}