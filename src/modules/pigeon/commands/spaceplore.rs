use std::time::Duration;

use chrono::NaiveDateTime;
use serenity::builder::CreateComponents;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType;
use serenity::model::interactions::ButtonStyle;
use serenity::model::interactions::InteractionData;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::User;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::exploration::ExplorationRepository;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::repository::reminder::NewReminder;
use crate::modules::shared::repository::reminder::ReminderRepository;

#[command("spaceplore")]
#[description("Send your pigeon into space.")]
pub async fn spaceplore(ctx: &Context, msg: &Message) -> CommandResult {
    let human_id = PigeonValidation::new()
        // .item_needed("space_shuttle")
        .needs_active_pigeon(true)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    let simple_location = ExplorationRepository::get_random_location()?;

    let arrival_date = (chrono::offset::Utc::now() + chrono::Duration::minutes(30)).naive_utc();

    ExplorationRepository::create_exploration(human_id, simple_location.id, arrival_date)?;
    PigeonRepository::update_status(human_id, PigeonStatus::SpaceExploring);
    success_scenario(msg, ctx, simple_location.image_url, arrival_date).await?;

    Ok(())
}

fn create_reminder_components<'a>(components: &'a mut CreateComponents) -> &'a mut CreateComponents {
    components.create_action_row(|a| a.create_button(|b| {
        b.custom_id("reminder")
        .emoji(ReactionType::Unicode("❗".into()))
        .style(ButtonStyle::Secondary)
        .label("Remind me")
    }))
}

async fn should_remind(ctx: &Context, msg: &Message, user: &User) -> bool {
    let interaction_result = &msg
        .await_component_interaction(&ctx)
        .author_id(user.id)
        .timeout(Duration::from_secs(120))
        .await;

    match interaction_result {
        Some(interaction) => {
            let _ = interaction
                .create_interaction_response(&ctx, |f| {
                    f.kind(InteractionResponseType::DeferredUpdateMessage)
                })
                .await;
                if let Some(data) = interaction.data.as_ref() {
                    if let InteractionData::MessageComponent(value) = data {
                        if value.custom_id == "reminder" {
                            return true;
                        }
                    }
                }
            false
        },
        None => false,
    }

}

async fn success_scenario(msg: &Message, ctx: &Context, image_url: String, arrival_date: NaiveDateTime) -> Result<(), String> {
    let interactive_msg = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.components(|c|create_reminder_components(c))
            .embed(|e| {
                e.normal_embed("Your pigeon has successfully taken off to space!")
                    .thumbnail(image_url)
            })
        })
        .await;

        match interactive_msg {
            Ok(message) => {
                if should_remind(ctx, &message, &msg.author).await {
                    let text = format!("Your pigeon has landed on {}", "Luna");

                    let mut reminder = NewReminder::new(msg.author.id.into(), text, arrival_date);
                    reminder.channel_id(msg.channel_id.into());
                    let result = ReminderRepository::create(&reminder);
                    match result {
                        Ok(_) => {
                            let _ = msg
                            .channel_id
                            .send_message(&ctx, |m| {
                                m.embed(|e| e.normal_embed("Okay, I will remind you when your pigeon has arrived."))
                            })
                            .await;
                        },
                        Err(e) => {return Err(format!("{:?}", e));},
                    }
                }
            },
            Err(e) => {return Err(format!("{:?}", e));}
        }

        Ok(())
}
