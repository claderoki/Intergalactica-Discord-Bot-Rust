use diesel::{RunQueryDsl, sql_query, sql_types::{Integer, VarChar}};

use crate::database::connection::get_connection_diesel;

#[derive(QueryableByName)]
pub struct SimpleItem {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "VarChar"]
    pub name: String,

    #[sql_type = "VarChar"]
    pub image_url: String,
}

pub struct ItemRepository;
impl ItemRepository {

    pub fn get_simple(id: i32) -> Result<SimpleItem, String> {
        let connection = get_connection_diesel();

        let results: Result<SimpleItem, _> =
            sql_query(include_str!("queries/item/get_simple_item.sql"))
                .bind::<Integer, _>(id)
                .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                println!("{:?}", e);
                Err(format!("{:?}", e))
            },
        }
    }

    fn get_child_category_ids(parent_category_id: i32) {

    }

    pub fn get_random(category_id: i32) {

    }

    pub fn add_item(id: i32, human_id: i32, amount: i32) -> Result<(), &'static str> {
        let connection = get_connection_diesel();

        let result = sql_query(include_str!("queries/item/add_item.sql"))
        .bind::<Integer, _>(id)
        .bind::<Integer, _>(human_id)
        .bind::<Integer, _>(amount)
        .bind::<Integer, _>(amount)
        .execute(&connection);

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err("Could not update item")
            },
        }
    }

}

// impl ItemRepository {
//     pub fn get(item_code: &str) -> SingleItemResult {
//         use crate::database::schema::item::dsl::*;
//         use diesel::prelude::*;

//         let connection = get_connection_diesel();
//         item.filter(code.eq(item_code))
//             .first::<Item>(&connection)
//             .map_err(|_| "Item not found.")
//     }
// }

// pub struct HumanItemRepository;

// impl HumanItemRepository {
//     pub fn has_item(item_code: &str, human_id: i32, min_amount: i32) -> Result<bool, &'static str> {
//         let connection = get_connection_diesel();

//         let results: Result<Countable, _> = sql_query(
//             "
//             SELECT
//             COUNT(*) AS count
//             FROM
//             human_item
//             WHERE amount >= ?
//             AND human_id = ?
//             AND item_id IN (SELECT id FROM item WHERE code = ?)
//             LIMIT 1
//             ",
//         )
//         .bind::<Integer, _>(min_amount)
//         .bind::<Integer, _>(human_id)
//         .bind::<VarChar, _>(item_code)
//         .get_result(&connection);

//         match results {
//             Ok(data) => Ok(data.count > 0),
//             Err(_) => Err("Item not found"),
//         }
//     }

//     pub fn add_item(item_code: &str, human_id: i32, amount: i32) -> Result<(), &'static str> {
//         /*
//             The human select is problematic because it assumes the human exists.
//             So we need to either pass the human_id, or ensure it exists somewhere.
//             Maybe use redis to map user_id to human_id?
//          */
//         let connection = get_connection_diesel();
//         let result = sql_query(
//             "
//             INSERT INTO human_item (item_id, human_id, amount)
//             VALUES (
//                 (SELECT id FROM item WHERE `code` = ? LIMIT 1),
//                 ?,
//                 ?
//             )
//             ON DUPLICATE KEY UPDATE
//                amount = amount + ?
//             ",
//         )
//         .bind::<VarChar, _>(item_code)
//         .bind::<Integer, _>(human_id)
//         .bind::<Integer, _>(amount)
//         .bind::<Integer, _>(amount)
//         .execute(&connection);

//         match result {
//             Ok(_) => Ok(()),
//             Err(_) => Err("Could not update item"),
//         }
//     }

//     pub fn spend_item(
//         item_code: &'static str,
//         human_id: i32,
//         amount: i32,
//     ) -> Result<(), &'static str> {
//         let connection = get_connection_diesel();

//         let result = sql_query(
//             "
//             UPDATE human_item
//             SET amount = amount-?
//             WHERE human_id = 1
//             AND amount > ?
//             AND item_id IN (SELECT id FROM item WHERE code = ?)",
//         )
//         .bind::<Integer, _>(amount)
//         .bind::<Integer, _>(human_id)
//         .bind::<Integer, _>(amount)
//         .bind::<VarChar, _>(item_code)
//         .execute(&connection);

//         match result {
//             Ok(_) => Ok(()),
//             Err(_) => Err("Could not update item"),
//         }
//     }
// }
