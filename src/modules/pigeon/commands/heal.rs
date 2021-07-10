use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::modules::pigeon::{
    helpers::{
        utils::PigeonWinningsBuilder, validation::PigeonValidation,
        winning_message::winnings_message,
    },
    models::pigeon::PigeonStatus,
    repository::pigeon::PigeonRepository,
};

#[command("heal")]
#[description("Heal your pigeon.")]
pub async fn heal(ctx: &Context, msg: &Message) -> CommandResult {
    let cost = 20;
    let increase = 20;

    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .gold_needed(cost)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    let winnings = PigeonWinningsBuilder::new()
        .gold(-cost)
        .health(increase)
        .build();

    winnings_message(
        ctx,
        msg,
        &winnings,
        "You give your pigeon some health. It's health is refilled!".into(),
    )
    .await?;
    // heal_message(ctx, msg, &winnings).await?;
    PigeonRepository::update_winnings(human_id, &winnings)?;
    Ok(())
}
