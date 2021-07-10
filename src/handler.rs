use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;

use serenity::async_trait;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use tracing::info;

use crate::modules::pigeon::tasks::decay::decay_pigeons;

pub struct Handler {
    is_loop_running: AtomicBool,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            is_loop_running: AtomicBool::new(false)
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);
        if !self.is_loop_running.load(Ordering::Relaxed) {

            tokio::spawn(async move {
                loop {
                    decay_pigeons(Arc::clone(&ctx)).await;
                    tokio::time::sleep(Duration::from_secs(120)).await;
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}
