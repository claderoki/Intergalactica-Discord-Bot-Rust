use serenity::{
    builder::CreateEmbedFooter,
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::{discord_helpers::embed_utils::EmbedExtension, modules::{pigeon::{helpers::{utils::PigeonWinnings, validation::PigeonValidation}, models::pigeon::PigeonStatus, repository::exploration::{Exploration, ExplorationAction, ExplorationRepository}}, shared::helpers::chooser::{choose, Choosable}}};

impl Choosable for ExplorationAction {
    fn get_identifier(&self) -> i32 {
        self.id
    }

    fn get_description(&self) -> String {
        String::from(&self.name)
    }

    fn get_emoji(&self) -> Option<String> {
        Some(String::from(&self.symbol))
    }
}

async fn choose_action(msg: &Message, ctx: &Context, activity: Exploration) -> Result<ExplorationAction, &'static str> {
    let mut actions = ExplorationRepository::get_available_actions(activity.location_id)?;

    let index = choose::<ExplorationAction, _>(msg, ctx, &actions, |e, t| {
        e.normal_embed(&t)
            .footer(|f| f.text(format!("{} actions remaining", activity.actions_remaining)))
            .title("What action would you like to perform?")
    })
    .await?;
    let action = actions.swap_remove(index);
    Ok(action)
}

#[command("space")]
#[description("Retrieve a space exploration.")]
pub async fn space(ctx: &Context, msg: &Message) -> CommandResult {
    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .required_pigeon_status(PigeonStatus::SpaceExploring)
        .validate(&msg.author)?;

    let activity = ExplorationRepository::get_exploration(human_id)?;

    if activity.arrived {
        if activity.actions_remaining == 0 {
            return Err("No further actions available...".into());
        } else {
            let action = choose_action(msg, ctx, activity).await?;

            let scenario = ExplorationRepository::get_scenario(action.id)?;
            let scenario_winnings = ExplorationRepository::get_scenario_winnings(scenario.id)?;

            let mut winnings = PigeonWinnings::new();
            winnings
                .gold(scenario_winnings.gold)
                .happiness(scenario_winnings.happiness)
                .health(scenario_winnings.health)
                .food(scenario_winnings.food)
                .build();

            let mut text = String::from(&scenario.text);
            text.push_str("\n");
            text.push_str(&winnings.to_string());

            return Err(text.into());
        }
    } else {
        let mut text = String::from("This activity is not ready to be retrieved yet\n**");
        text.push_str(&activity.percentage.to_string());
        text.push_str("%** / **100%**");
        return Err(text.into());
    }

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

Moon - sentient / sapient
Moon - moon cheese
Moon - has a flag
Moon - rollercoaster
Moon - Moonstone factory

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
