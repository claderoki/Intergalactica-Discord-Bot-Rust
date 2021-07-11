use diesel::sql_types::BigInt;
use diesel::sql_types::Bool;
use diesel::sql_types::Integer;
use diesel::sql_types::Nullable;
use diesel::sql_types::VarChar;

use crate::modules::pigeon::helpers::utils::PigeonWinnings;
use crate::modules::pigeon::helpers::utils::PigeonWinningsBuilder;
use crate::modules::shared::helpers::chooser::Choosable;

use super::pigeon::PigeonStatus;

#[derive(QueryableByName)]
pub struct Exploration {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "VarChar"]
    pub pigeon_status: PigeonStatus,

    #[sql_type = "Integer"]
    pub location_id: i32,

    #[sql_type = "Bool"]
    pub arrived: bool,

    #[sql_type = "Integer"]
    pub actions_remaining: i32,

    #[sql_type = "Integer"]
    pub total_actions: i32,

    #[sql_type = "BigInt"]
    pub remaining_seconds: i64,

    #[sql_type = "BigInt"]
    pub percentage: i64,
}

#[derive(QueryableByName)]
pub struct ExplorationActionScenario {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "VarChar"]
    pub text: String,

    #[sql_type = "Integer"]
    pub winnings_id: i32,
}

#[derive(QueryableByName)]
pub struct ExplorationAction {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "VarChar"]
    pub name: String,

    #[sql_type = "VarChar"]
    pub symbol: String,
}

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

#[derive(QueryableByName)]
pub struct ExplorationActionScenarioWinnings {
    #[sql_type = "BigInt"]
    pub gold: i64,

    #[sql_type = "Integer"]
    pub health: i32,

    #[sql_type = "Integer"]
    pub happiness: i32,

    #[sql_type = "Integer"]
    pub cleanliness: i32,

    #[sql_type = "Integer"]
    pub experience: i32,

    #[sql_type = "Integer"]
    pub food: i32,

    #[sql_type = "Nullable<Integer>"]
    pub item_id: Option<i32>,

    #[sql_type = "Nullable<Integer>"]
    pub item_category_id: Option<i32>,
}

#[derive(QueryableByName)]
pub struct ExplorationEndStats {
    #[sql_type = "BigInt"]
    pub gold: i64,

    #[sql_type = "BigInt"]
    pub health: i64,

    #[sql_type = "BigInt"]
    pub happiness: i64,

    #[sql_type = "BigInt"]
    pub cleanliness: i64,

    #[sql_type = "BigInt"]
    pub experience: i64,

    #[sql_type = "BigInt"]
    pub food: i64,

    #[sql_type = "BigInt"]
    pub total_seconds: i64,

    #[sql_type = "Nullable<VarChar>"]
    pub item_ids: Option<String>,
}

impl ExplorationEndStats {
    pub fn to_pigeon_winnings(&self) -> PigeonWinnings {
        let mut winnings = PigeonWinningsBuilder::new();

        winnings
            .food(self.food as i32)
            .gold(self.gold as i32)
            .happiness(self.happiness as i32)
            .cleanliness(self.cleanliness as i32)
            .health(self.health as i32)
            .experience(self.experience as i32);

        match &self.item_ids {
            Some(ids_str) => {
                for id in ids_str.split(",").map(|i| i.parse::<i32>().unwrap()) {
                    winnings.add_item_id(id);
                }
            }
            _ => {}
        }

        winnings.build()
    }
}

impl ExplorationActionScenarioWinnings {
    pub fn to_pigeon_winnings(&self) -> PigeonWinnings {
        let mut winnings = PigeonWinningsBuilder::new();

        winnings
            .food(self.food as i32)
            .gold(self.gold as i32)
            .happiness(self.happiness as i32)
            .cleanliness(self.cleanliness as i32)
            .health(self.health as i32)
            .experience(self.experience as i32);

        match self.item_id {
            Some(item_id) => {
                winnings.add_item_id(item_id);
            }
            _ => {}
        }

        winnings.build()
    }
}

#[derive(QueryableByName)]
pub struct SimplePlanetLocation {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "VarChar"]
    pub image_url: String,
}

#[derive(QueryableByName)]
pub struct PlanetLocation {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "VarChar"]
    pub planet_name: String,

    #[sql_type = "VarChar"]
    pub location_name: String,

    #[sql_type = "VarChar"]
    pub image_url: String,
}
