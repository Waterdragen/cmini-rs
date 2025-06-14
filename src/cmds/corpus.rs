use crate::util::{Commandable, Message};
use crate::util::corpora::{set_user_corpus, CORPORA};

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let arg = &msg.arg;
        if arg.is_empty() {
            let mut s = "```\nList of Corpora\n".to_owned();
            let corpora = CORPORA.as_slice();
            for corpus in corpora {
                s.push_str("- ");
                s.push_str(corpus);
                s.push('\n');
            }
            s.push_str("```");
            return s;
        }
        match set_user_corpus(msg.id, arg) {
            Ok(_) => format!("Your corpus preference has been changed to `{}`.", arg.to_lowercase()),
            Err(_) => format!("The corpus `{arg}` doesn\'t exist."),
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "corpus <corpus_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "set your preferred corpus"
    }
}