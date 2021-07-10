use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::modules::pigeon::helpers::utils::PigeonWinningsBuilder;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::helpers::winning_message::winnings_message;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;

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
