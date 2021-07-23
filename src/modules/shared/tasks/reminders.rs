use std::sync::Arc;

use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::channel::GuildChannel;
use serenity::model::prelude::User;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::modules::shared::repository::reminder::Reminder;
use crate::modules::shared::repository::reminder::ReminderRepository;

pub async fn reminder(ctx: Arc<Context>) {
    let reminders_result = ReminderRepository::get_due_reminders();

    let mut reminded: Vec<i32> = Vec::new();

    match reminders_result {
        Ok(reminders) => {
            for reminder in reminders.iter() {
                if remind(&ctx, reminder).await {
                    reminded.push(reminder.id);
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }

    if !reminded.is_empty() {
        if let Err(e) = ReminderRepository::set_sent_multiple(reminded) {
            println!("{:?}", e);
        }
    }
}

fn create_reminder_embed<'a>(embed: &'a mut CreateEmbed, message: &String) -> &'a mut CreateEmbed {
    embed.normal_embed(message)
}

async fn remind(ctx: &Context, reminder: &Reminder) -> bool {
    if let Some(channel_id) = reminder.channel_id {
        match ctx.cache.guild_channel(channel_id).await {
            Some(channel) => {
                match remind_channel(ctx, reminder.user_id, &channel, &reminder.message).await {
                    Ok(_) => {
                        return true;
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        return false;
                    }
                }
            }
            None => {
                println!("Channel not in cache");
                return false;
            }
        }
    } else {
        match ctx.cache.user(reminder.user_id).await {
            Some(user) => match remind_user(ctx, &user, &reminder.message).await {
                Ok(_) => {
                    return true;
                }
                Err(e) => {
                    println!("{:?}", e);
                    return false;
                }
            },
            None => {
                println!("User not found in cache");
                return false;
            }
        }
    }
}

async fn remind_user(ctx: &Context, user: &User, message: &String) -> Result<(), String> {
    match user
        .dm(&ctx, |m| m.embed(|e| create_reminder_embed(e, message)))
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err("Nope".into()),
    }
}

async fn remind_channel(
    ctx: &Context,
    user_id: u64,
    channel: &GuildChannel,
    message: &String,
) -> Result<(), String> {
    match channel
        .send_message(&ctx, |m| {
            m.content(format!("<@{}>", user_id))
                .embed(|e| create_reminder_embed(e, &message))
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err("Nope".into()),
    }
}
