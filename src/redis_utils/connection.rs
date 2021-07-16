use redis::Connection;

pub fn get_connection_redis() -> Connection {
    // TODO: remove hardcoded local ip?
    redis::Client::open("redis://127.0.0.1/").unwrap().get_connection().unwrap()
    //     Err(_) => return Err(""),
    //     Ok(client) => match client.get_connection() {
    //         Ok(conn) => return Ok(conn),
    //         Err(_) => return Err(""),
    //     },
    // }
}
