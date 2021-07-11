use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::modules::pigeon::helpers::utils::PigeonWinnable;
use crate::modules::pigeon::helpers::utils::PigeonWinnings;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::models::exploration::Exploration;
use crate::modules::pigeon::models::exploration::ExplorationAction;
use crate::modules::pigeon::models::exploration::ExplorationActionScenario;
use crate::modules::pigeon::models::exploration::ExplorationActionScenarioWinnings;
use crate::modules::pigeon::models::exploration::ExplorationEndStats;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::exploration::ExplorationRepository;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::helpers::chooser::choose;
use crate::modules::shared::helpers::chooser::Choosable;
use crate::modules::shared::helpers::utils::TimeDelta;
use crate::modules::shared::repository::item::ItemRepository;
use crate::modules::shared::repository::item::SimpleItem;

#[command("space")]
#[description("Retrieve a space exploration.")]
pub async fn space(ctx: &Context, msg: &Message) -> CommandResult {
    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .required_pigeon_status(PigeonStatus::SpaceExploring)
        .validate(&msg.author)?;

    let exploration = ExplorationRepository::get_exploration(human_id)?;

    if !exploration.arrived {
        still_travelling_message(msg, ctx, &exploration).await;
    } else if exploration.actions_remaining > 0 {
        let action = choose_action(msg, ctx, &exploration).await?;
        let scenario = ExplorationRepository::get_scenario(action.id)?;
        let scenario_winnings =
            ExplorationRepository::get_scenario_winnings(scenario.winnings_id, human_id)?;
        let mut winnings = scenario_winnings.to_pigeon_winnings();
        let item = get_item(&scenario_winnings, &mut winnings)?;
        PigeonRepository::update_winnings(human_id, &winnings)?;
        ExplorationRepository::reduce_action_remaining(exploration.id);
        ExplorationRepository::add_exploration_winnings(exploration.id, action.id, &winnings);
        let remaining = exploration.actions_remaining - 1;
        scenario_winnings_message(msg, ctx, &scenario, &action, &winnings, remaining, &item).await;
    }

    if exploration.arrived && exploration.actions_remaining-1 <= 0 {
        let end_stats = ExplorationRepository::get_end_stats(exploration.id)?;
        PigeonRepository::update_status(human_id, PigeonStatus::Idle);
        ExplorationRepository::finish_exploration(exploration.id);
        exploration_done_message(&msg, &ctx, &exploration, end_stats).await;
    }

    Ok(())
}

fn get_item(
    scenario_winnings: &ExplorationActionScenarioWinnings,
    winnings: &mut PigeonWinnings,
) -> Result<Option<SimpleItem>, String> {
    Ok(if winnings.item_ids.len() > 0 {
        ItemRepository::get_simple(*winnings.item_ids.get(0).unwrap()).ok()
    } else if scenario_winnings.item_category_id.is_some() {
        let item = ItemRepository::get_random(scenario_winnings.item_category_id.unwrap())?;
        winnings.item_ids.push(item.id);
        Some(item)
    } else {
        None
    })
}

async fn scenario_winnings_message(
    msg: &Message,
    ctx: &Context,
    scenario: &ExplorationActionScenario,
    action: &ExplorationAction,
    winnings: &PigeonWinnings,
    actions_remaining: i32,
    item: &Option<SimpleItem>,
) {
    let mut text = format!("{}\n\n{}", scenario.text, &winnings.to_string());

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title(format!(
                    "{} {}",
                    action.get_emoji().unwrap_or("".into()),
                    action.get_description()
                ));

                if let Some(i) = item {
                    e.thumbnail(&i.image_url);
                    text.push_str(&format!("\n\nReceived: 1 `{}`!", &i.name))
                };
                e.normal_embed(&text)
                    .footer(|f| f.text(format!("{} actions remaining", actions_remaining)))
            })
        })
        .await
        .or(Err("scenario_winnings_message failure"));
}

async fn exploration_done_message(
    msg: &Message,
    ctx: &Context,
    exploration: &Exploration,
    end_stats: ExplorationEndStats,
) {
    let text = format!(
        "After {} of exploring Luna, your pigeon finally returns home.\n\n",
        TimeDelta::from_seconds(end_stats.total_seconds).to_text()
    );

    let winnings = &end_stats.to_pigeon_winnings();

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.normal_embed(&text).footer(|f| f.text(""));
                e.field("Stats", &winnings.to_string(), false);
                if !&winnings.item_ids.is_empty() {
                    let items_result = ExplorationRepository::get_end_items(exploration.id);
                    match items_result {
                        Ok(items) => {
                            let mut value = String::from("");
                            for item in items {
                                value.push_str(&format!("{}x {}\n", item.amount, item.name));
                            }
                            e.field("Items", value, false);
                        }
                        _ => {}
                    }
                }
                e
            })
        })
        .await
        .or(Err("exploration_done_message failure"));
}

async fn still_travelling_message(msg: &Message, ctx: &Context, exploration: &Exploration) {
    let location = ExplorationRepository::get_location(exploration.location_id).unwrap();

    let text = format!(
        "Your pigeon is still travelling to {} and is set to arrive in {}\n",
        location.planet_name,
        TimeDelta::from_seconds(exploration.remaining_seconds).to_text()
    );

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.normal_embed(&text)
                    .footer(|f| f.text(format!("progress: {}% / 100%", exploration.percentage)))
                    .thumbnail(location.image_url)
            })
        })
        .await
        .or(Err("Oops"));
}

async fn choose_action(
    msg: &Message,
    ctx: &Context,
    exploration: &Exploration,
) -> Result<ExplorationAction, String> {
    let mut actions = ExplorationRepository::get_available_actions(exploration.location_id)?;
    let location = ExplorationRepository::get_location(exploration.location_id).unwrap();

    let index = choose::<ExplorationAction, _>(ctx, msg, &actions, |e| {
        e.normal_embed(&format!(
            "You arrive at {}.\n\nWhat action would you like to perform?\n",
            location.planet_name
        ))
        .footer(|f| {
            f.text(format!(
                "{} / {} actions remaining",
                exploration.actions_remaining, exploration.total_actions,
            ))
        })
        .thumbnail(location.image_url)
    })
    .await?;
    let action = actions.swap_remove(index);
    Ok(action)
}

/*
.create_select_menu(|s| {
    s.min_values(1)
        .placeholder("hmm")
        .max_values(max)
        .custom_id("abc")
        .options(|m| {
            m.create_option(|o| {
                o.description("Option 1").label("1").value("option 1")
            })
            .create_option(|o| {
                o.description("Option 2").label("2").value("otpion 2")
            })
        })
})
*/

/*

alien logbook, a way to contact aliens, npcs?

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
