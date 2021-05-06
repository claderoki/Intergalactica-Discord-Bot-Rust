use redis::{Commands};
use serenity::{builder::CreateEmbed, model::prelude::{User, UserId}};

use crate::{modules::shared::{models::human::Human, repository::human::HumanRepository}, redis_utils::connection::get_connection_redis};

pub trait Economy {
    fn pay(&mut self, cost: i32);
    fn has_enough_gold(&self, cost: i32) -> bool;
}

impl Economy for Human {
    fn pay(&mut self, cost: i32) {
        self.gold -= cost
    }

    fn has_enough_gold(&self, cost: i32) -> bool {
        self.gold >= cost
    }
}

struct HumanCache;
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

        let result : Result<(), _> = conn.set(HumanCache::get_key(user_id).as_str(), human_id);
        if result.is_err() {
            return Err("error");
        }
        Ok(())
    }

}

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
pub trait EmbedExtension {
    fn priced_embed(&mut self, text: &str, cost: i32) -> &mut Self;
    fn normal_embed(&mut self, text: &str) -> &mut Self;
    fn error_embed(&mut self, text: &str) -> &mut Self;
}

impl EmbedExtension for CreateEmbed {
    fn priced_embed(&mut self, text: &str, cost: i32) -> &mut Self {
        self.normal_embed(text)
    }

    fn normal_embed(&mut self, text: &str) -> &mut Self {
        self.color(serenity::utils::Color::from_rgb(242, 181, 37))
            .description(text)
    }

    fn error_embed(&mut self, text: &str) -> &mut Self {
        self.color(serenity::utils::Color::RED)
            .description(text)
    }
}
