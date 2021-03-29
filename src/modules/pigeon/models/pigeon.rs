use mysql::{from_row, Row};

pub enum PigeonStatus {
    Idle,
    Mailing,
    Exploring,
    Fighting,
    Dating,
}

impl PigeonStatus {
    pub fn from_str(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "idle" => Self::Idle,
            "mailing" => Self::Mailing,
            "exploring" => Self::Exploring,
            "fighting" => Self::Fighting,
            "dating" => Self::Dating,
            _ => Self::Idle,
        }
    }
}
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
}

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
}
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

type PigeonType = (
    i32,
    String,
    i32,
    String,
    i32,
    i32,
    i32,
    i32,
    i32,
    String,
    String,
);

impl Pigeon {
    pub fn from_row(row: Row) -> Pigeon {
        let values = from_row::<PigeonType>(row);
        Pigeon {
            id: values.0,
            name: values.1,
            human_id: values.2,
            condition: PigeonCondition::from_str(values.3.as_str()),
            experience: values.4,
            cleanliness: values.5,
            happiness: values.6,
            food: values.7,
            health: values.8,
            status: PigeonStatus::from_str(values.9.as_str()),
            gender: Gender::from_str(values.10.as_str()),
        }
    }
}
