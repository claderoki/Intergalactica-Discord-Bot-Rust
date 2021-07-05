use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::{
    discord_helpers::embed_utils::EmbedExtension,
    modules::pigeon::{
        helpers::{
            utils::{PigeonWinnings, PigeonWinningsBuilder},
            validation::PigeonValidation,
        },
        models::pigeon::PigeonStatus,
        repository::pigeon::PigeonRepository,
    },
};

#[command("heal")]
#[description("Heal your pigeon.")]
pub async fn heal(ctx: &Context, msg: &Message) -> CommandResult {
    let cost = 20;
    let increase = 20;

    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .gold_needed(cost)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    let winnings = PigeonWinningsBuilder::new()
        .food(increase)
        .health(-cost)
        .build();

    heal_message(ctx, msg, &winnings).await?;
    PigeonRepository::update_winnings(human_id, &winnings);
    Ok(())
}

pub async fn heal_message(
    ctx: &Context,
    msg: &Message,
    winnings: &PigeonWinnings,
) -> Result<(), &'static str> {
    let text = format!(
        "You give your pigeon some health. It's health is refilled!\n{}",
        winnings.to_string()
    );

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| m.embed(|e| e.normal_embed(&text)))
        .await
        .or(Err("Failed to send heal"));

    Err("")
}
