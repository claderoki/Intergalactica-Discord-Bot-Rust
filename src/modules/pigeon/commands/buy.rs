use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("This is a description.")]
pub async fn buy(ctx: &Context, msg: &Message) -> CommandResult {
    /*
     * 1. Check if the user already has an active pigeon.
     * 2. Check if they have enough gold (50g) for this action.
     * WAITER ??
     */
    msg.reply(&ctx.http, "This is a sub function!").await?;
    Ok(())
}
