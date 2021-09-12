use chrono::Duration;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::model::interactions::InteractionResponseType;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::modules::pigeon::helpers::utils::winning_to_string;
use crate::modules::pigeon::helpers::utils::PigeonWinnable;
use crate::modules::pigeon::helpers::utils::PigeonWinnings;
use crate::modules::pigeon::helpers::utils::PigeonWinningsBuilder;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::models::exploration::Exploration;
use crate::modules::pigeon::models::exploration::ExplorationAction;
use crate::modules::pigeon::models::exploration::ExplorationActionScenario;
use crate::modules::pigeon::models::exploration::ExplorationActionScenarioWinnings;
use crate::modules::pigeon::models::exploration::ExplorationEndStats;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::exploration::ExplorationRepository;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::caching::bucket::Bucket;
use crate::modules::shared::helpers::chooser::edit_msg;
use crate::modules::shared::helpers::chooser::generate_msg;
use crate::modules::shared::helpers::chooser::Choosable;
use crate::modules::shared::helpers::utils::TimeDelta;
use crate::modules::shared::repository::item::ItemRepository;
use crate::modules::shared::repository::item::SimpleItem;
use crate::modules::shared::repository::streak::StreakRepository;

#[command("space")]
#[only_in(guild)]
#[description("Retrieve a space exploration.")]
pub async fn space(ctx: &Context, msg: &Message) -> CommandResult {
    let bucket = Bucket::user("pigeon_space", msg.author.id, Duration::minutes(60));
    let now = bucket
        .validate()
        .map_err(|_| "You can only run this command once at a time.")?;
    bucket.spend(now);
    let result = run_command(&ctx, &msg).await;
    bucket.revert();
    result
}

async fn run_command(ctx: &Context, msg: &Message) -> CommandResult {
    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .required_pigeon_status(PigeonStatus::SpaceExploring)
        .validate(&msg.author)?;

    let exploration = ExplorationRepository::get_exploration(human_id)?;
    let mut actions_used = 0;

    if !exploration.arrived {
        still_travelling_message(msg, ctx, &exploration).await;
    } else if exploration.actions_remaining > 0 {
        let mut used_action_indexes: Vec<i32> = Vec::new();

        let actions = ExplorationRepository::get_available_actions(exploration.location_id)?;
        let location = ExplorationRepository::get_location(exploration.location_id)?;
        let mut interactive_msg = generate_msg(&ctx, &msg, &actions, |e| {
            e.normal_embed(&format!(
                "You arrive at {}.\n\nWhat action would you like to perform?\n",
                &location.planet_name
            ))
            .footer(|f| {
                f.text(format!(
                    "{} / {} actions remaining",
                    &exploration.actions_remaining, &exploration.total_actions,
                ))
            })
            .thumbnail(&location.image_url)
        })
        .await?;

        for i in 0..exploration.actions_remaining {
            if i > 0 {
                let _ = edit_msg(
                    &ctx,
                    &mut interactive_msg,
                    &actions,
                    &used_action_indexes,
                    |e| {
                        e.normal_embed(&format!(
                            "You arrive at {}.\n\nWhat action would you like to perform?\n",
                            &location.planet_name
                        ))
                        .footer(|f| {
                            f.text(format!(
                                "{} / {} actions remaining",
                                &exploration.actions_remaining, &exploration.total_actions,
                            ))
                        })
                        .thumbnail(&location.image_url)
                    },
                )
                .await;
            }

            let index = get_action_index(&ctx, &interactive_msg, msg.author.id).await?;
            let action = actions.get(index).ok_or("Index wrong.")?;
            let scenario = ExplorationRepository::get_scenario(action.id)?;
            let scenario_winnings =
                ExplorationRepository::get_scenario_winnings(scenario.winnings_id, human_id)?;
            let mut winnings = scenario_winnings.to_pigeon_winnings();
            let item = get_item(&scenario_winnings, &mut winnings)?;
            PigeonRepository::update_winnings(human_id, &winnings)?;
            ExplorationRepository::reduce_action_remaining(exploration.id)?;
            ExplorationRepository::add_exploration_winnings(exploration.id, action.id, &winnings)?;
            let remaining = exploration.actions_remaining - i;
            scenario_winnings_message(msg, ctx, &scenario, &action, &winnings, remaining, &item)
                .await;
            used_action_indexes.push(index as i32);
            actions_used += 1;
        }
    }

    if exploration.arrived && exploration.actions_remaining - actions_used <= 0 {
        let end_stats = ExplorationRepository::get_end_stats(exploration.id)?;
        PigeonRepository::update_status(human_id, PigeonStatus::Idle)?;
        ExplorationRepository::finish_exploration(exploration.id)?;
        let bonuses = get_bonuses(human_id)?;
        let gold = {
            let mut total = 0;
            for bonus in bonuses.iter() {
                total += bonus.gold;
            }
            total
        };
        exploration_done_message(&msg, &ctx, &exploration, end_stats, bonuses).await;
        let bonus_winnings = PigeonWinningsBuilder::new().gold(gold).build();
        PigeonRepository::update_winnings(human_id, &bonus_winnings)?;
    }

    Ok(())
}

struct Bonus {
    pub gold: i32,
    pub message: String,
}

impl Bonus {
    pub fn new(gold: i32, message: String) -> Self {
        Self {
            gold: gold,
            message: message,
        }
    }
}

fn get_bonuses(human_id: i32) -> Result<Vec<Bonus>, String> {
    let mut bonuses: Vec<Bonus> = Vec::new();

    let gold_modifier = PigeonRepository::get_gold_modifier(human_id)?;
    let streak = StreakRepository::get(human_id, "space_exploration")?;
    if streak.days_missed == 1 {
        let streak_bonus =
            ((std::cmp::min(streak.current + 1, 10) * 10) as f64 * gold_modifier.value) as i32;
        StreakRepository::add(human_id, "space_exploration")?;
        bonuses.push(Bonus {
            message: format!(
                "You're on a space exploration streak ({})! Come back tomorrow for more",
                streak.current + 1
            ),
            gold: streak_bonus,
        });
    } else if streak.days_missed > 2 {
        StreakRepository::reset(human_id, "space_exploration")?;
    }

    let count = ExplorationRepository::get_exploration_count(human_id)?;
    if (count + 1) % 10 == 0 {
        let text = format!("{}th space exploration!", count + 1);
        bonuses.push(match count + 1 {
            1000 => Bonus::new(80, text),
            100 => Bonus::new(60, text),
            _ => Bonus::new(40, text),
        });
    }

    Ok(bonuses)
}

async fn get_action_index(
    ctx: &Context,
    msg: &Message,
    author_id: UserId,
) -> Result<usize, &'static str> {
    let interaction = &msg
        .await_component_interaction(&ctx)
        .author_id(author_id)
        .timeout(std::time::Duration::from_secs(60))
        .await
        .ok_or("Timed out...")?;

    let _ = interaction
        .create_interaction_response(&ctx, |f| {
            f.kind(InteractionResponseType::DeferredUpdateMessage)
        })
        .await;

    match interaction.data.custom_id.parse::<usize>() {
        Ok(index) => Ok(index),
        Err(_) => Err("Can't convert to int"),
    }
}

fn get_item(
    scenario_winnings: &ExplorationActionScenarioWinnings,
    winnings: &mut PigeonWinnings,
) -> Result<Option<SimpleItem>, String> {
    Ok(if let Some(item_id) = winnings.item_ids.get(0) {
        ItemRepository::get_simple(*item_id).ok()
    } else if let Some(item_category_id) = scenario_winnings.item_category_id {
        let item = ItemRepository::get_random(item_category_id)?;
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
    bonuses: Vec<Bonus>,
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

                for bonus in bonuses.iter() {
                    e.field(
                        &bonus.message,
                        winning_to_string(bonus.gold, "gold", true),
                        false,
                    );
                }

                if !&winnings.item_ids.is_empty() {
                    let items_result = ExplorationRepository::get_end_items(exploration.id);
                    match items_result {
                        Ok(items) => {
                            let mut value = String::new();
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
    if let Ok(location) = ExplorationRepository::get_location(exploration.location_id) {
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
}
