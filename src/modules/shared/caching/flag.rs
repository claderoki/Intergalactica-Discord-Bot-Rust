use redis::Commands;
use chrono::NaiveDateTime;

use crate::redis_utils::connection::get_connection_redis;

pub trait Flag {
    fn get_key() -> String;
    fn new(when: NaiveDateTime) -> Self;
}

// trait CacheKeyBuilder {
//     fn build_key() -> String;
// }

// pub struct FlagCacheKeyBuilder;

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
