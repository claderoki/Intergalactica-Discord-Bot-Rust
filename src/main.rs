use client::get_client;
use redis_utils::connection::get_connection_redis;
use tracing::error;

#[macro_use]
extern crate diesel;

mod client;
mod database;
mod handler;
mod modules;
mod redis_utils;
mod wrappers;
mod discord_helpers;

#[tokio::main]
async fn main() {
    get_connection_redis().expect("redis not initialized");

    let mut client = get_client().await;
    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(e) = client.start().await {
        error!("Client error: {:?}", e);
    }
}
