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

pub fn get_active_pigeon(hid: i32) -> Result<Pigeon, &'static str> {
    //TODO: get rid of sql injection point.
    use crate::database::schema::pigeon::dsl::*;
    use diesel::prelude::*;

    let connection = get_connection_diesel();
    pigeon
        .filter(human_id.eq(hid))
        .filter(condition.eq("active"))
        .first::<Pigeon>(&connection)
        .map_err(|_| "No active pigeon found.")
}

pub fn create_pigeon(human_id: i32, name: &str) -> Result<Pigeon, &'static str> {
    let new_pigeon = NewPigeon {
        name: name.into(),
        human_id,
    };
    let conn = get_connection_diesel();
    diesel::insert_into(pigeon::table)
        .values(&new_pigeon)
        .execute(&conn)
        .or(Err("Sorry, we were not able to get you a pigeon."))?;

    let pigeon = get_active_pigeon(human_id)?;
    Ok(pigeon)
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
        let v = match *self {
            Self::Idle => "idle",
            Self::Mailing => "mailing",
            Self::Exploring => "exploring",
            Self::Fighting => "fighting",
            Self::Dating => "dating",
        };
        v.to_sql(out)
    }
}

impl<DB: Backend> FromSql<Varchar, DB> for PigeonStatus
where
    String: FromSql<Varchar, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let v = String::from_sql(bytes)?;
        Ok(match v.as_str() {
            "idle" => Self::Idle,
            "mailing" => Self::Mailing,
            "exploring" => Self::Exploring,
            "fighting" => Self::Fighting,
            "dating" => Self::Dating,
            &_ => Self::Idle,
        })
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
        let v = match *self {
            Self::Male => "male",
            Self::Female => "female",
            Self::Other => "other",
        };
        v.to_sql(out)
    }
}

impl<DB: Backend> FromSql<Varchar, DB> for Gender
where
    String: FromSql<Varchar, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let v = String::from_sql(bytes)?;
        Ok(match v.as_str() {
            "male" => Self::Male,
            "female" => Self::Female,
            "other" => Self::Other,
            &_ => Self::Other,
        })
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
        let v = match *self {
            Self::Active => "active",
            Self::RanAway => "ran_away",
            Self::Dead => "dead",
        };
        v.to_sql(out)
    }
}

impl<DB: Backend> FromSql<Varchar, DB> for PigeonCondition
where
    String: FromSql<Varchar, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let v = String::from_sql(bytes)?;
        Ok(match v.as_str() {
            "active" => Self::Active,
            "ran_away" => Self::RanAway,
            "dead" => Self::Dead,
            &_ => Self::Active,
        })
    }
}
