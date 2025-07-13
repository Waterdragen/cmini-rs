use std::cmp::Reverse;
use fxhash::FxHashMap;
use crate::{BoundedResponse, Message};
use crate::prelude::Commandable;
use serenity::all::colours::roles::TEAL;
use serenity::all::GuildId;
use serenity::futures::StreamExt;
use serenity::model::colour::Colour;
use crate::message::BOT_CLIENT_HTTP;

const LAYOUT_ROLE_COLOR: Colour = TEAL;
const QWERTY: &str = "QWERTY";
const AKL_ID: GuildId = GuildId::new(807843650717483049);

pub struct Command;

impl Command {
    pub async fn exec(&self) -> String {
        let Some(http) = BOT_CLIENT_HTTP.get() else {
            return "Cannot find akl server".to_owned();
        };
        let mut members = AKL_ID.members_iter(&http).boxed();
        // let Ok(members) = AKL_ID.members_iter(&http) else {
        //     return "Cannot find akl members".to_owned();
        // };
        let Ok(roles) = AKL_ID.roles(&http).await else {
            return "Cannot find akl roles".to_owned();
        };
        let mut layout_roles = roles.iter()
            .filter_map(|(id, role)| {
                (role.colour == LAYOUT_ROLE_COLOR || role.name == QWERTY)
                    .then_some((*id, RoleWrapper::new(&role.name)))
            })
            .collect::<FxHashMap<_, _>>();
        while let Some(member) = members.next().await {
            let Ok(member) = member else { return "Cannot find akl members".to_owned() };
            for id in member.roles.iter() {
                if let Some(role) = layout_roles.get_mut(id) {
                    role.counter += 1;
                }
            }
        }
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
}

impl Commandable for Command {
    fn exec(&self, _: &Message) -> String {
        unimplemented!()
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