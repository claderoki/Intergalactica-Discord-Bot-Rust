use chrono::Duration;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::modules::pigeon::helpers::increase_stats::increase;
use crate::modules::pigeon::helpers::increase_stats::IncreaseParams;

#[command("clean")]
#[description("Clean your pigeon.")]
pub async fn clean(ctx: &Context, msg: &Message) -> CommandResult {
    let increase_params = IncreaseParams::new(
        15,
        25,
        "pigeon_clean",
        Duration::minutes(45),
        "cleanliness",
        "Your pigeon leaves dirty food prints on the floor! You decide to give it a bath.",
    );
    increase(&ctx, &msg, increase_params).await?;
    Ok(())
}
