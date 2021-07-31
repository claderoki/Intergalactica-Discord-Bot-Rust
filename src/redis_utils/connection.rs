use redis::Connection;

pub fn get_connection_redis() -> Result<Connection, &'static str> {
    // TODO: remove hardcoded local ip?
    let client =
        redis::Client::open("redis://127.0.0.1/").map_err(|_| "Failed to get redis client")?;
    client
        .get_connection()
        .map_err(|_| "Failed to get Redis connection")
}
