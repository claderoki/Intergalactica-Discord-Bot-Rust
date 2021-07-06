use diesel::{
    sql_query,
    sql_types::{Integer, Nullable},
    RunQueryDsl,
};

use crate::{
    database::connection::get_connection_diesel,
    modules::{
        pigeon::{helpers::utils::PigeonWinnings, models::exploration::*},
        shared::repository::item::SimpleItem,
    },
};

pub struct ExplorationRepository;
impl ExplorationRepository {
    pub fn get_end_stats(exploration_id: i32) -> Result<ExplorationEndStats, String> {
        let connection = get_connection_diesel();

        let results: Result<ExplorationEndStats, _> =
            sql_query(include_str!("queries/exploration/get_end_stats.sql"))
                .bind::<Integer, _>(exploration_id)
                .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn reduce_action_remaining(exploration_id: i32) {
        let connection = get_connection_diesel();

        let _results = sql_query(include_str!(
            "queries/exploration/reduce_action_remaining.sql"
        ))
        .bind::<Integer, _>(exploration_id)
        .execute(&connection);
    }

    pub fn get_end_items(exploration_id: i32) -> Result<Vec<SimpleItem>, String> {
        let connection = get_connection_diesel();

        let results: Result<Vec<SimpleItem>, _> =
            sql_query(include_str!("queries/exploration/get_end_items.sql"))
                .bind::<Integer, _>(exploration_id)
                .get_results(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                println!("{:?}", e);
                Err(format!("{:?}", e))
            }
        }
    }

    pub fn finish_exploration(exploration_id: i32) {
        let connection = get_connection_diesel();

        let _results = sql_query(include_str!("queries/exploration/finish_exploration.sql"))
            .bind::<Integer, _>(exploration_id)
            .execute(&connection);
    }

    pub fn add_exploration_winnings(
        exploration_id: i32,
        action_id: i32,
        winnings: &PigeonWinnings,
    ) {
        let connection = get_connection_diesel();

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
            Ok(_) => {}
            Err(e) => {
                println!("{:?}", e);
            }
        };
    }

    pub fn get_scenario_winnings(
        winnings_id: i32,
    ) -> Result<ExplorationActionScenarioWinnings, String> {
        let connection = get_connection_diesel();

        let results: Result<ExplorationActionScenarioWinnings, _> = sql_query(include_str!(
            "queries/exploration/get_scenario_winnings.sql"
        ))
        .bind::<Integer, _>(winnings_id)
        .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn get_scenario(action_id: i32) -> Result<ExplorationActionScenario, String> {
        let connection = get_connection_diesel();

        let results: Result<ExplorationActionScenario, _> =
            sql_query(include_str!("queries/exploration/get_scenario.sql"))
                .bind::<Integer, _>(action_id)
                .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
    pub fn create_exploration(human_id: i32, location_id: i32) -> Result<(), String> {
        let connection = get_connection_diesel();

        let results = sql_query(include_str!("queries/exploration/create_exploration.sql"))
            .bind::<Integer, _>(location_id)
            .bind::<Integer, _>(3)
            .bind::<Integer, _>(human_id)
            .execute(&connection);

        match results {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn get_random_location() -> Result<SimplePlanetLocation, String> {
        let connection = get_connection_diesel();

        let results: Result<SimplePlanetLocation, _> =
            sql_query(include_str!("queries/exploration/random_location.sql"))
                .get_result(&connection);

        match results {
            Ok(simple_location) => Ok(simple_location),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn get_location(location_id: i32) -> Result<PlanetLocation, String> {
        let connection = get_connection_diesel();

        let results: Result<PlanetLocation, _> =
            sql_query(include_str!("queries/exploration/get_location.sql"))
                .bind::<Integer, _>(location_id)
                .get_result(&connection);

        match results {
            Ok(location) => Ok(location),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn get_exploration(human_id: i32) -> Result<Exploration, String> {
        let connection = get_connection_diesel();

        let results: Result<Exploration, _> =
            sql_query(include_str!("queries/exploration/get_exploration.sql"))
                .bind::<Integer, _>(human_id)
                .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn get_available_actions(location_id: i32) -> Result<Vec<ExplorationAction>, String> {
        let connection = get_connection_diesel();

        let results: Result<Vec<ExplorationAction>, _> = sql_query(include_str!(
            "queries/exploration/get_available_actions.sql"
        ))
        .bind::<Integer, _>(location_id)
        .get_results(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
