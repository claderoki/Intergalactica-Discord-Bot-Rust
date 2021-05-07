use diesel::{RunQueryDsl, sql_query, sql_types::{BigInt, Nullable, VarChar, Bool, Integer}};

use crate::database::connection::get_connection_diesel;

#[derive(QueryableByName)]
pub struct PigeonSimplifiedActivity {
    #[sql_type = "Nullable<BigInt>"]
    pub id: Option<i64>,

    #[sql_type = "Nullable<VarChar>"]
    pub pigeon_status: Option<String>,

    #[sql_type = "Nullable<Bool>"]
    pub ready_to_be_retrieved: Option<bool>,
}

struct PigeonActivityRepository;
impl PigeonActivityRepository {

    pub fn get_activity(human_id: i32) -> Result<Option<PigeonSimplifiedActivity>, &'static str> {
        let connection = get_connection_diesel();

        let results: Result<PigeonSimplifiedActivity, _> = sql_query(
            "SELECT
                coalesce(exploration.id, mail.id, fight.id, date.id) as id,
                pigeon.status as type,
                (coalesce(exploration.end_date, mail.end_date, fight.end_date, date.end_date) <= UTC_DATE() ) as ready_to_be_retrieved
            FROM
            pigeon
            LEFT JOIN exploration
                ON (exploration.pigeon_id = pigeon.id AND exploration.finished = 0 AND pigeon.status = 'exploring')
            LEFT JOIN mail
                ON (mail.sender_id = pigeon.id AND mail.finished = 0 AND pigeon.status = 'mailing')
            LEFT JOIN fight
                ON ((fight.pigeon1_id = pigeon.id OR fight.pigeon2_id = pigeon.id) AND fight.finished = 0 AND pigeon.status = 'fighting')
            LEFT JOIN date
                ON ((date.pigeon1_id = pigeon.id OR date.pigeon2_id = pigeon.id) AND date.finished = 0  AND pigeon.status = 'dating')
            WHERE pigeon.human_id = ?")
        .bind::<Integer, _>(human_id)
        .get_result(&connection);

        match results {
            Ok(data) => {
                if data.id.is_none() {
                    return Ok(None);
                }
                Ok(Some(data))
            },
            Err(_) => {
                Err("Something went wrong retrieving the activity.")
            },
        }
    }
}
enum Planet {
    Mars = 1,
}

/*
RETRIEVAL STEPS
1. Check if there is a retrieval and if it is ready to be retrieved
2.

planet
id, name

space_retrieval
id, planet_id, start_time, end_time, finished

space_bonus
id, name



*/
