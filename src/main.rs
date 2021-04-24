// #![allow(dead_code)]
#[macro_use]
extern crate diesel;

mod client;
mod database;
mod handler;
mod modules;
mod wrappers;

use client::get_client;
use tracing::error;

// extern crate cairo;
// use cairo::{ ImageSurface, Format, Context };
// use std::fs::File;

// let surface = ImageSurface::create(Format::ARgb32, 600, 600)?;
// let context = Context::new(&surface);
// context.set_source_rgb(1.0, 0.0, 0.0);
// context.paint();

// let mut file = File::create("output.png")?;
// surface.write_to_png(&mut file);


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
