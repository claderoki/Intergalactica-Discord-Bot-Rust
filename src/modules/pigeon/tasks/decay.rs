use std::sync::Arc;

use serenity::client::Context;
use serenity::model::id::UserId;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::modules::pigeon::helpers::utils::PigeonWinningsBuilder;
use crate::modules::pigeon::models::pigeon::DecayingPigeon;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;

pub async fn decay_pigeons(ctx: Arc<Context>) {
    let pigeons_result = PigeonRepository::get_decaying_pigeons();
    match pigeons_result {
        Ok(pigeons) => {
            for pigeon in pigeons.iter() {
                decay_pigeon(&ctx, pigeon).await;
            }
        }
        Err(e) => println!("{:?}", e),
    }
}

async fn decay_pigeon(ctx: &Context, pigeon: &DecayingPigeon) {
    let mut builder = PigeonWinningsBuilder::new();
    builder.cleanliness(-1).happiness(-1).food(-1);

    if pigeon.cleanliness <= 20 || pigeon.food <= 20 {
        builder.health(-1);
    }

    let new_health = pigeon.health + builder.health;
    if (new_health <= 20 && new_health > 17) || (new_health <= 6 && new_health > 2) {
        if let Err(e) = Notification::danger("Your pigeon is about to die!")
            .send(&ctx, UserId { 0: pigeon.user_id })
            .await
        {
            println!("{:?}", e);
        }
    }

    let result = PigeonRepository::update_winnings(pigeon.human_id, &builder.build());
    match result {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e)
        }
    }
}

enum NotificationType {
    // Warning,
    // Success,
    // Info,
    Danger,
}

struct Notification {
    notification_type: NotificationType,
    message: String,
}

impl Notification {
    fn new(notification_type: NotificationType, message: &'static str) -> Self {
        Self {
            notification_type,
            message: message.into(),
        }
    }

    // pub fn warning(message: &'static str) -> Self {
    //     Notification::new(NotificationType::Warning, message)
    // }

    // pub fn success(message: &'static str) -> Self {
    //     Notification::new(NotificationType::Success, message)
    // }

    // pub fn info(message: &'static str) -> Self {
    //     Notification::new(NotificationType::Info, message)
    // }

    pub fn danger(message: &'static str) -> Self {
        Notification::new(NotificationType::Danger, message)
    }

    pub async fn send(&self, ctx: &Context, user_id: UserId) -> Result<(), &'static str> {
        let user = user_id.to_user(&ctx).await.or(Err("User not found."))?;

        let result = user
            .dm(&ctx, |m| {
                m.embed(|e| {
                    match self.notification_type {
                        // NotificationType::Warning => e.warning_color(),
                        // NotificationType::Success => e.success_color(),
                        // NotificationType::Info => e.default_color(),
                        NotificationType::Danger => e.danger_color(),
                    };
                    e.description(&self.message)
                })
            })
            .await;

        if let Err(e) = result {
            println!("{:?}", e);
            return Err("Couldn't send notification");
        }

        Ok(())
    }
}
