//! Requires the 'framework' feature flag be enabled in your project's
//! `Cargo.toml`.
//!
//! This can be enabled by specifying the feature in the dependency section:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["framework", "standard_framework"]
//! ```
mod commands;

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{channel::Message, event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use commands::{math::*, meta::*, owner::*};

use regex::Regex;

mod modules;
use modules::conversion::core;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

pub fn clean_value(value: f64) -> String {
    if value % 1.0 == 0.0 {
        return format!("{}", (value as i64));
    }

    return format!("{0:.2}", value);
}

pub fn add_conversion_result_to_embed(result: &core::ConversionResult) -> (String, String, bool) {
    let mut value_field: String = String::from("").to_owned();

    for conversion in result.to.iter() {
        value_field.push_str(conversion.to_string().as_str());
    }

    (result.base.to_string(), value_field, false)
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        let re = Regex::new(r"([+-]?\d+(\.\d+)*)(c|f)(?:$|\n| )?").unwrap();

        let mut vec = Vec::new();
        for cap in re.captures_iter(&message.content) {
            let value = cap[1].parse::<f64>().unwrap_or(0.0).to_owned();
            let unit = cap[3].to_lowercase();
            let r = core::convert_measurement(value, unit);

            match r {
                Ok(result) => {
                    let r = add_conversion_result_to_embed(&result);
                    vec.push(r);
                }
                Err(_) => {}
            };
        }
        if !vec.is_empty() {
            message
                .channel_id
                .send_message(&ctx, |m| m.embed(|e| e.fields(vec)))
                .await
                .unwrap();
        }
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(multiply, ping, quit)]
struct General;

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    // Initialize the logger to use environment variables.
    //
    // In this case, a good default is setting the environment variable
    // `RUST_LOG` to debug`.
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
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
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

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
