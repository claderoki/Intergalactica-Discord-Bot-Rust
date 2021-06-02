use diesel::prelude::{Connection, MysqlConnection};
use std::env;

fn get_db_url() -> String {
    format!("mysql://{}:{}@{}/{}",
        env::var("DB_USER").expect("Expected DB_USER in the environment"),
        env::var("DB_NAME").expect("Expected DB_NAME in the environment"),
        env::var("DB_HOST").expect("Expected DB_HOST in the environment"),
        env::var("DB_PASSWORD").expect("Expected DB_PASSWORD in the environment"),
    )
}

pub fn get_connection_diesel() -> MysqlConnection {
    let database_url = get_db_url();
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {}: {}", database_url, e))
}
