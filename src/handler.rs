use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use serenity::async_trait;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use tracing::info;

use crate::modules::pigeon::tasks::decay::decay_pigeons;
use crate::modules::shared::tasks::reminders::reminder;

pub struct Handler {
    is_loop_running: AtomicBool,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            is_loop_running: AtomicBool::new(false),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    // async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    //     println!("got something");

    //     if let Some(data) = interaction.data.as_ref() {
    //         match data {
    //             InteractionData::ApplicationCommand(_) => {},
    //             InteractionData::MessageComponent(value) => {
    //                 println!("{:?}", value);
    //             },
    //         };
    //     }
    // }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);
        if !self.is_loop_running.load(Ordering::Relaxed) {
            // let ctx1 = Arc::clone(&ctx);
            // tokio::spawn(async move {
            //     loop {
            //         tokio::time::sleep(Duration::from_secs(60*60)).await;
            //         decay_pigeons(Arc::clone(&ctx1)).await;
            //     }
            // });

            let ctx2 = Arc::clone(&ctx);
            tokio::spawn(async move {
                loop {
                    reminder(Arc::clone(&ctx2)).await;
                    tokio::time::sleep(Duration::from_secs(20)).await;
                }
            });

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
}
