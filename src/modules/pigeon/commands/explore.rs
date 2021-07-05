use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::{
    discord_helpers::embed_utils::EmbedExtension,
    modules::pigeon::{
        helpers::validation::PigeonValidation,
        models::pigeon::PigeonStatus,
        repository::{exploration::ExplorationRepository, pigeon::PigeonRepository},
    },
};

async fn success_scenario(msg: &Message, ctx: &Context, image_url: String) {
    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.normal_embed("Your pigeon has successfully taken off to space!")
                    .thumbnail(image_url)
            })
        })
        .await;
}

#[command("explore")]
#[description("Send your pigeon into space.")]
pub async fn explore(ctx: &Context, msg: &Message) -> CommandResult {
    let human_id = PigeonValidation::new()
        // .item_needed("space_shuttle")
        .needs_active_pigeon(true)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    let simple_location = ExplorationRepository::get_random_location()?;
    ExplorationRepository::create_exploration(human_id, simple_location.id)?;
    PigeonRepository::update_status(human_id, PigeonStatus::SpaceExploring);
    success_scenario(msg, ctx, simple_location.image_url).await;

    Ok(())
}
