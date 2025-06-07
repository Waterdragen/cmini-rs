use crate::util::Commandable;
use crate::cmds::COMMANDS;
use crate::util::Message;

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let cmd_name = &msg.arg;
        if !cmd_name.is_empty() {
            let cmd = match COMMANDS.get(cmd_name) {
                Some(cmd) => cmd,
                None => return format!("Unknown command {cmd_name}"),
            };

            let mut cmds = COMMANDS.keys().collect::<Vec<_>>();
            cmds.sort();

            format!(
                "Help page for `{cmd_name}`:\
                 ```\n\
                 {}\n\
                 {}\n\
                 ```",
                cmd.usage(),
                cmd.desc(),
            )
        } else {
            let mut s = String::from(
                "Usage: `!cmini (command) [args]`\n\
                    ```");
            let mut cmds = COMMANDS.keys().collect::<Vec<_>>();
            cmds.sort();

            two_column_display(&mut s, &cmds);
            s.push_str("```");
            s
        }
    }

    fn usage<'a>(&self) -> &'a str {
        "help [command name]"
    }

    fn desc<'a>(&self) -> &'a str {
        "view list of commands or usage of a command"
    }
}

fn two_column_display(s: &mut String, cmds: &[&String]) {
    const ALIGN: usize = 16;
    let len = cmds.len();
    let rem_count = len % 2;
    let mid_pt = len / 2 + rem_count;
    let l_cmds = &cmds[..mid_pt];
    let r_cmds = &cmds[mid_pt..];
    for (l_cmd, r_cmd) in l_cmds.iter().zip(r_cmds) {
        s.push_str(l_cmd);
        s.push_str(&" ".repeat(ALIGN - l_cmd.chars().count()));
        s.push_str(r_cmd);
        s.push_str(&" ".repeat(ALIGN - r_cmd.chars().count()));
        s.push('\n');
    }
}