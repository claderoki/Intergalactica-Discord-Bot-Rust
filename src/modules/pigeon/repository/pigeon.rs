use super::super::models::pigeon::Pigeon;
use crate::database::connection::get_connection_diesel;
use crate::{
    database::schema::pigeon,
    modules::pigeon::models::pigeon::{Gender, PigeonCondition, PigeonStatus},
};
use diesel::prelude::*;
use diesel::{backend::Backend, deserialize, types::FromSql};
use diesel::{
    serialize::{self, Output},
    types::{ToSql, Varchar},
};
use diesel::{
    sql_query,
    sql_types::{Integer},
    Connection, RunQueryDsl,
};

use crate::database::{utils::Countable};

type SinglePigeonResult = Result<Pigeon, &'static str>;
// type MultiplePigeonResult = Result<Vec<Pigeon>, &'static str>;
pub struct PigeonRepository;

impl PigeonRepository {
    pub fn get_active(h_id: i32) -> SinglePigeonResult {
        use crate::database::schema::pigeon::dsl::*;
        use diesel::prelude::*;

        let connection = get_connection_diesel();
        pigeon
            .filter(human_id.eq(h_id))
            .filter(condition.eq("active"))
            .first::<Pigeon>(&connection)
            .map_err(|_| "No active pigeon found.")
    }

    pub fn has_active(human_id: i32) -> Result<bool, &'static str> {
        let connection = get_connection_diesel();

        let results: Result<Countable, _> = sql_query(
            "
            SELECT
            COUNT(*) AS count
            FROM
            pigeon
            WHERE `human_id` = ?
            AND `condition` = 'active'
            LIMIT 1
            ",
        )
        .bind::<Integer, _>(human_id)
        .get_result(&connection);

        match results {
            Ok(data) => Ok(data.count > 0),
            Err(_) => {
                Err("pigeon wtf")
            },
        }
    }


    pub fn create(human_id: i32, name: &str) -> Result<(), &'static str> {
        let new_pigeon = NewPigeon {
            name: name.into(),
            human_id,
        };
        let conn = get_connection_diesel();
        diesel::insert_into(pigeon::table)
            .values(&new_pigeon)
            .execute(&conn).or(Err(""))?;
        Ok(())
    }
}

#[derive(Insertable)]
#[table_name = "pigeon"]
pub struct NewPigeon {
    pub name: String,
    pub human_id: i32,
}

impl<DB> ToSql<Varchar, DB> for PigeonStatus
where
    DB: Backend,
    str: ToSql<Varchar, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: std::io::Write,
    {
        self.to_string().as_str().to_sql(out)
    }
}

impl<DB: Backend> FromSql<Varchar, DB> for PigeonStatus
where
    String: FromSql<Varchar, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let value = String::from_sql(bytes)?;
        Ok(Self::from_str(&value))
    }
}
impl<DB> ToSql<Varchar, DB> for Gender
where
    DB: Backend,
    str: ToSql<Varchar, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: std::io::Write,
    {
        self.to_string().as_str().to_sql(out)
    }
}

impl<DB: Backend> FromSql<Varchar, DB> for Gender
where
    String: FromSql<Varchar, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let value = String::from_sql(bytes)?;
        Ok(Self::from_str(&value))
    }
}

impl<DB> ToSql<Varchar, DB> for PigeonCondition
where
    DB: Backend,
    str: ToSql<Varchar, DB>,
{
    fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
    where
        W: std::io::Write,
    {
        self.to_string().as_str().to_sql(out)
    }
}

impl<DB: Backend> FromSql<Varchar, DB> for PigeonCondition
where
    String: FromSql<Varchar, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let value = String::from_sql(bytes)?;
        Ok(Self::from_str(&value))
    }
}
