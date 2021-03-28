mod commands;
mod modules;
mod wrappers;
mod client;
mod handler;

use std::collections::HashMap;

use client::get_client;

use tracing::{error};

use once_cell::sync::Lazy;

pub static SYMBOLS: Lazy<std::sync::Mutex<HashMap<String, String>>> =
    Lazy::new(|| std::sync::Mutex::new(HashMap::new()));

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

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
