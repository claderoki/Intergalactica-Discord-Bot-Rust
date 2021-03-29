use super::super::models::pigeon::Pigeon;
use mysql::{params, prelude::Queryable};

use crate::database::{connection::get_connection, helpers::general::get_select_rows};

pub fn get_active_pigeon(human_id: i32) -> Result<Pigeon, &'static str> {
    //TODO: get rid of sql injection point.
    let query = format!(
        "SELECT * FROM pigeon WHERE human_id = {} AND `condition` = 'active' LIMIT 1",
        human_id
    );
    let rows = get_select_rows(query.as_str());

    for row in rows {
        return Ok(Pigeon::from_row(row));
    }

    return Err("No active pigeon found.");
}

pub fn create_pigeon(human_id: i32, name: &str) -> Result<Pigeon, &'static str> {
    let query = "INSERT INTO pigeon (human_id, name) VALUES (:human_id, :name)";

    match get_connection() {
        Ok(mut conn) => {
            conn.exec::<i64, _, _>(query, params! {"human_id" => human_id, name});

            let pigeon = get_active_pigeon(human_id)?;
            return Ok(pigeon);
        }
        Err(_) => {
            return Err("Not able to create pigeon.");
        }
    }
}
