use redis::Connection;

pub fn get_connection_redis() -> Result<Connection, &'static str> {
    // TODO: remove hardcoded local ip?
    match redis::Client::open("redis://127.0.0.1/") {
        Err(_) =>  return Err(""),
        Ok(client) => {
            match client.get_connection() {
                Ok(conn) => return Ok(conn),
                Err(_) => {
                    return Err("")
                }
            }
        }
    }
}
