use serenity::framework::standard::{macros::{command, group}, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("This is a description.")]
async fn buy(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "This is a sub function!").await?;

    Ok(())
}

#[group]
#[prefix("pigeon")]
#[commands(buy)]
struct Pigeon;
