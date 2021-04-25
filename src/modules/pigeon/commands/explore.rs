use std::time::Duration;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{
    builder::{CreateEmbed},
    framework::standard::{macros::command, CommandResult},
};

use crate::modules::{pigeon::helpers::utils::PigeonUtils, shared::{helpers::utils::{Economy, HumanUtils}, repository::{human::HumanRepository, item::HumanItemRepository}}};

#[command("spacexplore")]
#[description("")]
pub async fn spacexplore(ctx: &Context, msg: &Message) -> CommandResult {

}

enum Planet {
    Mars = 1,
}

struct SpaceRetrieval {

}



#[command("retrieve")]
#[description("")]
pub async fn retrieve(ctx: &Context, msg: &Message) -> CommandResult {

}
