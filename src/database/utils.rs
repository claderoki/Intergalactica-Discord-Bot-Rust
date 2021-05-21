use diesel::{sql_types::{BigInt, Integer}};

#[derive(QueryableByName)]
pub struct Countable {
    #[sql_type = "BigInt"]
    pub count: i64,
}
#[derive(QueryableByName)]
pub struct IdOnly {
    #[sql_type = "Integer"]
    pub id: i32,
}
