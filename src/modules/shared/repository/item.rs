use diesel::{Connection, RunQueryDsl, sql_query, sql_types::{Integer, VarChar, BigInt}};

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


#[derive(QueryableByName)]
struct Count {
    #[sql_type="BigInt"]
    count: i64
}

pub struct HumanItemRepository;

impl HumanItemRepository {
    pub fn get_or_create(item_code: &str, human_id: i32) {

    }

    pub fn has_item(item_code: &str, human_id: i32, min_amount: i32) -> Result<bool, &'static str> {
        let connection = get_connection_diesel();

        let results: Result<Count, _> = sql_query("
            SELECT
            COUNT(*) AS count
            FROM
            human_item
            WHERE amount >= ?
            AND human_id = ?
            AND item_id IN (SELECT id FROM item WHERE code = ?)
            ")
            .bind::<Integer, _>(min_amount)
            .bind::<Integer, _>(human_id)
            .bind::<VarChar, _>(item_code)
            .get_result(&connection);

        match results {
            Ok(data) => Ok(data.count > 0),
            Err(_) => Err("Item not found")
        }
    }

    pub fn spend_item(item_code: &'static str, human_id: i32, amount: i32) -> Result<(), &'static str> {
        let connection = get_connection_diesel();

        let result = sql_query("
            UPDATE human_item
            SET amount = amount-?
            WHERE human_id = 1
            AND amount > ?
            AND item_id IN (SELECT id FROM item WHERE code = ?)")
            .bind::<Integer, _>(amount)
            .bind::<Integer, _>(human_id)
            .bind::<Integer, _>(amount)
            .bind::<VarChar, _>(item_code)
            .execute(&connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Could not update item")
        }
    }

}

