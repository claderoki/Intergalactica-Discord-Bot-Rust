use crate::database::connection::get_connection_diesel;
use crate::modules::pigeon::helpers::utils::PigeonWinnings;
use crate::modules::pigeon::models::pigeon::DecayingPigeon;
use crate::modules::pigeon::models::pigeon::GoldModifier;
use crate::modules::pigeon::models::pigeon::PigeonName;
use crate::modules::pigeon::models::pigeon::PigeonProfile;
use crate::modules::pigeon::models::pigeon::PigeonStatValue;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::shared::repository::item::ItemRepository;
use diesel::sql_query;
use diesel::sql_types::Bool;
use diesel::sql_types::Double;
use diesel::sql_types::Integer;
use diesel::types::Varchar;
use diesel::RunQueryDsl;
pub struct PigeonRepository;

use tracing::error;

impl PigeonRepository {
    pub fn update_winnings(human_id: i32, winnings: &PigeonWinnings) -> Result<(), String> {
        let connection = get_connection_diesel()?;

        let result = sql_query(include_str!("queries/pigeon/update_winnings.sql"))
            .bind::<Integer, _>(winnings.health)
            .bind::<Integer, _>(winnings.health)
            .bind::<Integer, _>(winnings.happiness)
            .bind::<Integer, _>(winnings.cleanliness)
            .bind::<Integer, _>(winnings.experience)
            .bind::<Integer, _>(winnings.food)
            .bind::<Integer, _>(winnings.gold)
            .bind::<Integer, _>(human_id)
            .execute(&connection);

        if let Err(e) = result {
            println!("{:?}", e);
            return Err("Failed to update winnings.".into());
        }

        if !winnings.item_ids.is_empty() {
            let _ = ItemRepository::add_items((*winnings.item_ids).to_vec(), human_id)?;
        }

        Ok(())
    }

    pub fn unjail_all() -> Result<(), String> {
        let connection = get_connection_diesel()?;

        let result = sql_query(include_str!("queries/pigeon/unjail_all.sql")).execute(&connection);

        if let Err(e) = result {
            println!("{:?}", e);
            return Err("Failed to unjail winnings.".into());
        }

        Ok(())
    }

    pub fn get_name(human_id: i32) -> Result<PigeonName, String> {
        let connection = get_connection_diesel()?;

        let results: Result<PigeonName, _> = sql_query(include_str!("queries/pigeon/get_name.sql"))
            .bind::<Integer, _>(human_id)
            .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get pigeon name.".into())
            }
        }
    }

    pub fn add_pooped_on_count(human_id: i32) -> Result<(), String> {
        let connection = get_connection_diesel()?;

        match sql_query(include_str!("queries/pigeon/add_pooped_on_count.sql"))
            .bind::<Integer, _>(human_id)
            .execute(&connection) {
                Ok(_) => Ok(()),
                Err(e) => {
                    println!("{:?}", e);
                    Err("Failed to add pooped on count.".into())
                }
            }
    }

    pub fn jail(human_id: i32, hours: i32) -> Result<(), String> {
        let connection = get_connection_diesel()?;

        match sql_query(include_str!("queries/pigeon/jail.sql"))
            .bind::<Integer, _>(hours)
            .bind::<Integer, _>(human_id)
            .execute(&connection) {
                Ok(_) => Ok(()),
                Err(e) => {
                    println!("{:?}", e);
                    Err("Failed to".into())
            }
        }
    }

    pub fn set_pvp_action_used(human_id: i32) -> Result<(), String> {
        let connection = get_connection_diesel()?;

        match sql_query(include_str!("queries/pigeon/set_pvp_action_used.sql"))
            .bind::<Integer, _>(human_id)
            .execute(&connection) {
                Ok(_) => Ok(()),
                Err(e) => {
                    println!("{:?}", e);
                    Err("Failed to".into())
                }
            }
    }

    pub fn add_poop_victim_count(human_id: i32) -> Result<(), String> {
        let connection = get_connection_diesel()?;

        match sql_query(include_str!("queries/pigeon/add_poop_victim_count.sql"))
            .bind::<Integer, _>(human_id)
            .execute(&connection) {
                Ok(_) => Ok(()),
                Err(e) => {
                    println!("{:?}", e);
                    Err("Failed to add_poop_victim_count".into())
                }
            }
    }

    pub fn get_decaying_pigeons() -> Result<Vec<DecayingPigeon>, String> {
        let connection = get_connection_diesel()?;

        let results: Result<Vec<DecayingPigeon>, _> =
            sql_query(include_str!("queries/pigeon/get_decaying_pigeons.sql"))
                .get_results(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get decaying pigeons.".into())
            }
        }
    }

    pub fn get_profile(human_id: i32) -> Result<PigeonProfile, String> {
        let connection = get_connection_diesel()?;

        let results: Result<PigeonProfile, _> =
            sql_query(include_str!("queries/pigeon/get_profile.sql"))
                .bind::<Integer, _>(human_id)
                .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get pigeon profile.".into())
            }
        }
    }

    pub fn get_stat_value(human_id: i32, stat_name: &str) -> Result<PigeonStatValue, String> {
        let connection = get_connection_diesel()?;

        let results: Result<PigeonStatValue, _> = sql_query(format!(
            "SELECT {} as value FROM pigeon WHERE human_id = ? AND `condition` = 'active' LIMIT 1",
            stat_name
        ))
        .bind::<Integer, _>(human_id)
        .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get stat value.".into())
            }
        }
    }

    pub fn get_gold_modifier(human_id: i32) -> Result<GoldModifier, String> {
        let connection = get_connection_diesel()?;

        let results: Result<GoldModifier, _> =
            sql_query(include_str!("queries/pigeon/get_gold_modifier.sql"))
                .bind::<Integer, _>(human_id)
                .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get gold modifier.".into())
            }
        }
    }

    pub fn increase_gold_modifier(human_id: i32, value: f64) -> Result<(), String> {
        let connection = get_connection_diesel()?;

        match sql_query(include_str!("queries/pigeon/increase_gold_modifier.sql"))
            .bind::<Double, _>(value)
            .bind::<Integer, _>(human_id)
            .execute(&connection) {
                Ok(_) => Ok(()),
                Err(_) => Err("Failed to increase gold modifier".into())
            }
    }

    pub fn update_status(human_id: i32, status: PigeonStatus) -> Result<(), &'static str> {
        let connection = get_connection_diesel()?;

        match sql_query(include_str!("queries/pigeon/update_status.sql"))
            .bind::<Varchar, _>(status.to_string())
            .bind::<Integer, _>(human_id)
            .execute(&connection) {
                Ok(_) => Ok(()),
                Err(_) => Err("Failed to update status".into())
            }
    }

    pub fn update_death_notified(human_id: i32, value: bool) -> Result<(), &'static str> {
        let connection = get_connection_diesel()?;

        match sql_query(include_str!("queries/pigeon/update_death_notified.sql"))
            .bind::<Bool, _>(value)
            .bind::<Integer, _>(human_id)
            .execute(&connection) {
                Ok(_) => Ok(()),
                Err(_) => Err("Failed to ".into())
            }
    }

    pub fn create(human_id: i32, name: &str) -> Result<(), &'static str> {
        let connection = get_connection_diesel()?;

        match sql_query(include_str!("queries/pigeon/create.sql"))
            .bind::<Varchar, _>(name)
            .bind::<Integer, _>(human_id)
            .execute(&connection) {
                Ok(_) => Ok(()),
                Err(_) => Err("Failed to create pigeon".into())
            }
    }
}
