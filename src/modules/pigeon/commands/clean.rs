use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::{
    modules::pigeon::{
        helpers::{
            utils::{PigeonWinningsBuilder},
            validation::PigeonValidation,
            winning_message::winnings_message,
        },
        models::pigeon::PigeonStatus,
        repository::pigeon::PigeonRepository,
    },
};

#[command("clean")]
#[description("Clean your pigeon.")]
pub async fn clean(ctx: &Context, msg: &Message) -> CommandResult {
    let cost = 20;
    let increase = 20;

    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .gold_needed(cost)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    let winnings = PigeonWinningsBuilder::new()
        .cleanliness(increase)
        .gold(-cost)
        .build();

    winnings_message(
        ctx,
        msg,
        &winnings,
        "Your pigeon leaves dirty food prints on the floor! You decide to give it a bath.".into(),
    )
    .await?;
    PigeonRepository::update_winnings(human_id, &winnings)?;
    Ok(())
}
