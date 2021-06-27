use crate::database::connection::get_connection_diesel;
use diesel::{
    sql_query,
    sql_types::{Integer, VarChar},
    RunQueryDsl,
};


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

pub struct PlanetExplorationRepository {}

impl PlanetExplorationRepository {
    pub fn create_exploration(human_id: i32, location_id: i32) -> Result<(), String> {
        let connection = get_connection_diesel();

        let results = sql_query(
            "INSERT INTO exploration
            (planet_location_id, start_date, arrival_date, finished, pigeon_id)
            VALUES
            (
                ?,
                UTC_TIMESTAMP(),
                DATE_ADD(UTC_TIMESTAMP(), INTERVAL ? MINUTE),
                0,
                (SELECT id FROM pigeon WHERE human_id = ? AND `pigeon`.`condition` = 'active' LIMIT 1)
            )",
        )
        .bind::<Integer, _>(location_id)
        .bind::<Integer, _>(30)
        .bind::<Integer, _>(human_id)
        .execute(&connection);

        match results {
            Ok(_) => Ok(()),
            Err(e) => {
                Err(format!("{:?}", e))
            }
        }
    }

    pub fn get_random_location() -> Result<SimplePlanetLocation, String> {
        let connection = get_connection_diesel();

        let results: Result<SimplePlanetLocation, _> = sql_query(
            "SELECT
            `exploration_planet_location`.`id` as id,
            (IFNULL(`exploration_planet_location`.`image_url`, `exploration_planet`.`image_url`)) as image_url
            FROM
            `exploration_planet_location`
            INNER JOIN `exploration_planet` ON `exploration_planet`.`id` = `exploration_planet_location`.`planet_id`
            ORDER BY RAND()
            LIMIT 1",
        )
        .get_result(&connection);

        match results {
            Ok(simple_location) => Ok(simple_location),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn get_location(location_id: i32) -> Result<PlanetLocation, String> {
        let connection = get_connection_diesel();

        let results: Result<PlanetLocation, _> = sql_query(
            "SELECT
            `exploration_planet_location`.`id` as id,
            `exploration_planet`.`name` as planet_name,
            `exploration_planet_location`.`name` as location_name,
            (IFNULL(`exploration_planet_location`.`image_url`, `exploration_planet`.`image_url`)) as image_url
            FROM
            `exploration_planet_location`
            INNER JOIN `exploration_planet` ON `exploration_planet`.`id` = `exploration_planet_location`.`planet_id`
            WHERE `exploration_planet_location`.`id` = ?
            LIMIT 1",
        )
        .bind::<Integer, _>(location_id)
        .get_result(&connection);

        match results {
            Ok(location) => Ok(location),
            Err(e) => Err(format!("{:?}", e))
        }
    }
}