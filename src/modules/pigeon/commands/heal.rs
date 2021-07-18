use chrono::Duration;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::modules::pigeon::helpers::increase_stats::increase;
use crate::modules::pigeon::helpers::increase_stats::IncreaseParams;

#[command("heal")]
#[description("Heal your pigeon.")]
pub async fn heal(ctx: &Context, msg: &Message) -> CommandResult {
    let increase_params = IncreaseParams::new(
        15,
        25,
        "pigeon_heal",
        Duration::minutes(45),
        "health",
        "You give your pigeon some health. It's health is refilled!",
    );
    increase(&ctx, &msg, increase_params).await?;
    Ok(())
}
