use diesel::{
    sql_query,
    sql_types::{Bool, Integer, VarChar},
    RunQueryDsl,
};
use serenity::model::prelude::User;

use crate::{database::connection::get_connection_diesel, modules::{pigeon::models::pigeon::PigeonStatus, shared::helpers::utils::HumanUtils}};

#[derive(QueryableByName)]
pub struct PigeonValidationResult {
    #[sql_type = "Bool"]
    has_gold_needed: bool,

    #[sql_type = "Bool"]
    has_active_pigeon: bool,

    #[sql_type = "Bool"]
    has_required_status: bool,

    #[sql_type = "Bool"]
    has_item_needed: bool,
}

pub struct PigeonValidation {
    gold_needed: i32,
    item_needed: Option<String>,
    needs_active_pigeon: Option<bool>,
    required_pigeon_status: Option<PigeonStatus>,
}

impl PigeonValidation {
    pub fn new() -> Self {
        PigeonValidation {
            gold_needed: 0,
            item_needed: None,
            needs_active_pigeon: None,
            required_pigeon_status: None,
        }
    }

    pub fn gold_needed(&mut self, value: i32) -> &mut Self {
        self.gold_needed = value;
        self
    }

    pub fn needs_active_pigeon(&mut self, value: bool) -> &mut Self {
        self.needs_active_pigeon = Some(value);
        self
    }

    pub fn required_pigeon_status(&mut self, value: PigeonStatus) -> &mut Self {
        self.required_pigeon_status = Some(value);
        self
    }

    pub fn item_needed(&mut self, value: &'static str) -> &mut Self {
        self.item_needed = Some(String::from(value));
        self
    }

    fn get_query(&self) -> String {
        let mut query = String::from("SELECT");
        query.push_str("(`human`.`gold` >= ?) as has_gold_needed, ");
        query.push_str("(`pigeon`.`id` IS NOT NULL) as has_active_pigeon, ");

        if self.required_pigeon_status.is_some() {
            query.push_str("(`pigeon`.`status` IS NOT NULL AND `pigeon`.`status` = ?) as has_required_status, ");
        } else {
            query.push_str("(1 OR ? = 1) as has_required_status, ");
        }

        if self.item_needed.is_some() {
            query.push_str("(`human_item`.`amount` IS NOT NULL AND `human_item`.`amount` >= ?) as has_item_needed ");
        } else {
            // this is a shit way to force it to use all the parameters and is essentially the same as `0` as has_item_needed
            query.push_str("(0 AND ? = ?) as has_item_needed ");
        }
        query.push_str("FROM human ");

        query.push_str("LEFT JOIN pigeon ON `pigeon`.`human_id` = `human`.`id` AND `pigeon`.`condition` = 'active' ");
        if self.item_needed.is_some() {
            query.push_str("LEFT JOIN human_item ON `human_item`.`human_id` = `human`.`id` AND `human_item`.`item_id` IN (SELECT `item`.`id` FROM item WHERE `code` = ?) ");
        }
        query.push_str("WHERE `human`.`id` = ? LIMIT 1 ");

        query
    }

    fn get_validation_result(&self, human_id: i32) -> Result<PigeonValidationResult, &'static str> {
        let connection = get_connection_diesel();

        let results: Result<PigeonValidationResult, _> = sql_query(self.get_query())
            .bind::<Integer, _>(self.gold_needed)
            .bind::<VarChar, _>(match self.required_pigeon_status {
                Some(status) => status.to_string(),
                None => String::from("")
            })
            .bind::<Integer, _>(1)
            .bind::<VarChar, _>(self.item_needed.as_ref().unwrap_or(&String::from("")))
            .bind::<Integer, _>(human_id)
            .get_result(&connection);

        return match results {
            Ok(result) => Ok(result),
            Err(e) => {
                println!("{:?}", e);
                Err("Error in sql query: PigeonValidation.validate()")
            }
        };
    }

    pub fn validate(&self, user: &User) -> Result<i32, String> {
        let human_id = user.get_human_id().ok_or("Error creating human")?;
        let result = self.get_validation_result(human_id)?;

        if self.gold_needed > 0 && !result.has_gold_needed {
            return Err(format!("You need {} gold to perform this action", self.gold_needed));
        }

        if self.needs_active_pigeon.is_some()
            && !self.needs_active_pigeon.eq(&Some(result.has_active_pigeon))
        {
            if result.has_active_pigeon {
                return Err("You already have a pigeon!".into());
            } else {
                return Err("You do not have a pigeon!".into());
            }
        }

        if self.item_needed.is_some() {
            if !result.has_item_needed {
                return Err(format!("To perform this action you need the `{}` item ", self.item_needed.as_ref().unwrap()));
            }
        }

        match self.required_pigeon_status {
            Some(_) => {
                if !result.has_required_status {
                    return Err(format!("Your pigeon isn't {}.", self.required_pigeon_status.unwrap().to_string()));
                }
            }
            None => {}
        }

        Ok(human_id)
    }
}
