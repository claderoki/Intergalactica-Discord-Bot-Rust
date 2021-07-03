use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::{
    discord_helpers::embed_utils::EmbedExtension,
    modules::pigeon::{
        helpers::{utils::winning_to_emoji, validation::PigeonValidation},
        models::pigeon::PigeonProfile,
        repository::pigeon::PigeonRepository,
    },
};

#[command("profile")]
#[description("View your pigeons profile.")]
pub async fn profile(ctx: &Context, msg: &Message) -> CommandResult {
    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .validate(&msg.author)?;

    let profile = PigeonRepository::get_profile(human_id)?;
    profile_message(ctx, msg, &profile).await?;
    Ok(())
}

pub async fn profile_message(
    ctx: &Context,
    msg: &Message,
    profile: &PigeonProfile,
) -> Result<(), &'static str> {
    let text = format!(
        "{} {}
{} {}
{} {}
{} {}
{} {}",
        winning_to_emoji("experience"),
        profile.experience,
        winning_to_emoji("cleanliness"),
        profile.cleanliness,
        winning_to_emoji("happiness"),
        profile.happiness,
        winning_to_emoji("food"),
        profile.food,
        winning_to_emoji("health"),
        profile.health,
    );

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title(&profile.name)
                    .normal_embed(&text)
                    .footer(|f| f.text(format!("Currently {}", profile.status.get_friendly_verb())))
            })
        })
        .await
        .or(Err("Failed to send profile"));

    Err("")
}
