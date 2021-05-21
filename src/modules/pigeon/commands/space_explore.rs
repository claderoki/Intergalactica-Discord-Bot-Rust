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
        repository::{pigeon::PigeonRepository, planet_exploration::PlanetExplorationRepository},
    },
};

enum ExploreScenario {
    Success = 1
}




#[command("spaceexplore")]
#[description("Retrieve a space.")]
pub async fn space_explore(ctx: &Context, msg: &Message) -> CommandResult {
    let human_id = PigeonValidation::new()
        .item_needed("space_shuttle")
        .needs_active_pigeon(true)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    let simple_location = PlanetExplorationRepository::get_location()?;
    PlanetExplorationRepository::create_exploration(simple_location.id)?;
    PigeonRepository::update_status(human_id, PigeonStatus::SpaceExploring);

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.normal_embed("Your pigeon has successfully taken off to space!")
                    .thumbnail(simple_location.image_url)
            })
        })
        .await;

    Ok(())
}

/*
Every planet will be unique with its scenario's

Space poop

Scenario's
- Meeting aliens
    - Befriend them
    - Be kidnapped by them (for a duration of time / until another pigeon saves them)
    - Learn their language
    - Date them?

Mercury - metal factory

Mars - waterpark
Mars - rover, maybe a scenario where you drive it?
Mars -
Mars -
Mars -
Mars -
Mars -
Mars -
Mars -
Mars -
Mars -

Moon - sentient / sapient
Moon - moon cheese
Moon - has a flag
Moon - rollercoaster
Moon - Moonstone factory
Moon -
Moon -
Moon -
Moon -
Moon -
Moon -
Moon -
Moon -

Moon - secret hideout
Moon - meteor shower (damage)
Moon -

Spaceship
3 parts
- fuel
- moon boots

"Oh no, an alien grabbed **{pigeon.name}** from the sky on its way to the moon"
"Your pigeon has found all parts for your space suit / spaceship and can now travel to space. Would you like to send **{pigeon.name}** into space?"

*/

/*
RETRIEVAL STEPS
1. Check if there is a retrieval and if it is ready to be retrieved
2.

planet
id, name

planet_exploration
id, planet_location_id, start_time, end_time, finished

space_bonus
id, name, planet_location_id

planet_location
id, name, planet_id

*/
