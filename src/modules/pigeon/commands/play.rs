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
use crate::modules::shared::caching::bucket::Bucket;

#[command("play")]
#[description("Play with your pigeon.")]
pub async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let bucket = Bucket::user("pigeon_play", msg.author.id, Duration::minutes(45));
    let now = bucket.validate()?;

    let cost = 15;
    let increase = 25;

    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .gold_needed(cost)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    let stat = PigeonRepository::get_stat_value(human_id, "happiness")?;
    if stat.value >= 100 {
        return Err("You already have max happiness!".into());
    }

    let winnings = PigeonWinningsBuilder::new()
        .happiness(increase)
        .gold(-cost)
        .build();

    winnings_message(
        ctx,
        msg,
        &winnings,
        "Your pigeon looks bored. You decide to play with it!".into(),
    )
    .await?;

    PigeonRepository::update_winnings(human_id, &winnings)?;
    bucket.spend(now);
    Ok(())
}
