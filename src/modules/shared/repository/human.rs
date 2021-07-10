use crate::database::{schema::human, utils::Countable};
use crate::modules::shared::models::human::Human;
use diesel::{sql_query, sql_types::Integer, RunQueryDsl};

use crate::database::connection::get_connection_diesel;

#[derive(Insertable, Default)]
#[table_name = "human"]
pub struct NewHuman {
    pub user_id: u64,
    pub gold: i32,
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

    fn update_gold(human_id: i32, amount: i32) -> Result<(), &'static str> {
        let connection = get_connection_diesel();
        let result = sql_query("UPDATE human SET gold = gold + ? WHERE id = ?")
            .bind::<Integer, _>(amount)
            .bind::<Integer, _>(human_id)
            .execute(&connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Could not update item"),
        }
    }

    pub fn has_gold(human_id: i32, min_amount: i32) -> Result<bool, &'static str> {
        let connection = get_connection_diesel();

        let results: Result<Countable, _> = sql_query(
            "
            SELECT
            COUNT(*) AS count
            FROM
            human
            WHERE gold >= ?
            AND id = ?
            LIMIT 1
            ",
        )
        .bind::<Integer, _>(min_amount)
        .bind::<Integer, _>(human_id)
        .get_result(&connection);

        match results {
            Ok(data) => Ok(data.count > 0),
            Err(e) => {
                println!("{:?}", e);
                Err("idk wtf is wrong")
            }
        }
    }

    pub fn spend_gold(human_id: i32, amount: i32) -> Result<(), &'static str> {
        HumanRepository::update_gold(human_id, -amount)
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
            .map_or(Err("Not able to create human."), |_| {
                HumanRepository::get(user_id)
            })
    }
}
