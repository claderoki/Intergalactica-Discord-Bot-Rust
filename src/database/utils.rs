use diesel::{sql_types::{BigInt}};

#[derive(QueryableByName)]
pub struct Countable {
    #[sql_type = "BigInt"]
    pub count: i64,
}
