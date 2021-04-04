use crate::database::connection::get_connection_diesel;
use crate::database::schema::human;
use crate::modules::shared::models::human::Human;

#[derive(Insertable, Default)]
#[table_name = "human"]
pub struct NewHuman {
    pub user_id: u64,
    pub gold: i32,
    pub timezone: Option<String>,
    pub date_of_birth: Option<String>,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub tester: bool,
    pub currencies: Option<String>,
}

pub fn get_or_create_human(user_id: u64) -> Result<Human, &'static str> {
    if let Ok(human) = get_human(user_id) {
        return Ok(human);
    } else {
        return create_human(user_id);
    }
}

pub fn get_human(uid: u64) -> Result<Human, &'static str> {
    //TODO: get rid of sql injection point.
    use crate::database::schema::human::dsl::*;
    use diesel::prelude::*;

    let connection = get_connection_diesel();
    human
        .filter(user_id.eq(uid))
        .first::<Human>(&connection)
        .map_err(|_| "Human not found.")
}

pub fn create_human(user_id: u64) -> Result<Human, &'static str> {
    use diesel::prelude::*;

    let new_human = NewHuman {
        user_id,
        gold: 0, // The Initial gold amount
        ..Default::default()
    };
    let conn = get_connection_diesel();
    diesel::insert_into(human::table)
        .values(&new_human)
        .execute(&conn)
        .map_or(Err("Not able to create human."), |_| get_human(user_id))
}

pub fn save_human(h: Human) -> Result<Human, &'static str> {
    use diesel::prelude::*;
    let conn = get_connection_diesel();

    return if h.id == 0 {
        // TODO do this in a cleaner way
        let new_human = NewHuman {
            user_id: h.user_id,
            gold: h.gold,
            timezone: h.timezone,
            date_of_birth: h.date_of_birth,
            city: h.city,
            country_code: h.country_code,
            tester: h.tester,
            currencies: h.currencies,
        };
        diesel::insert_into(human::table)
            .values(&new_human)
            .execute(&conn)
            .map_or(Err("Not able to create human."), |_| {
                get_human(new_human.user_id)
            })
    } else {
        diesel::update(&h)
            .set(&h)
            .execute(&conn)
            .map_or(Err("Not able to update human."), |_| get_human(h.user_id))
    };
}
