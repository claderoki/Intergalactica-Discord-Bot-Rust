use diesel::{backend::Backend, deserialize, serialize::{self, Output}, sql_types::{Integer, VarChar}, types::{FromSql, ToSql, Varchar}};


#[derive(Debug, Clone, Copy, FromSqlRow)]
pub enum PigeonStatus {
    Idle,
    Mailing,
    Exploring,
    Fighting,
    Dating,
    SpaceExploring,
}

impl PigeonStatus {
    pub fn from_str(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "idle" => Self::Idle,
            "mailing" => Self::Mailing,
            "exploring" => Self::Exploring,
            "space_exploring" => Self::SpaceExploring,
            "fighting" => Self::Fighting,
            "dating" => Self::Dating,
            _ => Self::Idle,
        }
    }

    pub fn get_friendly_verb(self) -> String {
        String::from(match self {
            PigeonStatus::Idle => "being lazy",
            PigeonStatus::Mailing => "sending a mail",
            PigeonStatus::Exploring => "exploring",
            PigeonStatus::Fighting => "in a fight",
            PigeonStatus::SpaceExploring => "exploring space",
            PigeonStatus::Dating => "on a date",
        })
    }

    pub fn to_string(&self) -> String {
        String::from(match *self {
            PigeonStatus::Idle => "idle",
            PigeonStatus::Mailing => "mailing",
            PigeonStatus::Exploring => "exploring",
            PigeonStatus::Fighting => "fighting",
            PigeonStatus::SpaceExploring => "space_exploring",
            PigeonStatus::Dating => "dating",
        })
    }
}
#[derive(Debug, Clone, Copy, FromSqlRow)]
pub enum PigeonCondition {
    Active,
    RanAway,
    Dead,
}

impl PigeonCondition {
    pub fn from_str(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "active" => Self::Active,
            "ran_away" => Self::RanAway,
            "dead" => Self::Dead,
            _ => Self::Active,
        }
    }

    pub fn to_string(&self) -> String {
        String::from(match *self {
            PigeonCondition::Active => "active",
            PigeonCondition::RanAway => "ran_away",
            PigeonCondition::Dead => "dead",
        })
    }
}

#[derive(Debug, Clone, Copy, FromSqlRow)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl Gender {
    pub fn from_str(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "male" => Self::Male,
            "female" => Self::Female,
            "other" => Self::Other,
            _ => Self::Other,
        }
    }

    pub fn to_string(&self) -> String {
        String::from(match *self {
            Gender::Male => "male",
            Gender::Female => "female",
            Gender::Other => "other,"
        })
    }
}
#[derive(Queryable)]
pub struct Pigeon {
    pub id: i32,
    pub name: String,
    pub human_id: i32,
    pub condition: PigeonCondition,
    pub experience: i32,
    pub cleanliness: i32,
    pub happiness: i32,
    pub food: i32,
    pub health: i32,
    pub status: PigeonStatus,
    pub gender: Gender,
}

#[derive(QueryableByName)]
pub struct PigeonProfile {
    #[sql_type = "VarChar"]
    pub name: String,

    #[sql_type = "Integer"]
    pub health: i32,

    #[sql_type = "Integer"]
    pub happiness: i32,

    #[sql_type = "Integer"]
    pub cleanliness: i32,

    #[sql_type = "Integer"]
    pub experience: i32,

    #[sql_type = "Integer"]
    pub food: i32,

    #[sql_type = "VarChar"]
    pub status: PigeonStatus,
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
