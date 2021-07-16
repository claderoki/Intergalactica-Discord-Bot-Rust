use redis::Commands;

use crate::redis_utils::connection::get_connection_redis;

pub struct HumanCache;
impl HumanCache {
    pub fn get_key(user_id: u64) -> String {
        format!("human:{}", user_id)
    }

    pub fn get_id(user_id: u64) -> Option<i32> {
        let mut connection = get_connection_redis();
        let id: Result<i32, _> = connection.get(HumanCache::get_key(user_id).as_str());
        id.ok()
    }

    pub fn cache_id(user_id: u64, human_id: i32) -> Result<(), &'static str> {
        let mut connection = get_connection_redis();

        let result: Result<(), _> = connection.set(HumanCache::get_key(user_id).as_str(), human_id);
        if result.is_err() {
            return Err("error");
        }
        Ok(())
    }
}
