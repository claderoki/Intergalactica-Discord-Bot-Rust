use crate::database::schema::currency;
#[derive(Debug, Queryable, AsChangeset, Identifiable)]
#[table_name = "currency"]
pub struct Currency {
    pub id: i32,
    pub rate: f64,
    pub is_base: bool,
    pub name: String,
    pub code: String,
    pub symbol: String,
}
