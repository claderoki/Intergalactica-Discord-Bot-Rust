use std::time::Duration;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{
    builder::{CreateEmbed},
    framework::standard::{macros::command, CommandResult},
};

use crate::modules::{pigeon::helpers::utils::PigeonUtils, shared::{helpers::utils::{Economy, HumanUtils}, repository::{human::HumanRepository, item::HumanItemRepository}}};

trait EmbedExtension {
    fn priced_embed(&mut self, text: &str, cost: i32) -> &mut Self;
    fn normal_embed(&mut self, text: &str) -> &mut Self;
    fn error_embed(&mut self, text: &str) -> &mut Self;
}

impl EmbedExtension for CreateEmbed {
    fn priced_embed(&mut self, text: &str, cost: i32) -> &mut Self {
        self.normal_embed(text)
    }

    fn normal_embed(&mut self, text: &str) -> &mut Self {
        self.color(serenity::utils::Color::from_rgb(242, 181, 37))
            .description(text)
    }

    fn error_embed(&mut self, text: &str) -> &mut Self {
        self.color(serenity::utils::Color::RED)
            .description(text)
    }
}

async fn ask_pigeon_name(msg: &Message, ctx: &Context) -> Result<String, &'static str> {
    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| e.normal_embed("What will you name your pigeon?"))
        })
        .await;

    let reply = &msg
        .author
        .await_reply(&ctx)
        .timeout(Duration::from_secs(60))
        .await
        .ok_or("No name given")?;

    Ok(String::from(reply.content.as_str()))
}

#[command("buy")]
#[description("Buy a pigeon.")]
pub async fn buy(ctx: &Context, msg: &Message) -> CommandResult {
    let cost = 50;

    let mut human = msg.author.get_human().ok_or("Could not create a human")?;

    let has = HumanItemRepository::has_item("milky_way", human.id, 1);
    println!("{:?}", has);

    if !human.has_enough_gold(cost) {
        return Err(format!("You do not have enough gold to perform this action.").into());
    }

    if human.has_pigeon() {
        return Err(format!("You already have a pigeon!").into());
    }

    let name = ask_pigeon_name(&msg, &ctx).await?;
    human.create_pigeon(name.as_str())?;
    human.pay(cost);
    HumanRepository::save(human)?;

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| e.priced_embed("You just purchased a pigeon!", cost))
        })
        .await;

    Ok(())
}
