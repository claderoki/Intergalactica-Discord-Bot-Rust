use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::{
    discord_helpers::embed_utils::EmbedExtension,
    modules::pigeon::{
        helpers::{utils::winning_to_emoji, validation::PigeonValidation},
        models::pigeon::{PigeonProfile, PigeonStatus},
        repository::{exploration::ExplorationRepository, pigeon::PigeonRepository},
    },
};

#[command("profile")]
#[description("View your pigeons profile.")]
pub async fn profile(ctx: &Context, msg: &Message) -> CommandResult {
    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .validate(&msg.author)?;

    let profile = PigeonRepository::get_profile(human_id)?;
    profile_message(ctx, msg, human_id, &profile).await?;
    Ok(())
}

pub async fn profile_message(
    ctx: &Context,
    msg: &Message,
    human_id: i32,
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
                e.title(&profile.name).normal_embed(&text).footer(|f| {
                    match profile.status {
                        PigeonStatus::SpaceExploring => {
                            let exploration = ExplorationRepository::get_exploration(human_id)
                                .expect("no exploration");
                            let location =
                                ExplorationRepository::get_location(exploration.location_id)
                                    .expect("no location");
                            f.icon_url(location.image_url).text({
                                if exploration.arrived {
                                    format!("exploring {}", location.planet_name)
                                } else {
                                    format!("traveling to {}", location.planet_name)
                                }
                            });
                        }
                        _ => {
                            f.text(profile.status.get_friendly_verb());
                        }
                    };
                    f
                })
            })
        })
        .await
        .or(Err("Failed to send profile"));

    Err("")
}
