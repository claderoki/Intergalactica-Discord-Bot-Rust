use chrono::NaiveDateTime;
use diesel::sql_query;
use diesel::sql_types::Datetime;
use diesel::sql_types::Double;
use diesel::sql_types::Integer;
use diesel::sql_types::Nullable;
use diesel::RunQueryDsl;

use tracing::error;

use crate::database::connection::get_connection_diesel;
use crate::modules::pigeon::helpers::utils::PigeonWinnings;
use crate::modules::pigeon::models::exploration::*;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::repository::item::SimpleItem;

pub struct ExplorationRepository;
impl ExplorationRepository {
    pub fn get_end_stats(exploration_id: i32) -> Result<ExplorationEndStats, String> {
        let connection = get_connection_diesel()?;

        let results: Result<ExplorationEndStats, _> =
            sql_query(include_str!("queries/exploration/get_end_stats.sql"))
                .bind::<Integer, _>(exploration_id)
                .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get end stats.".into())
            }
        }
    }

    pub fn reduce_action_remaining(exploration_id: i32) -> Result<(), String> {
        let connection = get_connection_diesel()?;

        match sql_query(include_str!(
            "queries/exploration/reduce_action_remaining.sql"
        ))
        .bind::<Integer, _>(exploration_id)
        .execute(&connection)
        {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err("Failed to reduce action remaining".into())
            }
        }
    }

    pub fn get_end_items(exploration_id: i32) -> Result<Vec<SimpleItem>, String> {
        let connection = get_connection_diesel()?;

        let results: Result<Vec<SimpleItem>, _> =
            sql_query(include_str!("queries/exploration/get_end_items.sql"))
                .bind::<Integer, _>(exploration_id)
                .get_results(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get end items.".into())
            }
        }
    }

    pub fn finish_exploration(exploration_id: i32) -> Result<(), String> {
        let connection = get_connection_diesel()?;

        match sql_query(include_str!("queries/exploration/finish_exploration.sql"))
            .bind::<Integer, _>(exploration_id)
            .execute(&connection)
        {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err("Failed to finish exploration".into())
            }
        }
    }

    pub fn add_exploration_winnings(
        exploration_id: i32,
        action_id: i32,
        winnings: &PigeonWinnings,
    ) -> Result<(), String> {
        let connection = get_connection_diesel()?;

        let results = sql_query(include_str!(
            "queries/exploration/add_exploration_winnings.sql"
        ))
        .bind::<Integer, _>(winnings.gold)
        .bind::<Integer, _>(winnings.health)
        .bind::<Integer, _>(winnings.experience)
        .bind::<Integer, _>(winnings.cleanliness)
        .bind::<Integer, _>(winnings.food)
        .bind::<Integer, _>(winnings.happiness)
        .bind::<Nullable<Integer>, _>(winnings.item_ids.get(0))
        .bind::<Integer, _>(exploration_id)
        .bind::<Integer, _>(action_id)
        .execute(&connection);

        match results {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err("Failed to add winnings".into())
            }
        }
    }

    pub fn get_scenario_winnings(
        winnings_id: i32,
        human_id: i32,
    ) -> Result<ExplorationActionScenarioWinnings, String> {
        let connection = get_connection_diesel()?;

        let gold_modifier = PigeonRepository::get_gold_modifier(human_id)?;

        let results: Result<ExplorationActionScenarioWinnings, _> = sql_query(include_str!(
            "queries/exploration/get_scenario_winnings.sql"
        ))
        .bind::<Double, _>(gold_modifier.value)
        .bind::<Integer, _>(winnings_id)
        .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get scenario winnings.".into())
            }
        }
    }

    pub fn get_scenario(action_id: i32) -> Result<ExplorationActionScenario, String> {
        let connection = get_connection_diesel()?;

        let results: Result<ExplorationActionScenario, _> =
            sql_query(include_str!("queries/exploration/get_scenario.sql"))
                .bind::<Integer, _>(action_id)
                .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get scenario.".into())
            }
        }
    }
    pub fn create_exploration(
        human_id: i32,
        location_id: i32,
        arrival_date: NaiveDateTime,
    ) -> Result<(), String> {
        let connection = get_connection_diesel()?;

        let results = sql_query(include_str!("queries/exploration/create_exploration.sql"))
            .bind::<Integer, _>(location_id)
            .bind::<Datetime, _>(arrival_date)
            .bind::<Integer, _>(human_id)
            .execute(&connection);

        match results {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to create exploration.".into())
            }
        }
    }

    pub fn get_random_location() -> Result<SimplePlanetLocation, String> {
        let connection = get_connection_diesel()?;

        let results: Result<SimplePlanetLocation, _> =
            sql_query(include_str!("queries/exploration/random_location.sql"))
                .get_result(&connection);

        match results {
            Ok(simple_location) => Ok(simple_location),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get random location.".into())
            }
        }
    }

    pub fn get_location(location_id: i32) -> Result<PlanetLocation, String> {
        let connection = get_connection_diesel()?;

        let results: Result<PlanetLocation, _> =
            sql_query(include_str!("queries/exploration/get_location.sql"))
                .bind::<Integer, _>(location_id)
                .get_result(&connection);

        match results {
            Ok(location) => Ok(location),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get location.".into())
            }
        }
    }

    pub fn get_exploration(human_id: i32) -> Result<Exploration, String> {
        let connection = get_connection_diesel()?;

        let results: Result<Exploration, _> =
            sql_query(include_str!("queries/exploration/get_exploration.sql"))
                .bind::<Integer, _>(human_id)
                .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get exploration.".into())
            }
        }
    }

    pub fn get_available_actions(location_id: i32) -> Result<Vec<ExplorationAction>, String> {
        let connection = get_connection_diesel()?;

        let results: Result<Vec<ExplorationAction>, _> = sql_query(include_str!(
            "queries/exploration/get_available_actions.sql"
        ))
        .bind::<Integer, _>(location_id)
        .get_results(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get available actions.".into())
            }
        }
    }
}
