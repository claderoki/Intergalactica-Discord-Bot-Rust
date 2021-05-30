use diesel::{
    sql_query,
    sql_types::{BigInt, Bool, Integer, Nullable, VarChar},
    RunQueryDsl,
};

use crate::{database::connection::get_connection_diesel, modules::pigeon::helpers::utils::PigeonWinnings};

pub fn seconds_to_text(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    time_to_text(seconds, minutes, hours)
}

pub fn time_to_text(seconds: i64, minutes: i64, hours: i64) -> String {
    let mut text = String::from("");

    if hours > 0 {
        text.push_str(&format!("{} hours", hours));
    }
    if minutes > 0 {
        if hours > 0 {
            text.push_str(" and ");
        }
        text.push_str(&format!("{} minutes", minutes));
    }

    if seconds > 0 && hours == 0 {

        if minutes > 0 {
            text.push_str(" and ");
        }
        text.push_str(&format!("{} seconds", seconds));
    }

    text

}

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

pub struct ExplorationRepository;
impl ExplorationRepository {

    pub fn reduce_action_remaining(exploration_id: i32) {
        let connection = get_connection_diesel();

        let _results = sql_query("
            UPDATE `exploration`
            SET
                `exploration`.`actions_remaining` = `exploration`.`actions_remaining` - 1
            WHERE `exploration`.`id` = ?"
        )
            .bind::<Integer, _>(exploration_id)
            .execute(&connection);
    }

    pub fn add_exploration_winnings(exploration_id: i32, action_id: i32, winnings: &PigeonWinnings) {
        let connection = get_connection_diesel();
        let item_id: Option<i32> = None;

        let results = sql_query(
            "INSERT INTO exploration_winnings
            (gold, health, experience, cleanliness, food, happiness, item_id, exploration_id, exploration_action_id)
            VALUES
            (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind::<Integer, _>(winnings.gold)
        .bind::<Integer, _>(winnings.health)
        .bind::<Integer, _>(winnings.experience)
        .bind::<Integer, _>(winnings.cleanliness)
        .bind::<Integer, _>(winnings.food)
        .bind::<Integer, _>(winnings.happiness)
        .bind::<Nullable<Integer>, _>(item_id)
        .bind::<Integer, _>(exploration_id)
        .bind::<Integer, _>(action_id)
        .execute(&connection);

        match results {
            Ok(_) => {},
            Err(e) => {
                println!("{:?}", e);
            }
        };
    }

    pub fn get_scenario_winnings(scenario_id: i32) -> Result<ExplorationActionScenarioWinnings, &'static str> {
        let connection = get_connection_diesel();

        let query = "
            SELECT
            gold, health, happiness, cleanliness, experience, food, item_id, item_category_id
            FROM
            exploration_action_scenario_winnings
            WHERE exploration_action_scenario_id = ?
            LIMIT 1";

        let results: Result<ExplorationActionScenarioWinnings, _> = sql_query(query)
            .bind::<Integer, _>(scenario_id)
            .get_result(&connection);

        match results {
            Ok(data) => {
                Ok(data)
            },
            Err(e) => {
                println!("{:?}", e);
                Err("Something went wrong retrieving the winnings.")
            }
        }
    }

    pub fn get_scenario(action_id: i32) -> Result<ExplorationActionScenario, &'static str> {
        let connection = get_connection_diesel();

        let query = "
            SELECT
            id, text
            FROM
            exploration_action_scenario
            WHERE action_id = ?
            ORDER BY RAND()
            LIMIT 1";

        let results: Result<ExplorationActionScenario, _> = sql_query(query)
            .bind::<Integer, _>(action_id)
            .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                println!("{:?}", e);
                Err("Something went wrong retrieving the activity.")
            }
        }
    }

    pub fn get_exploration(human_id: i32) -> Result<Exploration, &'static str> {
        let connection = get_connection_diesel();

        let query = "SELECT
            a.id as id,
            pigeon.status as pigeon_status,
            (a.end_date <= UTC_TIMESTAMP()) as arrived,
            actions_remaining,
            @remaining := ABS(TIME_TO_SEC(TIMEDIFF(UTC_TIMESTAMP(), end_date))) AS remaining_seconds,
            CAST(ABS(((@remaining / TIME_TO_SEC(TIMEDIFF(start_date, end_date)) * 100)-100)) AS INT) as percentage,
            planet_location_id as location_id
            FROM
            pigeon
            INNER JOIN exploration a ON a.pigeon_id = pigeon.id AND a.finished = 0
            WHERE pigeon.human_id = ?";

        let results: Result<Exploration, _> = sql_query(query)
            .bind::<Integer, _>(human_id)
            .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                println!("{:?}", e);
                Err("Something went wrong retrieving the exploration.")
            }
        }
    }

    pub fn get_available_actions(location_id: i32) -> Result<Vec<ExplorationAction>, &'static str> {
        let connection = get_connection_diesel();

        let query = "
            SELECT
            id,
            name,
            symbol
            FROM
            exploration_action
            WHERE exploration_planet_location_id = ?";

        let results: Result<Vec<ExplorationAction>, _> = sql_query(query)
            .bind::<Integer, _>(location_id)
            .get_results(&connection);

        return match results {
            Ok(data) => Ok(data),
            Err(e) => {
                println!("{:?}", e);
                Err("Something went wrong retrieving the actions.")
            }
        };
    }

    // pub fn get_any_activity(human_id: i32) -> Result<Option<PigeonSimplifiedActivity>, &'static str> {
    //     let connection = get_connection_diesel();

    //     let results: Result<PigeonSimplifiedActivity, _> = sql_query(
    //         "SELECT
    //             coalesce(exploration.id, mail.id, fight.id, date.id) as id,
    //             pigeon.status as type,
    //             (coalesce(exploration.end_date, mail.end_date, fight.end_date, date.end_date) <= UTC_DATE() ) as ready_to_be_retrieved
    //         FROM
    //         pigeon
    //         LEFT JOIN exploration
    //             ON (exploration.pigeon_id = pigeon.id AND exploration.finished = 0 AND pigeon.status = 'exploring')
    //         LEFT JOIN mail
    //             ON (mail.sender_id = pigeon.id AND mail.finished = 0 AND pigeon.status = 'mailing')
    //         LEFT JOIN fight
    //             ON ((fight.pigeon1_id = pigeon.id OR fight.pigeon2_id = pigeon.id) AND fight.finished = 0 AND pigeon.status = 'fighting')
    //         LEFT JOIN date
    //             ON ((date.pigeon1_id = pigeon.id OR date.pigeon2_id = pigeon.id) AND date.finished = 0  AND pigeon.status = 'dating')
    //         WHERE pigeon.human_id = ?")
    //     .bind::<Integer, _>(human_id)
    //     .get_result(&connection);

    //     match results {
    //         Ok(data) => {
    //             if data.id.is_none() {
    //                 return Ok(None);
    //             }
    //             Ok(Some(data))
    //         },
    //         Err(_) => {
    //             Err("Something went wrong retrieving the activity.")
    //         },
    //     }
    // }
}
