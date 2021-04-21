use diesel::prelude::{Connection, MysqlConnection};
use std::env;

fn get_db_url() -> String {
    let user = env::var("DB_USER").expect("Expected DB_USER in the environment");
    let name = env::var("DB_NAME").expect("Expected DB_NAME in the environment");
    let host = env::var("DB_HOST").expect("Expected DB_HOST in the environment");
    let pass = env::var("DB_PASSWORD").expect("Expected DB_PASSWORD in the environment");

    let mut url = String::from("mysql://");
    url.push_str(user.as_str());
    url.push_str(":");
    url.push_str(pass.as_str());
    url.push_str("@");
    url.push_str(host.as_str());
    url.push_str("/");
    url.push_str(name.as_str());
    url
}

pub fn get_connection_diesel() -> MysqlConnection {
    let database_url = get_db_url();
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {}: {}", database_url, e))
}
