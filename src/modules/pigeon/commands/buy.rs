use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::modules::{
    pigeon::repository::pigeon::{create_pigeon, get_active_pigeon},
    shared::repository::human::{get_or_create_human, save_human},
};

#[command]
#[description("This is a description.")]
pub async fn buy(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let cost = 50;
    let name = args.single::<String>()?;

    if let Ok(mut human) = get_or_create_human(*msg.author.id.as_u64()) {
        if human.gold < cost {
            msg.reply(
                &ctx.http,
                format!("You need **{}** gold to purchase a pigeon.", cost),
            )
            .await?;
            return Ok(());
        }

        match get_active_pigeon(human.id) {
            Ok(pigeon) => {
                msg.reply(
                    &ctx.http,
                    format!(
                        "You already have a lovely pigeon named **{}**.",
                        pigeon.name
                    ),
                )
                .await?;
            }
            Err(_) => match create_pigeon(human.id, name.as_str()) {
                Ok(_) => {
                    human.gold -= cost;
                    save_human(human);
                    msg.reply(
                        &ctx.http,
                        format!("You just bought yourself a new pigeon (**-{}**)", cost),
                    )
                    .await?;
                }
                Err(e) => {
                    println!("ERROR, {}", e);
                }
            },
        }
    }

    Ok(())
}
