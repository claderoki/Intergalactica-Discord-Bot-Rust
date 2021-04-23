use crate::{database::schema::human, modules::shared::repository::human::HumanRepository};

#[derive(Debug, Queryable, AsChangeset, Identifiable)]
#[table_name = "human"]
pub struct Human {
    pub id: i32,
    pub user_id: u64,
    pub gold: i32,
    pub timezone: Option<String>,
    pub date_of_birth: Option<String>,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub tester: bool,
    pub currencies: Option<String>,
}

impl Human {
    // pub fn save(&self) {
    //     HumanRepository::save(*self);
    // }
}
