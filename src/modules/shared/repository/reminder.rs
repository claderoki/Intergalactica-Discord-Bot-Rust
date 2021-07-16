use chrono::NaiveDateTime;
use diesel::sql_query;
use diesel::sql_types::BigInt;
use diesel::sql_types::Datetime;
use diesel::sql_types::Integer;
use diesel::sql_types::Nullable;
use diesel::sql_types::VarChar;
use diesel::types::Unsigned;
use diesel::RunQueryDsl;

use crate::database::connection::get_connection_diesel;

#[derive(QueryableByName, Debug)]
pub struct Reminder {
    #[sql_type = "Integer"]
    pub id: i32,

    #[sql_type = "VarChar"]
    pub message: String,

    #[sql_type = "Unsigned<BigInt>"]
    pub user_id: u64,

    #[sql_type = "Nullable<Unsigned<BigInt>>"]
    pub channel_id: Option<u64>,
}

pub struct NewReminder {
    user_id: u64,
    channel_id: Option<u64>,
    message: String,
    due_date: NaiveDateTime,
}

impl NewReminder {
    pub fn new(user_id: u64, message: String, due_date: NaiveDateTime) -> Self {
        Self {
            user_id,
            channel_id: None,
            message: message.into(),
            due_date,
        }
    }

    pub fn channel_id(&mut self, channel_id: u64) -> &mut Self {
        self.channel_id = Some(channel_id);
        self
    }
}

pub struct ReminderRepository;
impl ReminderRepository {
    pub fn create(reminder: &NewReminder) -> Result<(), String> {
        let connection = get_connection_diesel();

        let result = sql_query(include_str!("queries/reminder/create.sql"))
            .bind::<Unsigned<BigInt>, _>(reminder.user_id)
            .bind::<Nullable<Unsigned<BigInt>>, _>(reminder.channel_id)
            .bind::<VarChar, _>(&reminder.message)
            .bind::<Datetime, _>(reminder.due_date)
            .execute(&connection);

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err("Could not create reminder".into())
            }
        }
    }

    pub fn get_due_reminders() -> Result<Vec<Reminder>, String> {
        let connection = get_connection_diesel();

        let results: Result<Vec<Reminder>, _> =
            sql_query(include_str!("queries/reminder/get_due_reminders.sql"))
                .get_results(&connection);

        match results {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub fn set_sent_multiple(reminder_ids: Vec<i32>) -> Result<(), String> {
        let connection = get_connection_diesel();

        let result = sql_query(include_str!("queries/reminder/set_sent_multiple.sql"))
            .bind::<VarChar, _>(
                reminder_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )
            .execute(&connection);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
