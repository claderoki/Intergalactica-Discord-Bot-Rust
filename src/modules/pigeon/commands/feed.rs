use chrono::Duration;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::modules::pigeon::helpers::increase_stats::IncreaseParams;
use crate::modules::pigeon::helpers::increase_stats::increase;

#[command("feed")]
#[description("Feed your pigeon.")]
pub async fn feed(ctx: &Context, msg: &Message) -> CommandResult {
    let increase_params = IncreaseParams::new(
        15,
        25,
        "pigeon_feed",
        Duration::minutes(45),
        "food",
        "You give your pigeon some seeds. It's energy is refilled!",
    );
    increase(&ctx, &msg, increase_params).await?;
    Ok(())
}
