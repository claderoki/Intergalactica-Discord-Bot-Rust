use redis::Commands;

use crate::redis_utils::connection::get_connection_redis;

pub struct HumanCache;
impl HumanCache {
    pub fn get_key(user_id: u64) -> String {
        let mut key = String::from("human:");
        key.push_str(&user_id.to_string());
        key
    }

    pub fn get_id(user_id: u64) -> Option<i32> {
        let connection_result = get_connection_redis();
        if connection_result.is_err() {
            return None;
        }
        let mut conn = connection_result.unwrap();
        let id: Result<i32, _> = conn.get(HumanCache::get_key(user_id).as_str());
        id.ok()
    }

    pub fn cache_id(user_id: u64, human_id: i32) -> Result<(), &'static str> {
        let mut conn = get_connection_redis()?;

        let result: Result<(), _> = conn.set(HumanCache::get_key(user_id).as_str(), human_id);
        if result.is_err() {
            return Err("error");
        }
        Ok(())
    }
}
