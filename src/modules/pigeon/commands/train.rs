use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::discord_helpers::ui::GoldConfirmation;
use crate::modules::pigeon::helpers::utils::PigeonWinnable;
use crate::modules::pigeon::helpers::utils::PigeonWinnings;
use crate::modules::pigeon::helpers::utils::PigeonWinningsBuilder;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::caching::bucket::Bucket;
use crate::modules::shared::repository::human::HumanRepository;
use chrono::Duration;

#[command("train")]
#[only_in(guild)]
#[description("Bulk up your bird.")]
pub async fn train(ctx: &Context, msg: &Message) -> CommandResult {
    let bucket = Bucket::user("pigeon_train", msg.author.id, Duration::minutes(10));
    let now = bucket.validate().map_err(|e| {
        format!(
            "Your pigeon is still resting from his last, intense training session.\n{}",
            e
        )
    })?;

    let increase = 0.01;

    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    let gold_modifier = PigeonRepository::get_gold_modifier(human_id)?;
    if gold_modifier.value + increase > 1.5 {
        return Err("You can't go over 1.5 for now".into());
    }

    let cost = calculate_cost(gold_modifier.value);

    if !HumanRepository::has_gold(human_id, cost)? {
        return Err(format!("You need {} gold to perform this action", cost).into());
    }

    if !GoldConfirmation::new().confirm(ctx, msg, cost).await? {
        return Err("Cancelled".into());
    }

    PigeonRepository::increase_gold_modifier(human_id, increase)?;
    HumanRepository::spend_gold(human_id, cost)?;

    let winnings = PigeonWinningsBuilder::new().gold(-cost).build();

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| create_modifier_embed(e, increase, gold_modifier.value, &winnings))
        })
        .await?;

    bucket.spend(now);

    Ok(())
}

fn create_modifier_embed<'a>(
    embed: &'a mut CreateEmbed,
    increase: f64,
    modifier: f64,
    winnings: &PigeonWinnings,
) -> &'a mut CreateEmbed {
    embed.normal_embed(&format!(
        "Your gold modifier has been increased by {} and is now {:.2}!\n\n{}",
        increase,
        (modifier + increase),
        winnings.to_string()
    ))
}

fn calculate_cost(modifier: f64) -> i32 {
    let base_cost = 100.0;
    let cost = base_cost * (modifier * 3.0);
    cost as i32
}
