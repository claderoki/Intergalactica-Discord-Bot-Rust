use std::time::Duration;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{
    discord_helpers::embed_utils::EmbedExtension,
    modules::{
        pigeon::repository::pigeon::PigeonRepository,
        shared::{
            helpers::utils::HumanUtils,
            repository::{human::HumanRepository},
        },
    },
};

// trait Validator {
//     fn validate(&self, user_id: UserId) -> Result<(), &'static str>;
// }

// struct CommandValidation {
//     goldNeeded: i32,
// }

// struct PigeonValidation {
//     gold_needed: i32,
//     needs_active_pigeon: bool,
//     needs_pigeon_available: bool
// }

// impl Validator for PigeonValidation {
//     fn validate(&self, user_id: UserId) -> Result<(), &'static str> {
//         let human_id = user_id.get_human_id().ok_or("Could not get human id.")?;

//         if self.gold_needed > 0 {
//             let has_gold = HumanRepository::has_gold(human_id, self.gold_needed)?;
//             if !has_gold {
//                 return Err("You do not have enough gold to perform this action.");
//             }
//         }

//         if self.needs_active_pigeon {
//             let has_pigeon = PigeonRepository::has_active(human_id)?;
//             if has_pigeon {
//                 return Err("You already have a pigeon!");
//             }
//         }

//         Ok(())
//     }
// }

async fn ask_pigeon_name(msg: &Message, ctx: &Context) -> Result<String, &'static str> {
    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| e.normal_embed("What will you name your pigeon?"))
        })
        .await;

    let reply = &msg
        .author
        .await_reply(&ctx)
        .timeout(Duration::from_secs(60))
        .await
        .ok_or("No name given")?;

    Ok(String::from(reply.content.as_str()))
}

#[command("buy")]
#[description("Buy a pigeon.")]
pub async fn buy(ctx: &Context, msg: &Message) -> CommandResult {
    let cost = 50;

    let human_id = msg
        .author
        .get_human_id()
        .ok_or("Could not create a human")?;

    let has_pigeon = PigeonRepository::has_active(human_id)?;
    if has_pigeon {
        return Err(format!("You already have a pigeon!").into());
    }

    let has_gold = HumanRepository::has_gold(human_id, cost)?;
    if !has_gold {
        return Err(format!("You do not have enough gold to perform this action.").into());
    }

    let name = ask_pigeon_name(&msg, &ctx).await?;
    PigeonRepository::create(human_id, &name)?;

    HumanRepository::spend_gold(human_id, cost)?;

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| e.priced_embed("You just purchased a pigeon!", cost))
        })
        .await;

    Ok(())
}
