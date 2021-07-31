use chrono::Duration;
use chrono::NaiveDateTime;
use redis::Commands;

use crate::modules::shared::helpers::utils::TimeDelta;
use crate::redis_utils::connection::get_connection_redis;

pub trait Flag {
    fn get_key() -> String;
    fn new(datetime: NaiveDateTime) -> Self;
    fn get_datetime(&self) -> NaiveDateTime;
}

#[macro_export]
macro_rules! flags {
    ($($name:ident;)*) => {
        $(
            impl crate::modules::shared::caching::flag::Flag for $name {
                fn get_key() -> String {
                    stringify!($name).into()
                }

                fn new(when: NaiveDateTime) -> Self {
                    Self {
                        0: when
                    }
                }

                fn get_datetime(&self) -> NaiveDateTime {
                    self.0
                }

            }
        )*
    }
}
pub struct FlagValidator;
impl FlagValidator {
    pub fn validate<T>(user_id: u64, duration: Duration) -> Result<NaiveDateTime, String>
    where
        T: Flag,
    {
        let now = chrono::offset::Utc::now().naive_utc();
        if let Some(flag) = FlagCache::get::<T>(user_id) {
            let available_date = flag.get_datetime() + duration;
            if available_date >= now {
                let difference = available_date - now;
                return Err(format!(
                    "Try again in {}",
                    TimeDelta::from_seconds(difference.num_seconds()).to_text()
                ));
            }
        }
        Ok(now)
    }
}

const DT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub struct FlagCache;
impl FlagCache {
    fn get_key<T>(user_id: u64) -> String
    where
        T: Flag,
    {
        format!("flags:{}:{}", user_id, T::get_key())
    }

    pub fn get<T>(user_id: u64) -> Option<T>
    where
        T: Flag,
    {
        match get_connection_redis() {
            Ok(mut connection) => {
                let value: Result<String, _> = connection.get(&FlagCache::get_key::<T>(user_id));
                match value {
                    Ok(v) => Some(T::new(
                        NaiveDateTime::parse_from_str(&v, DT_FORMAT).unwrap(),
                    )),
                    Err(_) => {
                        return None;
                    }
                }
            }
            Err(_) => {
                return None;
            }
        }
    }

    pub fn add<T>(user_id: u64, when: NaiveDateTime) -> bool
    where
        T: Flag,
    {
        match get_connection_redis() {
            Ok(mut connection) => {
                let result: Result<(), _> = connection.set(
                    &FlagCache::get_key::<T>(user_id),
                    when.format(DT_FORMAT).to_string(),
                );
                result.is_ok()
            }
            Err(_) => false,
        }
    }
}

// trait CacheParams {
//     fn get_key() -> String;
//     fn get_value<T>(&self) -> T where T: ToRedisArgs;
// }

// trait RedisValue {
//     fn from(&self) -> String;
//     fn to(value: &str) -> Result<Self, &'static str> where Self: Sized;
// }

// impl RedisValue for NaiveDateTime {
//     fn from(&self) -> String {
//         self.format(DT_FORMAT).to_string()
//     }

//     fn to(value: &str) -> Result<Self, &'static str> {
//         NaiveDateTime::parse_from_str(value, DT_FORMAT).map_err(|_|"Failed to parse.")
//     }
// }

// trait Cache {
//     fn get_key<D>(params: &D) -> String where D: CacheParams;

//     fn get<T, D>(params: &D) -> Option<T>
//     where
//         T: Flag,
//         D: CacheParams
//     {
//         match get_connection_redis() {
//             Ok(mut connection) => {
//                 let value: Result<String, _> = connection.get(Self::get_key::<D>(params));
//                 match value {
//                     Ok(v) => Some(T::new(
//                         NaiveDateTime::parse_from_str(&v, DT_FORMAT).unwrap(),
//                     )),
//                     Err(_) => None,
//                 }
//             },
//             Err(_) => None
//         }
//     }

//     fn add<T, D>(params: D) -> bool where T: Flag, D: CacheParams {
//         match get_connection_redis() {
//             Ok(mut connection) => {

//                 let result: Result<String, _> = connection.set::<String, String, _>(
//                     Self::get_key::<D>(&params),
//                     params.get_value(),
//                 );
//                 result.is_ok()
//             },
//             Err(_) => false
//         }
//     }
// }
