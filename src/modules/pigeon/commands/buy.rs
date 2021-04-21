use std::{convert::TryFrom, convert::TryInto, time::Duration};

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::modules::{
    pigeon::repository::pigeon::PigeonRepository,
    shared::{models::human::Human, repository::human::HumanRepository},
};

struct CommandContext {
    pub human: Human,
    // pub ctx: &'fut Context,
    // pub msg: &'fut Message,
    pub name: String,
    pub cost: i32,
}

impl CommandContext {
    pub fn new(msg: &Message, name: &'static str, cost: i32) -> Result<Self, &'static str> {
        let human = HumanRepository::get_or_create(*msg.author.id.as_u64())?;
        Ok(Self {
            human,
            name: String::from(name),
            cost,
        })
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if self.human.gold < self.cost {
            return Err("Not enough gold.");
        }
        Ok(())
    }

    pub fn finish(mut self) {
        self.human.gold -= self.cost;
        HumanRepository::save(self.human).ok();
    }
}

struct MessageWaiter<'fut> {
    pub ctx: &'fut Context,
    pub msg: &'fut Message,
    // pub start_prompt: Option<String>,
    // pub end_prompt: Option<String>
}

impl MessageWaiter<'_> {

    pub async fn wait<T: TryFrom<String>>(&self, timeout: u64, prompt: &str) -> Result<T, &'static str> {
        let reply = &self
            .msg
            .author
            .await_reply(&self.ctx)
            .timeout(Duration::from_secs(timeout))
            .await;

        match reply {
            Some(message) => {
                let converted: std::result::Result<T, _> =
                    String::from(message.content.as_str()).try_into();
                return match converted {
                    Ok(value) => Ok(value),
                    Err(_) => Err("Failed to convert"),
                };
            }
            None => {
                return Err("Timed out".into());
            }
        };
    }
}

#[command("buy")]
#[description("This is a description.")]
pub async fn buy(ctx: &Context, msg: &Message) -> CommandResult {
    let waiter = MessageWaiter{ctx, msg};
    let name = waiter.wait::<String>(60, "What would you like?").await;

    Ok(())
}

// #[command("buy")]
// #[description("This is a description.")]
// pub async fn buy(ctx: &Context, msg: &Message) -> CommandResult {
//     let cmd_ctx = CommandContext::new(msg, "pigeon_buy", 50)?;
//     cmd_ctx.validate()?;

//     match PigeonRepository::get_active(cmd_ctx.human.id) {
//         Ok(pigeon) => {
//             return Err(format!(
//                 "You already have a lovely pigeon named **{}**.",
//                 pigeon.name
//             )
//             .into());
//         }
//         Err(_) => {
//             let _ = msg.reply(ctx, "What will you name your pigeon?").await;
//             let reply = &msg
//                 .author
//                 .await_reply(&ctx)
//                 .timeout(Duration::from_secs(60))
//                 .await;

//             let name = match reply {
//                 Some(name) => &name.content,
//                 None => {
//                     return Err("No name given.".into());
//                 }
//             };

//             match PigeonRepository::create(cmd_ctx.human.id, name.as_str()) {
//                 Ok(_) => {
//                     msg.reply(
//                         &ctx.http,
//                         format!(
//                             "You just bought yourself a new pigeon (**-{}**)",
//                             cmd_ctx.cost
//                         ),
//                     )
//                     .await?;
//                 }
//                 Err(_) => {}
//             }
//         }
//     }

//     cmd_ctx.finish();
//     Ok(())
// }
