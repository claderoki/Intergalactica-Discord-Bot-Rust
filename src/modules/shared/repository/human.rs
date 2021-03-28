use mysql::{params, prelude::Queryable};
use tracing_subscriber::fmt::format;

use crate::{
    database::{connection::get_connection, helpers::general::get_select_rows},
    modules::shared::models::human::Human,
};

pub fn get_or_create_human(user_id: i32) {}

pub fn get_human(user_id: i32) -> Result<Human, &'static str> {
    let mut query = String::from("");
    query.push_str("SELECT * FROM human WHERE user_id = ");
    query.push_str(user_id.to_string().as_str());
    query.push_str(" LIMIT 1");

    let rows = get_select_rows(query.as_str());

    for row in rows {
        return Ok(Human::from_row(row));
    }

    return Err("Human not found.");
}

pub fn create_human(user_id: i32) -> Result<Human, &'static str> {
    let query = "INSERT INTO human (user_id) VALUES (:user_id)";

    match get_connection() {
        Ok(mut conn) => {
            conn.exec::<i64, _, _>(query, params! {"user_id" => user_id});

            if let Ok(human) = get_human(user_id) {
                return Ok(human);
            } else {
                return Err("Not able to create human.");
            }
        }
        Err(_) => {
            return Err("Not able to create human.");
        }
    }
}
