use crate::modules::pigeon::models::pigeon::{PigeonProfile};
use crate::{
    database::connection::get_connection_diesel, modules::pigeon::helpers::utils::PigeonWinnings,
};
use crate::{
    modules::pigeon::models::pigeon::{PigeonStatus},
};
use diesel::{
    types::{Varchar},
};
use diesel::{sql_query, sql_types::Integer, RunQueryDsl};
pub struct PigeonRepository;

impl PigeonRepository {

    pub fn update_winnings(human_id: i32, winnings: &PigeonWinnings) {
        let connection = get_connection_diesel();

        let _results = sql_query(include_str!("queries/pigeon/update_winnings.sql")) 
            // TODO: set dead to true when health under 0? think about a way to notify people their pigeon died
            .bind::<Varchar, _>(winnings.health.to_string())
            .bind::<Varchar, _>(winnings.happiness.to_string())
            .bind::<Varchar, _>(winnings.cleanliness.to_string())
            .bind::<Varchar, _>(winnings.experience.to_string())
            .bind::<Varchar, _>(winnings.food.to_string())
            .bind::<Varchar, _>(winnings.gold.to_string())
            .bind::<Integer, _>(human_id)
            .execute(&connection);
    }

    pub fn get_profile(human_id: i32) -> Result<PigeonProfile, String> {
        let connection = get_connection_diesel();

        let results: Result<PigeonProfile, _> = sql_query(include_str!("queries/pigeon/get_profile.sql"))
            .bind::<Integer, _>(human_id)
            .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn update_status(human_id: i32, status: PigeonStatus) {
        let connection = get_connection_diesel();

        let _results = sql_query(include_str!("queries/pigeon/update_winnings.sql"))
        .bind::<Varchar, _>(status.to_string())
        .bind::<Integer, _>(human_id)
        .execute(&connection);
    }

    pub fn create(human_id: i32, name: &str) -> Result<(), &'static str> {

        let connection = get_connection_diesel();

        let _results = sql_query(include_str!("queries/pigeon/update_winnings.sql"))
        .bind::<Varchar, _>(name)
        .bind::<Integer, _>(human_id)
        .execute(&connection)
        .or(Err("Failed to create a pigeon."))?;
        Ok(())
    }
}
