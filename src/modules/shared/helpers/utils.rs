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

    pub fn to_text(&self) -> String {
        let mut messages: Vec<String> = Vec::new();

        if messages.len() < 2 && self.years > 0 {
            if self.years == 1 {
                messages.push(format!("{} year", self.years));
            } else {
                messages.push(format!("{} years", self.years));
            }
        }

        if messages.len() < 2 && self.months > 0 {
            if self.months == 1 {
                messages.push(format!("{} month", self.months));
            } else {
                messages.push(format!("{} months", self.months));
            }
        }

        if messages.len() < 2 && self.days > 0 {
            if self.days == 1 {
                messages.push(format!("{} day", self.days));
            } else {
                messages.push(format!("{} days", self.days));
            }
        }

        if messages.len() < 2 && self.hours > 0 {
            if self.hours == 1 {
                messages.push(format!("{} hour", self.hours));
            } else {
                messages.push(format!("{} hours", self.hours));
            }
        }

        if messages.len() < 2 && self.minutes > 0 {
            if self.minutes == 1 {
                messages.push(format!("{} minute", self.minutes));
            } else {
                messages.push(format!("{} minutes", self.minutes));
            }
        }

        if messages.len() < 2 && self.seconds > 0 {
            if self.seconds == 1 {
                messages.push(format!("{} second", self.seconds));
            } else {
                messages.push(format!("{} seconds", self.seconds));
            }
        }

        messages.join(" and ")
    }

}
