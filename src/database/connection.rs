use diesel::prelude::Connection;
use diesel::prelude::MysqlConnection;
use std::env;

// const fn abc() -> String {
//     format!(
//         "mysql://{}:{}@{}/{}",
//         env::var("DB_USER").expect("Expected DB_USER in the environment"),
//         env::var("DB_PASSWORD").expect("Expected DB_PASSWORD in the environment"),
//         env::var("DB_HOST").expect("Expected DB_HOST in the environment"),
//         env::var("DB_NAME").expect("Expected DB_NAME in the environment"),
//     )
// }

// static DB_URL: &str = ;

fn get_db_url() -> String {
    format!(
        "mysql://{}:{}@{}/{}",
        env::var("DB_USER").expect("Expected DB_USER in the environment"),
        env::var("DB_PASSWORD").expect("Expected DB_PASSWORD in the environment"),
        env::var("DB_HOST").expect("Expected DB_HOST in the environment"),
        env::var("DB_NAME").expect("Expected DB_NAME in the environment"),
    )
}

pub fn get_connection_diesel() -> Result<MysqlConnection, &'static str> {
    MysqlConnection::establish(&get_db_url()).map_err(|_|"Couldn't connect to db.")
    // MysqlConnection::establish(&get_db_url()).unwrap_or_else(|_| panic!("Error connecting to db"))
}
