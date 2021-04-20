use serenity::{
    async_trait,
    model::{channel::Message, event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    // async fn message(&self, ctx: Context, message: Message) {
    //     if message.author.bot {
    //         return ();
    //     }
    // }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
