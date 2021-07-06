use serenity::model::prelude::{User, UserId};

use crate::modules::shared::{
    caching::human::HumanCache, models::human::Human, repository::human::HumanRepository,
};

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
            }
            Err(_) => None,
        };
    }
}

pub struct TimeDelta {
    pub years: i64,
    pub months: i64,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
}

impl TimeDelta {
    pub fn from_seconds(seconds: i64) -> Self {
        let years = (seconds / 2592000) / 12;
        let months = (seconds / 2592000) % 30;
        let days = (seconds / 86400) % 30;
        let hours = (seconds / 3600) % 24;
        let minutes = (seconds % 3600) / 60;
        let seconds = seconds % 60;

        Self {
            years,
            months,
            days,
            hours,
            minutes,
            seconds,
        }
    }

    fn push_single_value(value: i64, name: &'static str, messages: &mut Vec<String>) {
        if messages.len() < 2 && value > 0 {
            if value == 1 {
                messages.push(format!("{} {}", value, name));
            } else {
                messages.push(format!("{} {}s", value, name));
            }
        }
    }

    pub fn to_text(&self) -> String {
        let mut messages: Vec<String> = Vec::new();

        TimeDelta::push_single_value(self.years, "year", &mut messages);
        TimeDelta::push_single_value(self.months, "month", &mut messages);
        TimeDelta::push_single_value(self.days, "day", &mut messages);
        TimeDelta::push_single_value(self.hours, "hour", &mut messages);
        TimeDelta::push_single_value(self.minutes, "minute", &mut messages);
        TimeDelta::push_single_value(self.seconds, "second", &mut messages);

        messages.join(" and ")
    }
}
