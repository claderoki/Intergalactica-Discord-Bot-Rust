use std::time::Duration;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::modules::{pigeon::{models::pigeon::Pigeon, repository::pigeon::PigeonRepository}, shared::{models::human::Human, repository::human::HumanRepository}};

trait Economy {
    fn pay(&mut self, cost: i32);
    fn has_enough_gold(&self, cost: i32) -> bool;
}

impl Economy for Human {
    fn pay(&mut self, cost: i32) {
        self.gold -= cost
    }

    fn has_enough_gold(&self, cost: i32) -> bool {
        self.gold >= cost
    }
}

trait PigeonUtils {
    fn get_pigeon(&self) -> Option<Pigeon>;
    fn has_pigeon(&self) -> bool;
    fn create_pigeon(&self, name: &str) -> Result<Pigeon, &'static str>;
}

impl PigeonUtils for Human {
    fn get_pigeon(&self) -> Option<Pigeon> {
        PigeonRepository::get_active(self.id).ok()
    }

    fn has_pigeon(&self) -> bool {
        //TODO: create a PigeonRepository method for this (to avoid needlessly selecting the entire pigeon.)
        self.get_pigeon().is_some()
    }

    fn create_pigeon(&self, name: &str) -> Result<Pigeon, &'static str> {
        PigeonRepository::create(self.id, name)
    }
}

trait HumanUtils {
    fn get_human(&self) -> Option<Human>;
}

impl HumanUtils for User {
    fn get_human(&self) -> Option<Human> {
        HumanRepository::get_or_create(*self.id.as_u64()).ok()
    }
}

impl HumanUtils for UserId {
    fn get_human(&self) -> Option<Human> {
        HumanRepository::get_or_create(*self.as_u64()).ok()
    }
}

async fn ask_pigeon_name(msg: &Message, ctx: &Context) -> Result<String, &'static str> {
    let _ = msg.reply(ctx, "What will you name your pigeon?").await;
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

    let mut human = msg.author.id.get_human().ok_or("Could not create a human")?;
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
    Ok(())
}
