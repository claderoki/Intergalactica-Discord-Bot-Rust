use std::sync::Arc;

use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use serenity::client::Context;

pub async fn unjail_all(ctx: Arc<Context>) {
    let result = PigeonRepository::unjail_all();
}
