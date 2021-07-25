// use chrono::{Duration, NaiveDateTime};
// use redis::{Commands, ToRedisArgs};

// use crate::redis_utils::connection::get_connection_redis;

// pub trait Flag {
//     fn get_key() -> String;
//     fn new(datetime: NaiveDateTime) -> Self;
//     fn get_datetime(&self) -> NaiveDateTime;
// }

// // pub struct GenericFlag<T> {
// //     pub when: NaiveDateTime,
// //     pub identifier: T,
// // }

// // impl Flag for GenericFlag<T> where T: ToString {
// //     fn get_key() -> String {
// //         T.into()
// //     }

// //     fn new(when: NaiveDateTime) -> Self {
// //         Self {
// //             when,
// //             identifier: (),
// //         }
// //     }

// //     fn get_datetime(&self) -> NaiveDateTime {
// //         todo!()
// //     }
// // }

// // pub struct PigeonLastHealed {
// //     pub datetime: NaiveDateTime,
// //     pub identifier: String,
// // }
// // impl Flag for PigeonLastHealed {
// //     fn get_key() -> String {
// //         "pigeon_last_healed".into()
// //     }

// //     fn new(datetime: NaiveDateTime, identifier: &'static str) -> Self {
// //         Self {
// //             datetime,
// //             identifier: identifier.into(),
// //         }
// //     }

// //     fn get_datetime(&self) -> NaiveDateTime {
// //         self.datetime
// //     }
// // }
// pub struct FlagValidator;
// impl FlagValidator {
//     pub fn validate<T>(human_id: i32, duration: Duration) -> Result<NaiveDateTime, String>
//     where
//         T: Flag,
//     {
//         let now = chrono::offset::Utc::now().naive_utc();
//         if let Some(flag) = FlagCache::get::<T>(human_id) {
//             let difference = flag.get_datetime() - now;
//             if difference <= duration {
//                 return Err(format!("You can only use this command every ..."));
//             }
//         }
//         Ok(now)
//     }
// }

// const DT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

// pub struct FlagCache;
// impl FlagCache {
//     fn get_key<T>(human_id: i32) -> String
//     where
//         T: Flag,
//     {
//         format!("flags:{}:{}", human_id, T::get_key())
//     }

//     pub fn get<T>(human_id: i32) -> Option<T>
//     where
//         T: Flag,
//     {
//         let mut connection = get_connection_redis();

//         let value: Result<String, _> = connection.get(&FlagCache::get_key::<T>(human_id));
//         match value {
//             Ok(v) => Some(T::new(
//                 NaiveDateTime::parse_from_str(&v, DT_FORMAT).unwrap(),
//             )),
//             Err(_) => None,
//         }
//     }

//     pub fn add<T>(human_id: i32, when: NaiveDateTime) -> bool
//     where
//         T: Flag,
//     {
//         let mut connection = get_connection_redis();

//         let result: Result<(), _> = connection.set(
//             &FlagCache::get_key::<T>(human_id),
//             when.format(DT_FORMAT).to_string(),
//         );
//         result.is_ok()
//     }
// }

// trait CacheParams {
//     fn get_key() -> String;
//     fn get_value<T>(&self) -> T where T: ToRedisArgs;
// }

// // impl ToRedisArgs for NaiveDateTime {
// //     fn write_redis_args<W>(&self, out: &mut W)
// //     where
// //         W: ?Sized + redis::RedisWrite {
// //         self.format(DT_FORMAT).to_string()
// //     }
// // }

// trait Cache {
//     fn get_key<D>(params: D) -> String where D: CacheParams;

//     fn get<T, D>(params: D) -> Option<T>
//     where
//         T: Flag,
//         D: CacheParams
//     {
//         let mut connection = get_connection_redis();

//         let value: Result<String, _> = connection.get(Self::get_key::<D>(params));
//         match value {
//             Ok(v) => Some(T::new(
//                 NaiveDateTime::parse_from_str(&v, DT_FORMAT).unwrap(),
//             )),
//             Err(_) => None,
//         }
//     }

//     fn add<T, D>(params: D) -> bool where T: Flag, D: CacheParams {
//         let mut connection = get_connection_redis();

//         let result: Result<String, _> = connection.set::<_, String, String>(
//             Self::get_key::<D>(params),
//             params.get_value(),
//         );
//         result.is_ok()
//     }
// }