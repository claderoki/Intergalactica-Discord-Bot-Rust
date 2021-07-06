use serenity::{client::Context, model::channel::Message};

use super::utils::PigeonWinnings;
use crate::discord_helpers::embed_utils::EmbedExtension;

pub async fn winnings_message(
    ctx: &Context,
    msg: &Message,
    winnings: &PigeonWinnings,
    message: String,
) -> Result<(), &'static str> {
    let text = format!("{}\n{}", message, winnings.to_string());

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| m.embed(|e| e.normal_embed(&text)))
        .await
        .or(Err("Failed to send heal"));

    Ok(())
}
