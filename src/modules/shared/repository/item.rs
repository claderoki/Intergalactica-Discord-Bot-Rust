use diesel::sql_query;
use diesel::sql_types::BigInt;
use diesel::sql_types::Integer;
use diesel::sql_types::VarChar;
use diesel::MysqlConnection;
use diesel::RunQueryDsl;

use crate::database::connection::get_connection_diesel;
use crate::database::utils::NullableIdOnly;
use tracing::error;

#[derive(QueryableByName, Debug)]
pub struct SimpleItem {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "VarChar"]
    pub name: String,

    #[sql_type = "VarChar"]
    pub image_url: String,

    #[sql_type = "BigInt"]
    pub amount: i64,
}

enum CategoryType {
    Children,
    _Parents,
}

pub struct ItemRepository;
impl ItemRepository {
    pub fn get_simple(id: i32) -> Result<SimpleItem, String> {
        let connection = get_connection_diesel()?;

        let results: Result<SimpleItem, _> =
            sql_query(include_str!("queries/item/get_simple_item.sql"))
                .bind::<Integer, _>(id)
                .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                println!("{:?}", e);
                Err(format!("{:?}", e))
            }
        }
    }

    // pub fn get_simple_items(ids: &Vec<i32>) -> Result<Vec<SimpleItem>, String> {
    //     let connection = get_connection_diesel()?;

    //     let results: Result<Vec<SimpleItem>, _> =
    //         sql_query(include_str!("queries/item/get_simple_items.sql"))
    //             .bind::<VarChar, _>(
    //                 ids.iter()
    //                     .map(|id| id.to_string())
    //                     .collect::<Vec<String>>()
    //                     .join(", "),
    //             )
    //             .get_results(&connection);

    //     match results {
    //         Ok(data) => Ok(data),
    //         Err(e) => {
    //             println!("{:?}", e);
    //             Err(format!("{:?}", e))
    //         }
    //     }
    // }

    fn get_categories(
        connection: &MysqlConnection,
        category_id: i32,
        category_type: &CategoryType,
    ) -> Result<Vec<i32>, String> {
        let results: Result<Vec<NullableIdOnly>, _> = sql_query(match category_type {
            CategoryType::Children => include_str!("queries/item/get_children.sql"),
            CategoryType::_Parents => include_str!("queries/item/get_parents.sql"),
        })
        .bind::<Integer, _>(category_id)
        .get_results(connection);

        match results {
            Ok(data) => {
                let mut ids: Vec<i32> = Vec::new();
                for child in data.iter() {
                    match child.id {
                        Some(id) => {
                            ids.push(id);
                        }
                        _ => {}
                    }
                }
                Ok(ids)
            }
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get_categories".into())
            }
        }
    }

    fn get_all_categories(
        connection: &MysqlConnection,
        category_id: i32,
        category_type: &CategoryType,
    ) -> Result<Vec<i32>, String> {
        let mut all: Vec<i32> = Vec::new();
        all.push(category_id);
        let mut category_ids: Vec<i32> = Vec::new();
        category_ids.push(category_id);

        let mut i = 0;
        while category_ids.len() > 0 && i < 5 {
            for cat_id in category_ids.clone().iter() {
                let categories =
                    ItemRepository::get_categories(&connection, *cat_id, &category_type);
                match categories {
                    Ok(c) => {
                        category_ids.clear();
                        if c.len() > 0 {
                            for id in c.iter() {
                                all.push(*id);
                                category_ids.push(*id);
                            }
                        }
                    }
                    Err(_) => {
                        category_ids.clear();
                    }
                }
            }
            i += 1;
        }

        Ok(all)
    }

    pub fn get_random(category_id: i32) -> Result<SimpleItem, String> {
        let connection = get_connection_diesel()?;

        let parents =
            ItemRepository::get_all_categories(&connection, category_id, &CategoryType::Children)?;

        if parents.is_empty() {
            return Err("No items found.".into());
        }

        let placeholders = parents
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let results: Result<SimpleItem, _> = sql_query(include_str!("queries/item/get_random.sql"))
            .bind::<VarChar, _>(placeholders.to_string())
            .bind::<VarChar, _>(placeholders)
            .get_result(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => {
                error!("{:?}", e);
                Err("Failed to get_random".into())
            }
        }
    }

    pub fn add_item(id: i32, human_id: i32, amount: i32) -> Result<(), &'static str> {
        let connection = get_connection_diesel()?;

        let result = sql_query(include_str!("queries/item/add_item.sql"))
            .bind::<Integer, _>(id)
            .bind::<Integer, _>(human_id)
            .bind::<Integer, _>(amount)
            .bind::<Integer, _>(amount)
            .execute(&connection);

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{:?}", e);
                Err("Could not update item")
            }
        }
    }

    pub fn add_items(ids: Vec<i32>, human_id: i32) -> Result<(), &'static str> {
        for id in ids {
            let _ = ItemRepository::add_item(id, human_id, 1);
        }
        Ok(())
    }
}

// impl ItemRepository {
//     pub fn get(item_code: &str) -> SingleItemResult {
//         use crate::database::schema::item::dsl::*;
//         use diesel::prelude::*;

//         let connection = get_connection_diesel()?;
//         item.filter(code.eq(item_code))
//             .first::<Item>(&connection)
//             .map_err(|_| "Item not found.")
//     }
// }

// pub struct HumanItemRepository;

// impl HumanItemRepository {
//     pub fn has_item(item_code: &str, human_id: i32, min_amount: i32) -> Result<bool, &'static str> {
//         let connection = get_connection_diesel()?;

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
//         let connection = get_connection_diesel()?;
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
//         let connection = get_connection_diesel()?;

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
