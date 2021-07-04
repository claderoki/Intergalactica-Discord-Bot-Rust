use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{
        standard::{CommandError},
        StandardFramework,
    },
    http::Http,
    model::channel::Message,
    prelude::*,
};

use std::{collections::HashSet, env, sync::Arc};

use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::modules::pigeon::commands::base::*;
use crate::modules::games::base::*;

use crate::handler::Handler;

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

use serenity::framework::standard::macros::hook;

// #[hook]
// async fn before_hook(ctx: &Context, msg: &Message, cmd_name: &str) -> bool {
//     // println!("Running command {}", cmd_name);
//     true
// }

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

pub async fn get_client() -> Client {
    // dotenv::from_filename("dev.env")
    // dotenv::from_filename("prod.env")

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

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("~"))
        .after(after_hook)
        .group(&GAMES_GROUP)
        .group(&PIGEON_GROUP);

    let client = Client::builder(&token)
        .framework(framework)
        .application_id(742365922244952095)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    client
}
