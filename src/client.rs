use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{
        standard::{CommandError, DispatchError},
        StandardFramework,
    },
    http::Http,
    model::channel::Message,
    prelude::*,
};

use std::{collections::HashSet, env, sync::Arc};

use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::modules::pigeon::commands::base::*;

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

#[hook]
async fn before_hook(ctx: &Context, msg: &Message, cmd_name: &str) -> bool {
    // println!("Running command {}", cmd_name);
    true
}

#[hook]
async fn after_hook(ctx: &Context, msg: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    if let Err(why) = error {
        let _ = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.color(serenity::utils::Color::from_rgb(255, 0, 0))
                        .description(why)
                })
            })
            .await;
    }
}

#[hook]
async fn dispatch_error_hook(context: &Context, msg: &Message, error: DispatchError) {
    // match msg
    //     .channel_id
    //     .say(&context, format!("{:?}", error).as_str())
    //     .await
    // {
    //     Err(e) => {
    //         println!("{:?}", e);
    //     }
    //     Ok(data) => {
    //         println!("{:?}", data)
    //     }
    // }

    // match error {
    //     DispatchError::NotEnoughArguments { min, given } => {
    //         let s = format!("Need {} arguments, but only got {}.", min, given);

    //         let _ = msg.channel_id.say(&context, &s).await;
    //     }
    //     DispatchError::TooManyArguments { max, given } => {
    //         let s = format!("Max arguments allowed is {}, but got {}.", max, given);

    //         let _ = msg.channel_id.say(&context, &s).await;
    //     }
    //     _ => println!("Unhandled dispatch error."),
    // }
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

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("~"))
        .before(before_hook)
        .after(after_hook)
        // .on_dispatch_error(dispatch_error_hook)
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
