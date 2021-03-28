use serenity::{
    async_trait,
    model::{channel::Message, event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use tracing::info;

use crate::modules::shared::repository::human::get_or_create_human;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        let result = get_or_create_human(*message.author.id.as_u64());
        match result {
            Ok(human) => {
                println!("{:?}", human);
            },
            Err(e) => {
                println!("{:?}", e);
            }
        };
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
