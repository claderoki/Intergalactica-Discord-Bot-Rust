use chrono::Duration;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::modules::pigeon::helpers::increase_stats::increase;
use crate::modules::pigeon::helpers::increase_stats::IncreaseParams;

#[command("play")]
#[description("Play with your pigeon.")]
pub async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let increase_params = IncreaseParams::new(
        15,
        25,
        "pigeon_play",
        Duration::minutes(45),
        "happiness",
        "Your pigeon looks bored. You decide to play with it!",
    );
    increase(&ctx, &msg, increase_params).await?;

    Ok(())
}
