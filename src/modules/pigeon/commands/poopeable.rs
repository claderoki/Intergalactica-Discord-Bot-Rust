use chrono::Duration;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::guild::Member;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::discord_helpers::ui::GoldConfirmation;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::caching::bucket::Bucket;

#[command("poopeable")]
#[only_in(guild)]
#[description("Find a poopeable pigeon.")]
pub async fn poopeable(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id.0 != 120566758091259906 {
        return Err("Not author".into());
    }

    let bucket = Bucket::user("pigeon_poopeable", msg.author.id, Duration::minutes(60));
    let now = bucket.validate()?;
    let cost = 100;

    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .gold_needed(cost)
        .validate(&msg.author)?;

    if !GoldConfirmation::new().confirm(ctx, msg, cost).await? {
        return Err("Cancelled".into());
    }

    let member = get_member(ctx, msg).await.ok_or("No members found.")?;

    match msg
        .author
        .dm(&ctx, |m| m.embed(|e| e.normal_embed(format!("{}", member))))
        .await
    {
        Ok(_) => {}
        Err(_) => {}
    }

    bucket.spend(now);
    Ok(())
}

pub async fn get_member(ctx: &Context, msg: &Message) -> Option<Member> {
    if let Some(guild) = msg.guild(ctx).await {
        println!("Guild found.");
        if let Ok(user_ids) = PigeonRepository::get_idle_pigeon_users() {
            println!("User ids found.");
            for user_id in user_ids {
                if let Ok(member) = guild.member(ctx, user_id.value).await {
                    println!("Member found.");
                    return Some(member);
                }
            }
        }
    }

    None
}
