use diesel::Connection;

use crate::database::connection::get_connection_diesel;
use crate::modules::shared::models::human::Item;

type SingleItemResult = Result<Item, &'static str>;
pub struct ItemRepository;

impl ItemRepository {
    pub fn get(item_code: &str) -> SingleItemResult {
        use crate::database::schema::item::dsl::*;
        use diesel::prelude::*;

        let connection = get_connection_diesel();
        item
            .filter(code.eq(item_code))
            .first::<Item>(&connection)
            .map_err(|_| "Item not found.")
    }
}

pub struct HumanItemRepository;

impl HumanItemRepository {
    pub fn get_or_create(item_code: &str, human_id: i32) {

    }

    pub fn has_item(item_id: i32, human_id: i32, min_amount: i32) {
        let connection = get_connection_diesel();
        connection.execute("SELECT * FROM human_item WHERE amount > :min_amount AND human_id = :human_id AND item_id = :item_id");
    }
}

