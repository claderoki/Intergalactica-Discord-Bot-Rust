// use diesel::{backend::Backend, deserialize, serialize::{self, Output}, sql_types::Integer, types::{FromSql, ToSql}};

// pub trait Stat {
//     fn get_name(&self) -> String;
//     fn get_emoji(&self) -> String;
//     fn get_value(&self) -> i32;
// }

// pub enum HumanStat {
//     Gold(i32)
// }

// #[derive(Debug)]
// pub enum PigeonStat {
//     Food(i32),
//     Health(i32),
//     Cleanliness(i32),
//     Experience(i32),
//     Happiness(i32),
// }

// impl Stat for PigeonStat {
//     fn get_name(&self) -> String {
//         match self {
//             PigeonStat::Food(_) => "food",
//             PigeonStat::Health(_) => "health",
//             PigeonStat::Cleanliness(_) => "cleanliness",
//             PigeonStat::Experience(_) => "experience",
//             PigeonStat::Happiness(_) => "happiness",
//         }.into()
//     }

//     fn get_emoji(&self) -> String {
//         match self {
//             PigeonStat::Food(_) => "🌾",
//             PigeonStat::Health(_) => "",
//             PigeonStat::Cleanliness(_) => "",
//             PigeonStat::Experience(_) => "",
//             PigeonStat::Happiness(_) => "",
//         }.into()
//     }

//     fn get_value(&self) -> i32 {
//         match self {
//             PigeonStat::Food(v) => *v,
//             PigeonStat::Health(v) => *v,
//             PigeonStat::Cleanliness(v) => *v,
//             PigeonStat::Experience(v) => *v,
//             PigeonStat::Happiness(v) => *v,
//         }
//     }
// }

// impl<DB> ToSql<Integer, DB> for PigeonStat
// where
//     DB: Backend,
//     i32: ToSql<Integer, DB>,
// {
//     fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
//     where
//         W: std::io::Write,
//     {
//         self.get_value().to_sql(out)
//     }
// }

// impl<DB: Backend> FromSql<Integer, DB> for PigeonStat
// where
//     i32: FromSql<Integer, DB>,
// {
//     fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
//         let value = i32::from_sql(bytes)?;
//         Ok(Self::Food(value))
//     }
// }
