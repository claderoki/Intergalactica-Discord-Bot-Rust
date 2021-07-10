use crate::database::connection::get_connection_diesel;
use crate::modules::pigeon::helpers::utils::PigeonWinnings;
use crate::modules::pigeon::models::pigeon::GoldModifier;
use crate::modules::pigeon::models::pigeon::PigeonName;
use crate::modules::pigeon::models::pigeon::PigeonProfile;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::models::pigeon::DecayingPigeon;
use crate::modules::shared::repository::item::ItemRepository;
use diesel::sql_query;
use diesel::sql_types::Bool;
use diesel::sql_types::Double;
use diesel::sql_types::Integer;
use diesel::types::Varchar;
use diesel::RunQueryDsl;
pub struct PigeonRepository;

impl PigeonRepository {
    pub fn update_winnings(human_id: i32, winnings: &PigeonWinnings) -> Result<(), String> {
        let connection = get_connection_diesel();

        let _results = sql_query(include_str!("queries/pigeon/update_winnings.sql"))
            .bind::<Integer, _>(winnings.health)
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

    pub fn get_name(human_id: i32) -> Result<PigeonName, String> {
        let connection = get_connection_diesel();

        let results: Result<PigeonName, _> = sql_query(include_str!("queries/pigeon/get_name.sql"))
            .bind::<Integer, _>(human_id)
            .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn get_decaying_pigeons() -> Result<Vec<DecayingPigeon>, String> {
        let connection = get_connection_diesel();

        let results: Result<Vec<DecayingPigeon>, _> =
            sql_query(include_str!("queries/pigeon/get_decaying_pigeons.sql"))
                .get_results(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
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

    pub fn get_gold_modifier(human_id: i32) -> Result<GoldModifier, String> {
        let connection = get_connection_diesel();

        let results: Result<GoldModifier, _> =
            sql_query(include_str!("queries/pigeon/get_gold_modifier.sql"))
                .bind::<Integer, _>(human_id)
                .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn increase_gold_modifier(human_id: i32, value: f64) {
        let connection = get_connection_diesel();

        let _results = sql_query(include_str!("queries/pigeon/increase_gold_modifier.sql"))
            .bind::<Double, _>(value)
            .bind::<Integer, _>(human_id)
            .execute(&connection);
    }

    pub fn update_status(human_id: i32, status: PigeonStatus) {
        let connection = get_connection_diesel();

        let _results = sql_query(include_str!("queries/pigeon/update_status.sql"))
            .bind::<Varchar, _>(status.to_string())
            .bind::<Integer, _>(human_id)
            .execute(&connection);
    }

    pub fn update_death_notified(human_id: i32, value: bool) {
        let connection = get_connection_diesel();

        let _results = sql_query(include_str!("queries/pigeon/update_death_notified.sql"))
            .bind::<Bool, _>(value)
            .bind::<Integer, _>(human_id)
            .execute(&connection);
    }

    pub fn create(human_id: i32, name: &str) -> Result<(), &'static str> {
        let connection = get_connection_diesel();

        let _results = sql_query(include_str!("queries/pigeon/create.sql"))
            .bind::<Varchar, _>(name)
            .bind::<Integer, _>(human_id)
            .execute(&connection)
            .or(Err("Failed to update pigeon."))?;
        Ok(())
    }
}
