use diesel::sql_query;
use diesel::sql_types::Bool;
use diesel::sql_types::Integer;
use diesel::sql_types::VarChar;
use diesel::RunQueryDsl;
use serenity::model::prelude::User;

use crate::database::connection::get_connection_diesel;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::helpers::utils::HumanUtils;

#[derive(QueryableByName)]
pub struct PigeonValidationResult {
    #[sql_type = "Bool"]
    has_gold_needed: bool,

    #[sql_type = "Bool"]
    has_active_pigeon: bool,

    #[sql_type = "Bool"]
    has_required_status: bool,

    #[sql_type = "Bool"]
    should_notify_death: bool,

    #[sql_type = "Bool"]
    has_item_needed: bool,
}

pub struct PigeonValidation {
    gold_needed: i32,
    item_needed: Option<String>,
    needs_active_pigeon: Option<bool>,
    required_pigeon_status: Option<PigeonStatus>,
    other: bool,
    human_id: Option<i32>,
}

impl PigeonValidation {
    pub fn new() -> Self {
        PigeonValidation {
            gold_needed: 0,
            item_needed: None,
            other: false,
            human_id: None,
            needs_active_pigeon: None,
            required_pigeon_status: None,
        }
    }

    pub fn gold_needed(&mut self, value: i32) -> &mut Self {
        self.gold_needed = value;
        self
    }

    pub fn other(&mut self, value: bool) -> &mut Self {
        self.other = value;
        self
    }

    pub fn needs_active_pigeon(&mut self, value: bool) -> &mut Self {
        self.needs_active_pigeon = Some(value);
        self
    }

    // pub fn human_id(&mut self, value: i32) -> &mut Self {
    //     self.human_id = Some(value);
    //     self
    // }

    pub fn required_pigeon_status(&mut self, value: PigeonStatus) -> &mut Self {
        self.required_pigeon_status = Some(value);
        self
    }

    // pub fn item_needed(&mut self, value: &'static str) -> &mut Self {
    //     self.item_needed = Some(String::from(value));
    //     self
    // }

    fn get_query(&self) -> String {
        let mut query = String::from("SELECT");
        query.push_str("(`human`.`gold` >= ?) as has_gold_needed, ");
        query.push_str("(`pigeon`.`id` IS NOT NULL) as has_active_pigeon, ");

        if self.required_pigeon_status.is_some() {
            query.push_str("(`pigeon`.`status` IS NOT NULL AND `pigeon`.`status` = ?) as has_required_status, ");
        } else {
            query.push_str("(1 OR ? = 1) as has_required_status, ");
        }

        query.push_str("(SELECT COUNT(*) from `pigeon` p WHERE `p`.`human_id` = `human`.`id` AND `p`.`condition` = 'dead' AND `p`.`death_notified` = 0) as should_notify_death,");

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

    fn get_validation_result(&self, human_id: i32) -> Result<PigeonValidationResult, String> {
        let connection = get_connection_diesel();

        let results: Result<PigeonValidationResult, _> = sql_query(self.get_query())
            .bind::<Integer, _>(self.gold_needed)
            .bind::<VarChar, _>(match self.required_pigeon_status {
                Some(status) => status.to_string(),
                None => String::from(""),
            })
            .bind::<Integer, _>(1)
            .bind::<VarChar, _>(self.item_needed.as_ref().unwrap_or(&String::from("")))
            .bind::<Integer, _>(human_id)
            .get_result(&connection);

        return match results {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("{:?}", e)),
        };
    }

    pub fn validate(&self, user: &User) -> Result<i32, String> {
        let human_id = match self.human_id {
            Some(id) => id,
            None => user.get_human_id().ok_or("Error creating human")?,
        };

        let result = self.get_validation_result(human_id)?;

        if result.should_notify_death && !self.other {
            PigeonRepository::update_death_notified(human_id, true);
            return Err("Your pigeon has died. Better take better care of it next time!".into());
        }

        if self.gold_needed > 0 && !result.has_gold_needed {
            return Err(if self.other {
                format!(
                    "The other person needs {} gold to perform this action",
                    self.gold_needed
                )
            } else {
                format!("You need {} gold to perform this action", self.gold_needed)
            });
        }

        if self.needs_active_pigeon.is_some()
            && !self.needs_active_pigeon.eq(&Some(result.has_active_pigeon))
        {
            if result.has_active_pigeon {
                if self.other {
                    return Err("The other person already has a pigeon!".into());
                } else {
                    return Err("You already have a pigeon!".into());
                }
            } else {
                if self.other {
                    return Err("The other person does not have a pigeon!".into());
                } else {
                    return Err("You do not have a pigeon!".into());
                }
            }
        }

        if self.item_needed.is_some() {
            if !result.has_item_needed {
                return Err(if self.other {
                    format!(
                        "To perform this action the other person needs the `{}` item ",
                        self.item_needed.as_ref().unwrap()
                    )
                } else {
                    format!(
                        "To perform this action you need the `{}` item ",
                        self.item_needed.as_ref().unwrap()
                    )
                });
            }
        }

        match self.required_pigeon_status {
            Some(_) => {
                if !result.has_required_status {
                    return Err(if self.other {
                        format!(
                            "The other pigeon needs to be {} to perform this action.",
                            self.required_pigeon_status.unwrap().get_friendly_verb()
                        )
                    } else {
                        format!(
                            "Your pigeon needs to be {} to perform this action.",
                            self.required_pigeon_status.unwrap().get_friendly_verb()
                        )
                    });
                }
            }
            None => {}
        }

        Ok(human_id)
    }
}
