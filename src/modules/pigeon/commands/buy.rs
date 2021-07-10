use std::time::Duration;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::repository::human::HumanRepository;

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

    let human_id = PigeonValidation::new()
        .gold_needed(cost)
        .needs_active_pigeon(false)
        .validate(&msg.author)?;

    let name = ask_pigeon_name(&msg, &ctx).await?;
    PigeonRepository::create(human_id, &name)?;

    HumanRepository::spend_gold(human_id, cost)?;

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| e.normal_embed("You just purchased a pigeon!"))
        })
        .await;

    Ok(())
}