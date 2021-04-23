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

type HumanResult = Result<Human, &'static str>;
pub struct HumanRepository;

impl HumanRepository {
    pub fn get_or_create(user_id: u64) -> HumanResult {
        if let Ok(human) = HumanRepository::get(user_id) {
            return Ok(human);
        } else {
            return HumanRepository::create_for(user_id);
        }
    }

    pub fn get(uid: u64) -> HumanResult {
        use crate::database::schema::human::dsl::*;
        use diesel::prelude::*;

        let connection = get_connection_diesel();
        human
            .filter(user_id.eq(uid))
            .first::<Human>(&connection)
            .map_err(|_| "Human not found.")
    }

    pub fn create_for(user_id: u64) -> HumanResult {
        use diesel::prelude::*;
        let new_human = NewHuman {
            user_id,
            gold: 250,
            ..Default::default()
        };
        let conn = get_connection_diesel();
        diesel::insert_into(human::table)
            .values(&new_human)
            .execute(&conn)
            .map_or(Err("Not able to create human."), |_| HumanRepository::get(user_id))
    }

    pub fn create(h: Human) -> HumanResult{
        use diesel::prelude::*;
        let conn = get_connection_diesel();

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
                HumanRepository::get(new_human.user_id)
            })
    }

    pub fn update(h: Human) -> HumanResult {
        use diesel::prelude::*;
        let conn = get_connection_diesel();

        diesel::update(&h)
        .set(&h)
        .execute(&conn)
        .map_or(Err("Not able to update human."), |_| HumanRepository::get(h.user_id))
    }

    pub fn save(h: Human) -> HumanResult {
        return if h.id == 0 {
            HumanRepository::create(h)
        } else {
            HumanRepository::update(h)
        };
    }

}


