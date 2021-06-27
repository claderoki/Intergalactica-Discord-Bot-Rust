use diesel::{
    sql_query,
    sql_types::{Integer, Nullable},
    RunQueryDsl,
};

use crate::{
    database::connection::get_connection_diesel,
    modules::pigeon::{
        helpers::utils::PigeonWinnings,
        models::exploration::{*},
    },
};

pub struct ExplorationRepository;
impl ExplorationRepository {
    pub fn get_end_stats(exploration_id: i32) -> Result<ExplorationEndStats, String> {
        let connection = get_connection_diesel();

        let query = "
            SELECT
                CAST(SUM(gold) AS SIGNED) as gold,
                CAST(SUM(health) AS SIGNED) as health,
                CAST(SUM(happiness) AS SIGNED) as happiness,
                CAST(SUM(cleanliness) AS SIGNED) as cleanliness,
                CAST(SUM(experience) AS SIGNED) as experience,
                CAST(SUM(food) AS SIGNED) as food,
                TIME_TO_SEC(TIMEDIFF(UTC_TIMESTAMP(), `exploration`.`start_date`)) as total_seconds
            FROM
            exploration_winnings
            INNER JOIN exploration ON exploration.id = exploration_winnings.exploration_id
            WHERE exploration_id = ?
        ";
        let results: Result<ExplorationEndStats, _> = sql_query(query)
            .bind::<Integer, _>(exploration_id)
            .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn reduce_action_remaining(exploration_id: i32) {
        let connection = get_connection_diesel();

        let _results = sql_query(
            "
            UPDATE `exploration`
            SET
                `exploration`.`actions_remaining` = `exploration`.`actions_remaining` - 1
            WHERE `exploration`.`id` = ?",
        )
        .bind::<Integer, _>(exploration_id)
        .execute(&connection);
    }

    pub fn finish_exploration(exploration_id: i32) {
        let connection = get_connection_diesel();

        let _results = sql_query(
            "
            UPDATE `exploration`
            SET
                `exploration`.`end_date` = UTC_TIMESTAMP(),
                `exploration`.`finished` = 1
            WHERE `exploration`.`id` = ?",
        )
        .bind::<Integer, _>(exploration_id)
        .execute(&connection);
    }

    pub fn add_exploration_winnings(
        exploration_id: i32,
        action_id: i32,
        winnings: &PigeonWinnings,
    ) {
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

        let query = "
            SELECT
            gold, health, happiness, cleanliness, experience, food, item_id, item_category_id
            FROM
            exploration_action_scenario_winnings
            WHERE id = ?
            LIMIT 1";

        let results: Result<ExplorationActionScenarioWinnings, _> = sql_query(query)
            .bind::<Integer, _>(winnings_id)
            .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn get_scenario(action_id: i32) -> Result<ExplorationActionScenario, String> {
        let connection = get_connection_diesel();

        let query = "
            SELECT
            id, text, scenario_winnings_id as winnings_id
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
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn get_exploration(human_id: i32) -> Result<Exploration, String> {
        let connection = get_connection_diesel();

        let query = "
            SELECT
            a.id as id,
            pigeon.status as pigeon_status,
            (a.arrival_date <= UTC_TIMESTAMP()) as arrived,
            actions_remaining,
            ABS(TIME_TO_SEC(TIMEDIFF(UTC_TIMESTAMP(), arrival_date))) AS remaining_seconds,
            CAST(ABS(((TIME_TO_SEC(TIMEDIFF(UTC_TIMESTAMP(), arrival_date)) / TIME_TO_SEC(TIMEDIFF(start_date, arrival_date)) * 100)-100)) AS INT) as percentage,
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
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn get_available_actions(location_id: i32) -> Result<Vec<ExplorationAction>, String> {
        let connection = get_connection_diesel();

        let query = "
            SELECT
            id,
            name,
            symbol
            FROM
            exploration_action
            WHERE location_id = ?";

        let results: Result<Vec<ExplorationAction>, _> = sql_query(query)
            .bind::<Integer, _>(location_id)
            .get_results(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
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
