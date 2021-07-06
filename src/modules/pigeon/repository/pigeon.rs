use crate::modules::pigeon::models::pigeon::PigeonProfile;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::shared::repository::item::ItemRepository;
use crate::{
    database::connection::get_connection_diesel, modules::pigeon::helpers::utils::PigeonWinnings,
};
use diesel::types::Varchar;
use diesel::{sql_query, sql_types::Integer, RunQueryDsl};
pub struct PigeonRepository;

impl PigeonRepository {
    pub fn update_winnings(human_id: i32, winnings: &PigeonWinnings) -> Result<(), String> {
        let connection = get_connection_diesel();

        let _results = sql_query(include_str!("queries/pigeon/update_winnings.sql"))
            // TODO: set dead to true when health under 0? think about a way to notify people their pigeon died
            .bind::<Integer, _>(winnings.health)
            .bind::<Integer, _>(winnings.happiness)
            .bind::<Integer, _>(winnings.cleanliness)
            .bind::<Integer, _>(winnings.experience)
            .bind::<Integer, _>(winnings.food)
            .bind::<Integer, _>(winnings.gold)
            .bind::<Integer, _>(human_id)
            .execute(&connection);

        match _results {
            Err(e) => return Err(format!("{:?}", e).into()),
            _ => {}
        }

        if !winnings.item_ids.is_empty() {
            let _ = ItemRepository::add_items((*winnings.item_ids).to_vec(), human_id)?;
        }

        Ok(())
    }

    pub fn get_profile(human_id: i32) -> Result<PigeonProfile, String> {
        let connection = get_connection_diesel();

        let results: Result<PigeonProfile, _> =
            sql_query(include_str!("queries/pigeon/get_profile.sql"))
                .bind::<Integer, _>(human_id)
                .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn update_status(human_id: i32, status: PigeonStatus) {
        let connection = get_connection_diesel();

        let _results = sql_query(include_str!("queries/pigeon/update_status.sql"))
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
            .or(Err("Failed to update pigeon."))?;
        Ok(())
    }
}
