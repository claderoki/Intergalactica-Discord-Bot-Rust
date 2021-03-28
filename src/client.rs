use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{StandardFramework},
    http::Http,
    prelude::*,
};

use std::{
    collections::{HashSet},
    env,
    sync::Arc,
};

use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::modules::pigeon::commands::{base::*};

use crate::Handler;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

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

pub async fn get_client() -> Client {
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

    let client = Client::builder(&token)
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
