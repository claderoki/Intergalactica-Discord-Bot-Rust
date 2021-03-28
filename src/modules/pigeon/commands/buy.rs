use serenity::framework::standard::{macros::{command}, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("This is a description.")]
pub async fn buy(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "This is a sub function!").await?;
    Ok(())
}