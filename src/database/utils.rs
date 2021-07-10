use diesel::sql_types::BigInt;
use diesel::sql_types::Integer;
use diesel::sql_types::Nullable;

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

#[derive(QueryableByName)]
pub struct NullableIdOnly {
    #[sql_type = "Nullable<Integer>"]
    pub id: Option<i32>,
}
