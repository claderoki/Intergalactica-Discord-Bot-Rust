use chrono::Duration;
use chrono::NaiveDateTime;
use redis::Commands;

use crate::modules::shared::helpers::utils::TimeDelta;
use crate::redis_utils::connection::get_connection_redis;

const DT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub enum BucketType {
    _Guild(u64),
    _Member(u64, u64),
    User(u64),
}

pub struct Bucket {
    pub identifier: String,
    pub bucket_type: BucketType,
    pub cooldown: Duration,
}

impl Bucket {
    pub fn new(identifier: &str, bucket_type: BucketType, cooldown: Duration) -> Self {
        Self {
            identifier: identifier.into(),
            bucket_type,
            cooldown,
        }
    }

    pub fn user<T: Into<u64>>(identifier: &str, user_id: T, cooldown: Duration) -> Self {
        Self::new(identifier, BucketType::User(user_id.into()), cooldown)
    }

    pub fn validate(&self) -> Result<NaiveDateTime, String> {
        match BucketCache::get(self) {
            Some(datetime) => {
                let now = chrono::offset::Utc::now().naive_utc();
                let difference = now - datetime;
                if difference <= self.cooldown {
                    let delta = TimeDelta::from_seconds(
                        self.cooldown.num_seconds() - difference.num_seconds(),
                    );
                    return Err(format!(
                        "This command will be available again in {}.",
                        delta.to_text()
                    ));
                }
                Ok(now)
            }
            None => Ok(chrono::offset::Utc::now().naive_utc()),
        }
    }

    pub fn spend(&self, datetime: NaiveDateTime) {
        BucketCache::add(self, datetime);
    }
}

pub struct BucketCache;
impl BucketCache {
    fn get_key(bucket: &Bucket) -> String {
        let key = match bucket.bucket_type {
            BucketType::_Guild(g) => format!("{}", g),
            BucketType::_Member(g, u) => format!("{}:{}", g, u),
            BucketType::User(u) => format!("{}", u),
        };
        format!("buckets:{}:{}", key, bucket.identifier)
    }

    pub fn get(bucket: &Bucket) -> Option<NaiveDateTime> {
        match get_connection_redis() {
            Ok(mut connection) => {
                let value: Result<String, _> = connection.get(&BucketCache::get_key(bucket));
                match value {
                    Ok(v) => NaiveDateTime::parse_from_str(&v, DT_FORMAT).ok(),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    pub fn add(bucket: &Bucket, when: NaiveDateTime) -> bool {
        match get_connection_redis() {
            Ok(mut connection) => {
                let result: Result<(), _> = connection.set(
                    &BucketCache::get_key(bucket),
                    when.format(DT_FORMAT).to_string(),
                );
                result.is_ok()
            }
            Err(_) => false,
        }
    }
}
