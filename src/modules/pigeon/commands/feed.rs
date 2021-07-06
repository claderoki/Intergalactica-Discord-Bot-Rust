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

#[command("feed")]
#[description("Feed your pigeon.")]
pub async fn feed(ctx: &Context, msg: &Message) -> CommandResult {
    let cost = 20;
    let increase = 20;

    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .gold_needed(cost)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    let winnings = PigeonWinningsBuilder::new()
        .food(increase)
        .gold(-cost)
        .build();

    winnings_message(
        ctx,
        msg,
        &winnings,
        "You give your pigeon some seeds. It's energy is refilled!".into(),
    )
    .await?;

    PigeonRepository::update_winnings(human_id, &winnings)?;
    Ok(())
}
