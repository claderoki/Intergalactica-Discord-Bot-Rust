use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::CommandError;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::prelude::*;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use tracing_subscriber::EnvFilter;
use tracing_subscriber::FmtSubscriber;

use crate::modules::games::base::*;
use crate::modules::pigeon::commands::base::*;

use crate::handler::Handler;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[hook]
async fn after_hook(
    ctx: &Context,
    msg: &Message,
    _cmd_name: &str,
    error: Result<(), CommandError>,
) {
    if let Err(why) = error {
        let _ = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| e.color(serenity::utils::Color::RED).description(why))
            })
            .await;
    }
}

pub enum Environment {
    Production,
    Development,
}

impl TypeMapKey for Environment {
    type Value = Environment;
}

fn get_environment() -> Environment {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(mode) => {
            match mode.as_str() {
                "production" => Environment::Production,
                _ => Environment::Development
            }

        },
        None => Environment::Development,
    }
}

pub async fn get_client() -> Client {
    dotenv::dotenv().expect("Failed to load .env file");

    let environment = get_environment();

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, application_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(match environment {
            Environment::Production => "/",
            Environment::Development => ".",
        }))
        .after(after_hook)
        .group(&GAMES_GROUP)
        .group(&PIGEON_GROUP);

    let client = Client::builder(&token)
        .framework(framework)
        .application_id(application_id.into())
        .event_handler(Handler::new())
        .intents(GatewayIntents::all())
        .await
        .expect("Err creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<Environment>(environment);
    }

    client
}
