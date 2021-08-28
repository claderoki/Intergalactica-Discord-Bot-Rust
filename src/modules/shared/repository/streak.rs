use diesel::sql_query;
use diesel::sql_types::BigInt;
use diesel::sql_types::Integer;
use diesel::sql_types::VarChar;

use diesel::RunQueryDsl;

use crate::database::connection::get_connection_diesel;

#[derive(QueryableByName, Debug)]
pub struct Streak {
    #[sql_type = "Integer"]
    pub current: i32,

    #[sql_type = "BigInt"]
    pub days_missed: i64,
}

pub struct StreakRepository;
impl StreakRepository {
    pub fn add(human_id: i32, key: &'static str) -> Result<(), String> {
        let connection = get_connection_diesel()?;

        let result = sql_query(include_str!("queries/streak/add.sql"))
            .bind::<VarChar, _>(key)
            .bind::<Integer, _>(human_id)
            .execute(&connection);

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err("Could not add streak".into())
            }
        }
    }
    pub fn reset(human_id: i32, key: &'static str) -> Result<(), String> {
        let connection = get_connection_diesel()?;

        let result = sql_query(include_str!("queries/streak/reset.sql"))
            .bind::<VarChar, _>(key)
            .bind::<Integer, _>(human_id)
            .execute(&connection);

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err("Could not reset streak".into())
            }
        }
    }

    pub fn get(human_id: i32, key: &'static str) -> Result<Streak, String> {
        let connection = get_connection_diesel()?;

        let results: Result<Streak, _> = sql_query(include_str!("queries/streak/get.sql"))
            .bind::<VarChar, _>(key)
            .bind::<Integer, _>(human_id)
            .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(_) => Ok(Streak {
                current: 0,
                days_missed: 0,
            }),
        }
    }
}
