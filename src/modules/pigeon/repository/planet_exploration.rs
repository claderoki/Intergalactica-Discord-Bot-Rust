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

pub struct PlanetExplorationRepository {}

impl PlanetExplorationRepository {
    pub fn create_exploration(location_id: i32) -> Result<(), &'static str> {
        let connection = get_connection_diesel();

        let results = sql_query(
            "INSERT INTO planet_exploration
            (planet_location_id, start_time, end_time, finished)
            VALUES
            (?, UTC_TIMESTAMP(), DATE_ADD(UTC_TIMESTAMP(), INTERVAL ? HOUR), 0)",
        )
        .bind::<Integer, _>(location_id)
        .bind::<Integer, _>(10)
        .execute(&connection);

        return match results {
            Ok(_) => Ok(()),
            Err(_) => Err("Query in PlanetExplorationRepository.create_exploration failed."),
        };
    }

    pub fn get_location() -> Result<SimplePlanetLocation, &'static str> {
        let connection = get_connection_diesel();

        let results: Result<SimplePlanetLocation, _> = sql_query(
            "SELECT
            (`planet_location`.`id`) as id,
            (IFNULL(`planet_location`.`image_url`, `planet`.`image_url`)) as image_url
            FROM
            `planet_location`
            INNER JOIN `planet` ON `planet`.`id` = `planet_location`.`planet_id`
            ORDER BY RAND()
            LIMIT 1",
        )
        .get_result(&connection);

        match results {
            Ok(simple_location) => Ok(simple_location),
            Err(_) => Err("Couldn't find a location."),
        }
    }
}