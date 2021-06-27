use diesel::{
    sql_types::{BigInt, Bool, Integer, Nullable, VarChar}
};

use crate::{modules::pigeon::helpers::utils::PigeonWinnings};

#[derive(QueryableByName)]
pub struct Exploration {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "VarChar"]
    pub pigeon_status: String,

    #[sql_type = "Integer"]
    pub location_id: i32,

    #[sql_type = "Bool"]
    pub arrived: bool,

    #[sql_type = "Integer"]
    pub actions_remaining: i32,

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
#[derive(QueryableByName)]
pub struct ExplorationActionScenarioWinnings {
    #[sql_type = "Integer"]
    pub gold: i32,

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
    pub total_seconds: i64
}

impl ExplorationEndStats {
    pub fn to_pigeon_winnings(&self) -> PigeonWinnings {
        PigeonWinnings {
            gold: self.gold as i32,
            experience: self.happiness as i32,
            cleanliness: self.health as i32,
            happiness: self.cleanliness as i32,
            food: self.experience as i32,
            health: self.food as i32,
        }
    }
}

impl ExplorationActionScenarioWinnings {
    pub fn to_pigeon_winnings(&self) -> PigeonWinnings {
        PigeonWinnings {
            gold: self.gold,
            experience: self.happiness,
            cleanliness: self.health,
            happiness: self.cleanliness,
            food: self.experience,
            health: self.food,
        }
    }
}