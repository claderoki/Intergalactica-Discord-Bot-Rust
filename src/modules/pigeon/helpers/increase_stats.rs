use chrono::Duration;
use serenity::client::Context;
use serenity::model::channel::Message;

use super::utils::PigeonWinnings;
use super::utils::PigeonWinningsBuilder;
use super::validation::PigeonValidation;
use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::modules::pigeon::helpers::utils::PigeonWinnable;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::caching::bucket::Bucket;

pub struct IncreaseParams {
    pub cost: i32,
    pub increase: i32,
    pub command_name: String,
    pub cooldown: Duration,
    pub stat_name: String,
    pub message: String,
}

impl IncreaseParams {
    pub fn new(
        cost: i32,
        increase: i32,
        command_name: &'static str,
        cooldown: Duration,
        stat_name: &'static str,
        message: &'static str,
    ) -> Self {
        Self {
            cost: cost,
            increase: increase,
            command_name: command_name.into(),
            cooldown: cooldown,
            stat_name: stat_name.into(),
            message: message.into(),
        }
    }
}

pub async fn increase(ctx: &Context, msg: &Message, params: IncreaseParams) -> Result<(), String> {
    let bucket = Bucket::user(&params.command_name, msg.author.id, params.cooldown);
    let now = bucket.validate()?;

    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .gold_needed(params.cost)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    let stat = PigeonRepository::get_stat_value(human_id, &params.stat_name)?;
    if stat.value >= 100 {
        return Err(format!("You already have max {}!", params.stat_name));
    }

    let winnings = PigeonWinningsBuilder::new()
        .dynamic_stat(&params.stat_name, params.increase)
        .gold(-params.cost)
        .build();

    winnings_message(ctx, msg, &winnings, params.message).await?;

    PigeonRepository::update_winnings(human_id, &winnings)?;
    bucket.spend(now);
    Ok(())
}

pub async fn winnings_message(
    ctx: &Context,
    msg: &Message,
    winnings: &PigeonWinnings,
    message: String,
) -> Result<(), &'static str> {
    let text = format!("{}\n{}", message, winnings.to_string());

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| m.embed(|e| e.normal_embed(&text)))
        .await
        .or(Err("Failed to send winnings message"));

    Ok(())
}
