use crate::{
    database::{connection::get_connection, helpers::general::get_select_rows},
    modules::shared::models::human::Human,
};
use mysql::{params, prelude::Queryable};

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

pub fn save_human(human: Human) {
    let mut query = String::from("");
    if human.id == 0 {
        query.push_str("INSERT INTO human ");
        query.push_str("(`user_id`, `gold`, `timezone`, `date_of_birth`, `city`, `country_code`, `tester`, `currencies`)");
        query.push_str("VALUES ");
        query.push_str("(:user_id, :gold, :timezone, :date_of_birth, :city, :country_code, :tester, :currencies)");
    } else {
        query.push_str("UPDATE human ");
        query.push_str("`user_id` = :user_id, `gold` = :gold, `timezone` = :timezone, `date_of_birth` = :date_of_birth, `city` = :city, `country_code` = :country_code, `tester` = :tester, `currencies` = :currencies");
        query.push_str("WHERE `id` = :id");
    }

    match get_connection() {
        Ok(mut conn) => {
            let _ = conn.exec::<i64, _, _>(
                query,
                params! {
                    "id" => human.id,
                    "user_id" => human.user_id,
                    "gold" => human.gold,
                    "timezone" => human.timezone,
                    "date_of_birth" => human.date_of_birth,
                    "city" => human.city,
                    "country_code" => human.country_code,
                    "tester" => human.tester,
                    "currencies" => human.currencies,
                },
            );
        }
        Err(e) => {}
    }
}
