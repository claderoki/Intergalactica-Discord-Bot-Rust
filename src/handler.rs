use serenity::{
    async_trait,
    model::{channel::Message, event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use tracing::info;

use crate::modules::{conversion::core::match_conversion, shared::repository::human::get_or_create_human};
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.author.bot {
            return ();
        }

        match_conversion(message.content.as_str());

        get_or_create_human(*message.author.id.as_u64());
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
