use diesel::{
    backend::Backend,
    deserialize,
    serialize::{self, Output},
    sql_types::Integer,
    types::{FromSql, ToSql},
};

pub trait Stat {
    fn get_name(&self) -> String;
    fn get_emoji(&self) -> String;
    fn get_value(&self) -> i32;
}

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

macro_rules! stat_i32 {
    ($($name:ident;)*) => {
        $(
            impl $name {

                /// Immutably borrow inner Id.
                #[inline]
                pub fn as_i32(&self) -> &i32 {
                    &self.0
                }

                /// Mutably borrow inner Id.
                #[inline]
                pub fn as_mut_i32(&mut self) -> &mut i32 {
                    &mut self.0
                }
            }

            impl From<i32> for $name {
                fn from(value_as_i32: i32) -> $name {
                    $name(value_as_i32)
                }
            }

            impl<DB> ToSql<Integer, DB> for $name
            where
                DB: Backend,
                i32: ToSql<Integer, DB>,
            {
                fn to_sql<W>(&self, out: &mut Output<W, DB>) -> serialize::Result
                where
                    W: std::io::Write,
                {
                    self.0.to_sql(out)
                }
            }

            impl<DB: Backend> FromSql<Integer, DB> for $name
            where
                i32: FromSql<Integer, DB>,
            {
                fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
                    let value = i32::from_sql(bytes)?;
                    Ok($name(value))
                }
            }

            impl PartialEq<i32> for $name {
                fn eq(&self, u: &i32) -> bool {
                    self.0 == *u
                }
            }

            impl From<$name> for i32 {
                fn from(id: $name) -> i32 {
                    id.0 as i32
                }
            }

        )*
    }
}

#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PigeonExperience(pub i32);

#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PigeonFood(pub i32);

#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PigeonCleanliness(pub i32);

#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PigeonHealth(pub i32);

#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PigeonHappiness(pub i32);

stat_i32! {
    PigeonExperience;
    PigeonFood;
    PigeonCleanliness;
    PigeonHealth;
    PigeonHappiness;
}
