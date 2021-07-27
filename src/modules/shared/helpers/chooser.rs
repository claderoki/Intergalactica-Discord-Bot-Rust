use std::time::Duration;

use serenity::builder::CreateButton;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType;
use serenity::model::interactions::ButtonStyle;
use serenity::model::interactions::InteractionData;
use serenity::model::interactions::InteractionResponseType;

use crate::discord_helpers::embed_utils::EmbedExtension;

pub trait Choosable {
    fn get_identifier(&self) -> i32;
    fn get_description(&self) -> String;
    fn get_emoji(&self) -> Option<String>;
}

pub async fn choose<T, F>(
    ctx: &Context,
    msg: &Message,
    choosables: &Vec<T>,
    f: F,
) -> Result<usize, &'static str>
where
    T: Choosable,
    F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
{
    let interactive_msg = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| f(e)).components(|c| {
                // c.create_action_row(|f| {
                //     f.create_select_menu(|s| {
                //         s.min_values(1)
                //             .placeholder("hmm")
                //             .max_values(2)
                //             .custom_id("abc")
                //             .options(|m| {
                //                 m.create_option(|o| {
                //                     o.description("Option 1").label("1").value("option 1")
                //                 })
                //                 .create_option(|o| {
                //                     o.description("Option 2").label("2").value("option 2")
                //                 })
                //         })
                //     })
                // });

                let length = choosables.len();
                const MAX_ELEMENTS_PER_ROW: usize = 5;
                let remainder = length % MAX_ELEMENTS_PER_ROW;
                let row_count = (length / MAX_ELEMENTS_PER_ROW) + remainder;
                let mut index = 0;

                for i in 0..row_count {
                    let last = if i == row_count - 1 && remainder != 0 {
                        length
                    } else {
                        MAX_ELEMENTS_PER_ROW
                    };

                    c.create_action_row(|f| {
                        for choosable in choosables
                            .get((i * MAX_ELEMENTS_PER_ROW)..last)
                            .unwrap()
                            .iter()
                        {
                            f.create_button(|b| {
                                create_button_for_choosable::<T>(b, &choosable, index)
                            });
                            index += 1;
                        }
                        f
                    });
                }
                c
            })
        })
        .await
        .or(Err("Oops"))?;

    let interaction = &interactive_msg
        .await_component_interaction(&ctx)
        .author_id(msg.author.id)
        .timeout(Duration::from_secs(60))
        .await
        .ok_or("Timed out...")?;

    let _ = interaction
        .create_interaction_response(&ctx, |f| {
            f.kind(InteractionResponseType::DeferredUpdateMessage)
        })
        .await;

    match interaction.data.as_ref().ok_or("")? {
        InteractionData::ApplicationCommand(_) => Err("Wrong type of interaction"),
        InteractionData::MessageComponent(value) => match value.custom_id.parse::<usize>() {
            Ok(index) => Ok(index),
            Err(_) => Err("Can't convert to int"),
        },
    }
}

fn create_button_for_choosable<'a, T>(
    button: &'a mut CreateButton,
    choosable: &T,
    index: usize,
) -> &'a mut CreateButton
where
    T: Choosable,
{
    button
        .style(ButtonStyle::Secondary)
        .custom_id(index)
        .label(choosable.get_description())
        .emoji(ReactionType::Unicode(
            choosable.get_emoji().unwrap_or("".to_string()).to_string(),
        ))
}

pub async fn confirm(ctx: &Context, msg: &Message, message: String) -> Result<bool, &'static str> {
    let interactive_msg = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| e.normal_embed(message)).components(|c| {
                c.create_action_row(|f| {
                    f.create_button(|b| b.style(ButtonStyle::Success).custom_id(1).label("Yes"));
                    f.create_button(|b| b.style(ButtonStyle::Danger).custom_id(0).label("No"));
                    f
                })
            })
        })
        .await
        .or(Err("Couldn't send confirmation message for some reason"))?;

    let interaction = &interactive_msg
        .await_component_interaction(&ctx)
        .author_id(msg.author.id)
        .timeout(Duration::from_secs(60))
        .await
        .ok_or("Timed out...")?;

    let _ = interaction
        .create_interaction_response(&ctx, |f| {
            f.kind(InteractionResponseType::DeferredUpdateMessage)
        })
        .await;

    match interaction.data.as_ref().ok_or("")? {
        InteractionData::ApplicationCommand(_) => Err("Wrong type of interaction"),
        InteractionData::MessageComponent(value) => match value.custom_id.parse::<usize>() {
            Ok(index) => Ok(index == 1),
            Err(_) => Err("Can't convert to bool"),
        },
    }
}
