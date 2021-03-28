use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{StandardFramework},
    http::Http,
    model::{channel::Message, event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};
pub static SYMBOLS: Lazy<std::sync::Mutex<HashMap<String, String>>> =
    Lazy::new(|| std::sync::Mutex::new(HashMap::new()));

use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use modules::pigeon::commands::{base::*};

mod commands;
mod modules;
mod wrappers;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
struct Handler;

trait Utils {
    fn get_color(&self) -> serenity::utils::Color;
    fn translate(&self, key: &'static str) -> String;
}

impl Utils for Context {
    fn get_color(&self) -> serenity::utils::Color {
        serenity::utils::Color::from_rgb(242, 181, 37)
    }
    fn translate(&self, key: &'static str) -> String {
        String::from(key)
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

    }

    async fn message(&self, ctx: Context, message: Message) {

    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

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

async fn get_client() -> Client {
    dotenv::dotenv().expect("Failed to load .env file");

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("~"))
        .group(&PIGEON_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    client
}
