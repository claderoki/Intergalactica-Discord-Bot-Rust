use chrono::Duration;
use chrono::NaiveDateTime;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::model::user::User;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::discord_helpers::ui::GoldConfirmation;
use crate::flags;
use crate::modules::pigeon::helpers::utils::PigeonWinnable;
use crate::modules::pigeon::helpers::utils::PigeonWinnings;
use crate::modules::pigeon::helpers::utils::PigeonWinningsBuilder;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::caching::bucket::Bucket;
use crate::modules::shared::caching::flag::FlagCache;
use crate::modules::shared::caching::flag::FlagValidator;
use crate::modules::shared::repository::human::HumanRepository;

pub struct LastPoopedOn(NaiveDateTime);
flags! {
    LastPoopedOn;
}

const COST: i32 = 100;

#[command("poop")]
#[description("Poop on another pigeon.")]
pub async fn poop(ctx: &Context, msg: &Message) -> CommandResult {
    let bucket = Bucket::user("pigeon_poop", msg.author.id, Duration::minutes(60));
    let now = bucket.validate()?;

    let initiator = &msg.author;

    let initiator_human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(initiator)?;

    let user_id = match msg.mentions.get(0) {
        Some(user) => {
            run_command(ctx, msg, user, initiator_human_id, false).await?;
            user.id.0
        }
        None => {
            let user = get_poopable(ctx, msg, initiator_human_id).await?;
            run_command(ctx, msg, &user, initiator_human_id, true).await?;
            user.id.0
        }
    };

    FlagCache::add::<LastPoopedOn>(user_id, now);
    bucket.spend(now);

    Ok(())
}

async fn run_command(
    ctx: &Context,
    msg: &Message,
    recipient: &User,
    initiator_human_id: i32,
    randomed: bool,
) -> Result<(), String> {
    FlagValidator::validate::<LastPoopedOn>(recipient.id.0, Duration::minutes(60))
        .map_err(|e| format!("You can not poop on this pigeon yet. {}", e))?;

    let recipient_human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .required_pigeon_status(PigeonStatus::Idle)
        .other(true)
        .validate(recipient)?;

    let initiator_winnings = PigeonWinningsBuilder::new().cleanliness(5).build();
    let recipient_winnings = PigeonWinningsBuilder::new().cleanliness(-10).build();

    PigeonRepository::update_winnings(initiator_human_id, &initiator_winnings)?;
    PigeonRepository::update_winnings(recipient_human_id, &recipient_winnings)?;

    winnings_message(
        ctx,
        msg,
        &initiator_winnings,
        &recipient_winnings,
        initiator_human_id,
        recipient_human_id,
        randomed,
        recipient.id.0,
    )
    .await?;

    PigeonRepository::add_poop_victim_count(initiator_human_id)?;
    PigeonRepository::add_pooped_on_count(recipient_human_id)?;

    if randomed {
        HumanRepository::spend_gold(initiator_human_id, COST)?;
    }

    Ok(())
}

async fn get_poopable(ctx: &Context, msg: &Message, human_id: i32) -> Result<User, String> {
    if !HumanRepository::has_gold(human_id, COST)? {
        return Err("You do not have enough gold".into());
    }

    if !GoldConfirmation::new().confirm(ctx, msg, COST).await? {
        return Err("Cancelled".into());
    }

    let member = get_member(ctx, msg).await.ok_or("No members found.")?;
    Ok(member.user)
}

pub async fn get_member(ctx: &Context, msg: &Message) -> Option<Member> {
    if let Some(guild) = msg.guild(ctx).await {
        if let Ok(user_ids) = PigeonRepository::get_idle_pigeon_users(guild.id.0) {
            for user_id in user_ids {
                if user_id.value == msg.author.id.0 {
                    continue;
                }
                if FlagValidator::validate::<LastPoopedOn>(user_id.value, Duration::minutes(60))
                    .is_ok()
                {
                    if let Ok(member) = guild.member(ctx, user_id.value).await {
                        return Some(member);
                    }
                }
            }
        }
    }
    None
}

async fn winnings_message(
    ctx: &Context,
    msg: &Message,
    initiator_winnings: &PigeonWinnings,
    recipient_winnings: &PigeonWinnings,
    initiator_human_id: i32,
    recipient_human_id: i32,
    randomed: bool,
    recipient_user_id: u64,
) -> Result<(), String> {
    let initiator_name = PigeonRepository::get_name(initiator_human_id)?;
    let recipient_name = PigeonRepository::get_name(recipient_human_id)?;

    let text = format!(
        "Your pigeon successfully poops on {}, and to finish it off, {} wipes its butt clean on its fur.",
        recipient_name.value,
        initiator_name.value,
    );

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            if randomed {
                m.content(format!("<@{}>", recipient_user_id));
            }
            m.embed(|e| {
                e.normal_embed(&text)
                    .field(initiator_name.value, initiator_winnings.to_string(), false)
                    .field(recipient_name.value, recipient_winnings.to_string(), false)
            })
        })
        .await
        .or(Err("Failed to send winnings"));

    Ok(())
}
