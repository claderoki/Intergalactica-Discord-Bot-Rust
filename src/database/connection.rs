
fn get_db_opts() -> OptsBuilder {
    OptsBuilder::new()
    .user(Some(env::var("DB_USER").expect("Expected DB_USER in the environment")))
    .db_name(Some(env::var("DB_NAME").expect("Expected DB_NAME in the environment")))
    .ip_or_hostname(Some(env::var("DB_HOST").expect("Expected DB_HOST in the environment")))
    .pass(Some(env::var("DB_PASSWORD").expect("Expected DB_PASSWORD in the environment")))
}

fn get_connection() -> Conn {
    Conn::new(get_db_opts())
}