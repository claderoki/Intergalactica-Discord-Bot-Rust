use crate::database::schema::human;

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
    pub fn assert_gold(&self, cost: i32) -> Result<(), &'static str> {
        if self.gold > cost {
            Ok(())
        } else {
            Err("You need {} gold to perform this action.")
        }
    }
}
