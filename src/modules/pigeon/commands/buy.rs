use std::{io::Error, io::ErrorKind, time::Duration};

use serenity::framework::standard::{
    macros::{check, command},
    Args, CommandError, CommandOptions, CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::modules::{
    pigeon::repository::pigeon::{create_pigeon, get_active_pigeon},
    shared::{
        models::human::Human,
        repository::human::{get_or_create_human, save_human},
    },
};

#[command]
#[description("This is a description.")]
pub async fn buy(ctx: &Context, msg: &Message) -> CommandResult {
    let cost = 50;
    let mut human = get_or_create_human(*msg.author.id.as_u64())?;
    human.assert_gold(cost)?;

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
        Err(_) => {
            let _ = msg.reply(ctx, "What will you name your pigeon?").await;
            let reply = &msg
                .author
                .await_reply(&ctx)
                .timeout(Duration::from_secs(10))
                .await;

            let name = match reply {
                Some(name) => &name.content,
                None => {
                    msg.reply(&ctx.http, "No name given.").await?;
                    return Err("No name given.".into());
                }
            };

            match create_pigeon(human.id, name.as_str()) {
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
            }
        }
    }

    Ok(())
}
