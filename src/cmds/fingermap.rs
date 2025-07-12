use crate::message::Message;
use crate::prelude::Commandable;
use crate::util::memory::LAYOUTS;
use crate::util::layout::header;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        if msg.arg.is_empty() {
            return self.help();
        }
        let ll = &*LAYOUTS.find(msg.arg);
        let header = header(ll);
        let matrix = ll.matrix_str();
        let finger_matrix = ll.finger_matrix_str();
        format!("```\n\
                {header}\n\
                {matrix}\
                \n\
                {finger_matrix}\
                ```")
    }

    fn usage<'a>(&self) -> &'a str {
        "fingermap <layout_name>"
    }

    fn desc<'a>(&self) -> &'a str {
        "view the fingermap of a layout"
    }
}