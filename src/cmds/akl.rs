use std::cmp::Reverse;
use fxhash::FxHashMap;
use crate::{BoundedResponse, Message};
use crate::prelude::Commandable;
use serenity::all::colours::roles::TEAL;
use serenity::model::colour::Colour;

const LAYOUT_ROLE_COLOR: Colour = TEAL;
const QWERTY: &str = "QWERTY";

pub struct Command;

impl Commandable for Command {
    fn exec(&self, msg: &Message) -> String {
        let guild = match msg.guild(msg.context.as_ref()) {
            None => return "Cannot find akl server".to_owned(),
            Some(guild) => guild,
        };
        let mut layout_roles = guild.roles.iter()
            .filter_map(|(id, role)| {
                (role.colour == LAYOUT_ROLE_COLOR || role.name == QWERTY)
                    .then_some((*id, RoleWrapper::new(&role.name)))
            })
            .collect::<FxHashMap<_, _>>();
        if guild.members.is_empty() {
            return format!("There are {} members in the guild, cannot get member information", guild.member_count);
        }
        guild.members.values()
            .flat_map(|member| &member.roles)
            .for_each(|id| {
                if let Some(role) = layout_roles.get_mut(id) {
                    role.counter += 1;
                }
            });
        let mut layout_roles = layout_roles.into_values().collect::<Vec<_>>();
        layout_roles.sort_unstable_by_key(|role| Reverse(role.counter));

        let mut output = BoundedResponse::from("```\n\
                               --- AKL STATS ---\n\
                               Layout role count:\n".to_owned()).reserve(100);
        for role in layout_roles.iter() {
            let (name, count) = (role.name, role.counter);
            let _ = output.push_str(&format!("    {name:<15} ({count} users)\n"));
        }
        let mut output = output.finish();
        output.push_str("```");
        output
    }

    fn usage<'a>(&self) -> &'a str {
        "akl"
    }

    fn desc<'a>(&self) -> &'a str {
        "view the akl layout role stats"
    }
}

struct RoleWrapper<'a> {
    name: &'a str,
    counter: usize,
}
impl<'a> RoleWrapper<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name, counter: 0 }
    }
}