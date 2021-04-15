use crate::database::schema::measurement;
#[derive(Debug, Queryable, AsChangeset, Identifiable)]
#[table_name = "measurement"]
pub struct Measurement {
    pub id: i32,
    pub rate: f64,
    pub is_base: bool,
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub subtype: String,
}
