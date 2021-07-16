use redis::Commands;
use chrono::{Duration, NaiveDateTime};

use crate::redis_utils::connection::get_connection_redis;

pub trait Flag {
    fn get_key() -> String;
    fn new(when: NaiveDateTime) -> Self;
}

pub struct PigeonLastHealed {
    pub when: NaiveDateTime
}
impl Flag for PigeonLastHealed {
    fn get_key() -> String {
        "pigeon_last_healed".into()
    }

    fn new(when: NaiveDateTime) -> Self {
        Self {
            when: when
        }
    }
}
pub struct PigeonLastFed {
    pub when: NaiveDateTime
}
impl Flag for PigeonLastFed {
    fn get_key() -> String {
        "pigeon_last_fed".into()
    }

    fn new(when: NaiveDateTime) -> Self {
        Self {
            when: when
        }
    }
}
pub struct PigeonLastCleaned {
    pub when: NaiveDateTime
}
impl Flag for PigeonLastCleaned {
    fn get_key() -> String {
        "pigeon_last_cleaned".into()
    }

    fn new(when: NaiveDateTime) -> Self {
        Self {
            when: when
        }
    }
}
pub struct PigeonLastPlayedWith {
    pub when: NaiveDateTime
}
impl Flag for PigeonLastPlayedWith {
    fn get_key() -> String {
        "pigeon_last_played_with".into()
    }

    fn new(when: NaiveDateTime) -> Self {
        Self {
            when: when
        }
    }
}

pub struct FlagValidator;
impl FlagValidator {
    pub fn validate<T>(human_id: i32, duration: Duration) -> Result<NaiveDateTime, String> where T: Flag {
        let now = chrono::offset::Utc::now().naive_utc();
        if let Some(flag) = FlagCache::get::<PigeonLastPlayedWith>(human_id) {
            let difference = flag.when - now;
            if difference <= duration {
                return Err(format!("You can only use this command every ..."));
            }
        }
        Ok(now)
    }
}

const DT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub struct FlagCache;
impl FlagCache {
    fn get_key<T>(human_id: i32) -> String where T: Flag {
        format!("flags:{}:{}", human_id, T::get_key())
    }

    pub fn get<T>(human_id: i32) -> Option<T> where T: Flag {
        let mut connection = get_connection_redis();

        let value: Result<String, _> = connection.get(&FlagCache::get_key::<T>(human_id));
        match value {
            Ok(v) => Some(T::new(NaiveDateTime::parse_from_str(&v, DT_FORMAT).unwrap())),
            Err(_) => None
        }
    }

    pub fn add<T>(human_id: i32, when: NaiveDateTime) -> bool where T: Flag {
        let mut connection = get_connection_redis();

        let result: Result<(), _> = connection.set(&FlagCache::get_key::<T>(human_id), when.format(DT_FORMAT).to_string());
        result.is_ok()
    }

}
