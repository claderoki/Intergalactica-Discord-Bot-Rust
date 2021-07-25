use chrono::Duration;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::modules::pigeon::helpers::utils::PigeonWinnable;
use crate::modules::pigeon::helpers::utils::PigeonWinnings;
use crate::modules::pigeon::helpers::utils::PigeonWinningsBuilder;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::caching::bucket::Bucket;

#[command("poop")]
#[description("Poop on another pigeon.")]
pub async fn poop(ctx: &Context, msg: &Message) -> CommandResult {
    let bucket = Bucket::user("pigeon_poop", msg.author.id, Duration::minutes(60));
    let now = bucket.validate()?;

    let recipient = msg.mentions.get(0).ok_or("No one mentioned")?;

    let initiator = &msg.author;

    let mut validator = PigeonValidation::new();
    validator
        .needs_active_pigeon(true)
        .required_pigeon_status(PigeonStatus::Idle);

    let initiator_human_id = validator.validate(initiator)?;
    let recipient_human_id = validator.other(true).validate(recipient)?;

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
    )
    .await?;

    PigeonRepository::add_poop_victim_count(initiator_human_id);
    PigeonRepository::add_pooped_on_count(recipient_human_id);

    bucket.spend(now);
    Ok(())
}

pub async fn winnings_message(
    ctx: &Context,
    msg: &Message,
    initiator_winnings: &PigeonWinnings,
    recipient_winnings: &PigeonWinnings,
    initiator_human_id: i32,
    recipient_human_id: i32,
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
