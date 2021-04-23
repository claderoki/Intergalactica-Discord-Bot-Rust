use crate::{database::schema::{human, item, human_item, item_category}};

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

#[derive(Debug, Queryable, AsChangeset, Identifiable)]
#[table_name = "item"]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub description: String,
    pub image_url: String,
    pub rarity: String,
    pub explorable: bool,
    pub usable: bool,
    pub category_id: i32,
    pub chance: i32,
}

#[derive(Debug, Queryable, AsChangeset, Identifiable)]
#[table_name = "human_item"]
pub struct HumanItem {
    pub id: i32,
    pub human_id: i32,
    pub item_id: i32,
    pub amount: i32,
    pub found: bool,
}

#[derive(Debug, Queryable, AsChangeset, Identifiable)]
#[table_name = "item_category"]
pub struct ItemCategory {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub parent_id: i32,
}
