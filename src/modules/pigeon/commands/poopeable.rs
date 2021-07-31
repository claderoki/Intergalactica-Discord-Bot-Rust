use chrono::Duration;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::guild::Member;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::discord_helpers::ui::GoldConfirmation;
use crate::modules::pigeon::commands::poop::LastPoopedOn;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::caching::bucket::Bucket;
use crate::modules::shared::caching::flag::FlagValidator;
use crate::modules::shared::repository::human::HumanRepository;

#[command("poopeable")]
#[only_in(guild)]
#[description("Find a poopeable pigeon.")]
pub async fn poopeable(ctx: &Context, msg: &Message) -> CommandResult {
    let bucket = Bucket::user("pigeon_poopeable", msg.author.id, Duration::minutes(60));
    let now = bucket.validate()?;
    let cost = 100;

    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .gold_needed(cost)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    if !GoldConfirmation::new().confirm(ctx, msg, cost).await? {
        return Err("Cancelled".into());
    }

    let member = get_member(ctx, msg).await.ok_or("No members found.")?;

    msg.author
        .dm(&ctx, |m| m.embed(|e| e.normal_embed(format!("{}", member))))
        .await
        .map_err(|_| "Unable to send you a DM.")?;

    HumanRepository::spend_gold(human_id, cost)?;
    bucket.spend(now);
    Ok(())
}

pub async fn get_member(ctx: &Context, msg: &Message) -> Option<Member> {
    if let Some(guild) = msg.guild(ctx).await {
        if let Ok(user_ids) = PigeonRepository::get_idle_pigeon_users(guild.id.0) {
            for user_id in user_ids {
                if FlagValidator::validate::<LastPoopedOn>(user_id.value, Duration::minutes(60)).is_ok() {
                    if let Ok(member) = guild.member(ctx, user_id.value).await {
                        return Some(member);
                    }
                }
            }
        }
    }

    None
}
