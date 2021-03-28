#![allow(dead_code)]

mod client;
mod commands;
mod database;
mod handler;
mod modules;
mod wrappers;

use client::get_client;
use tracing::error;


#[tokio::main]
async fn main() {
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
