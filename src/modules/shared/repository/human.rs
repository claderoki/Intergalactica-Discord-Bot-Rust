use mysql::{params, prelude::Queryable};
use tracing_subscriber::fmt::format;

use crate::{
    database::{connection::get_connection, helpers::general::get_select_rows},
    modules::shared::models::human::Human,
};

pub fn get_or_create_human(user_id: u64) -> Result<Human, &'static str> {
    if let Ok(human) = get_human(user_id) {
        return Ok(human);
    } else {
        return create_human(user_id);
    }
}

pub fn get_human(user_id: u64) -> Result<Human, &'static str> {
    //TODO: get rid of sql injection point.
    let query = format!("SELECT * FROM human WHERE user_id = {} LIMIT 1", user_id);
    let rows = get_select_rows(query.as_str());

    for row in rows {
        return Ok(Human::from_row(row));
    }

    return Err("Human not found.");
}

pub fn create_human(user_id: u64) -> Result<Human, &'static str> {
    let query = "INSERT INTO human (user_id) VALUES (:user_id)";

    match get_connection() {
        Ok(mut conn) => {
            conn.exec::<i64, _, _>(query, params! {"user_id" => user_id});
            let human = get_human(user_id)?;
            return Ok(human);
        }
        Err(_) => {
            return Err("Not able to create human.");
        }
    }
}
