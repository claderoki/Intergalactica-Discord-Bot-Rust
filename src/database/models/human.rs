#[derive(Queryable)]
pub struct Human {
    pub id: i32,
    pub user_id: i64,
    pub gold: i32,
    pub timezone: String,
    pub date_of_birth: String,
    pub city: String,
    pub country_code: String,
    pub tester: Bool,
    pub currencies: String,
}

use super::schema::human;

#[derive(Insertable)]
#[table_name="human"]
pub struct NewHuman<'a> {
    pub user_id: i64
}