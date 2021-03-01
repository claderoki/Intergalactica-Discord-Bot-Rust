#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    PgConnection::establish(&db_host).expect(&format!("Error connecting to {}", db_host))
}