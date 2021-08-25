use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType;
use serenity::model::interactions::message_component::ButtonStyle;

use crate::discord_helpers::embed_utils::EmbedExtension;

#[command("battle")]
#[only_in(guild)]
#[description("battle other pigeons.")]
pub async fn battle(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| e.normal_embed("Choose an action"))
                .components(|c| {
                    c.create_action_row(|a| {
                        a.create_button(|b| {
                            b.custom_id("claw")
                                .emoji(ReactionType::Unicode("💅".into()))
                                .style(ButtonStyle::Secondary)
                                .label("Claw")
                        })
                        .create_button(|b| {
                            b.custom_id("confuse")
                                .emoji(ReactionType::Unicode("💫".into()))
                                .style(ButtonStyle::Secondary)
                                .label("Confuse")
                        })
                        .create_button(|b| {
                            b.custom_id("seduce")
                                .emoji(ReactionType::Unicode("💋".into()))
                                .style(ButtonStyle::Secondary)
                                .label("Seduce")
                        })
                    })
                })
        })
        .await
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}
