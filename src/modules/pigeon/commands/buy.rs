use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::modules::shared::repository::human::get_or_create_human;

#[command]
#[description("This is a description.")]
pub async fn buy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let name = args.rest();

    let mut human = get_or_create_human(*msg.author.id.as_u64())?;
    let s = human.buy_pigeon(name);
    msg.reply(&ctx.http, s).await?;

    Ok(())
}
