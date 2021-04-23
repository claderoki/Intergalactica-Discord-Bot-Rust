use serenity::model::prelude::{User, UserId};

use crate::modules::shared::{models::human::Human, repository::human::HumanRepository};


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

pub trait HumanUtils {
    fn get_human(&self) -> Option<Human>;
}

impl HumanUtils for User {
    fn get_human(&self) -> Option<Human> {
        HumanRepository::get_or_create(*self.id.as_u64()).ok()
    }
}

impl HumanUtils for UserId {
    fn get_human(&self) -> Option<Human> {
        HumanRepository::get_or_create(*self.as_u64()).ok()
    }
}