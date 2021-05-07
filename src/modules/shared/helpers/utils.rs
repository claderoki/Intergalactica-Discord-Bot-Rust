use redis::{Commands};
use serenity::{builder::CreateEmbed, model::prelude::{User, UserId}};

use crate::{modules::shared::{caching::human::HumanCache, models::human::Human, repository::human::HumanRepository}, redis_utils::connection::get_connection_redis};

pub trait HumanUtils {
    fn get_human(&self) -> Option<Human>;
    fn get_human_id(&self) -> Option<i32>;
}

impl HumanUtils for User {
    fn get_human(&self) -> Option<Human> {
        self.id.get_human()
    }
    fn get_human_id(&self) -> Option<i32> {
        self.id.get_human_id()
    }
}

impl HumanUtils for UserId {
    fn get_human(&self) -> Option<Human> {
        HumanRepository::get_or_create(*self.as_u64()).ok()
    }
    fn get_human_id(&self) -> Option<i32> {
        let cached_id = HumanCache::get_id(*self.as_u64());
        if cached_id.is_some() {
            return cached_id;
        }

        return match HumanRepository::get_or_create(*self.as_u64()) {
            Ok(human) => {
                let _ = HumanCache::cache_id(*self.as_u64(), human.id);
                Some(human.id)
            },
            Err(_) => {
                None
            }
        }
    }
}