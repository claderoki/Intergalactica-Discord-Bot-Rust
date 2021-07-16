use chrono::Duration;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::modules::pigeon::helpers::utils::PigeonWinningsBuilder;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::helpers::winning_message::winnings_message;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::caching::flag::FlagCache;
use crate::modules::shared::caching::flag::FlagValidator;
use crate::modules::shared::caching::flag::PigeonLastFed;

#[command("feed")]
#[description("Feed your pigeon.")]
pub async fn feed(ctx: &Context, msg: &Message) -> CommandResult {
    let cost = 15;
    let increase = 25;

    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .gold_needed(cost)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    let stat = PigeonRepository::get_stat_value(human_id, "food")?;
    if stat.value >= 100 {
        return Err("You already have max food!".into());
    }

    let now = FlagValidator::validate::<PigeonLastFed>(human_id, Duration::minutes(45))?;

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
    FlagCache::add::<PigeonLastFed>(human_id, now);
    Ok(())
}
