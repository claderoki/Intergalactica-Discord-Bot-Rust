use std::env;

use mysql::{Conn, OptsBuilder};

fn get_db_opts() -> OptsBuilder {
    OptsBuilder::new()
        .user(Some(
            env::var("DB_USER").expect("Expected DB_USER in the environment"),
        ))
        .db_name(Some(
            env::var("DB_NAME").expect("Expected DB_NAME in the environment"),
        ))
        .ip_or_hostname(Some(
            env::var("DB_HOST").expect("Expected DB_HOST in the environment"),
        ))
        .pass(Some(
            env::var("DB_PASSWORD").expect("Expected DB_PASSWORD in the environment"),
        ))
}

pub fn get_connection() -> Result<Conn, mysql::Error> {
    Conn::new(get_db_opts())
}

use diesel::prelude::*;
use dotenv::dotenv;

pub fn get_connection_diesel() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
