use std::time::Duration;

use serenity::{
    builder::CreateEmbed,
    client::Context,
    model::{
        channel::{Message, ReactionType},
        interactions::{ButtonStyle, InteractionData, InteractionResponseType},
    },
};

pub trait Choosable {
    fn get_identifier(&self) -> i32;
    fn get_description(&self) -> String;
    fn get_emoji(&self) -> Option<String>;
}

// pub struct KeyCap;

// impl KeyCap {
//     pub fn get(number: i32) -> String {
//         let mut emoji = String::from("");

//         for char in number.to_string().chars() {
//             emoji.push_str(match char {
//                 '0' => "0️⃣",
//                 '1' => "1️⃣",
//                 '2' => "2️⃣",
//                 '3' => "3️⃣",
//                 '4' => "4️⃣",
//                 '5' => "5️⃣",
//                 '6' => "6️⃣",
//                 '7' => "7️⃣",
//                 '8' => "8️⃣",
//                 '9' => "9️⃣",
//                 _ => "",
//             });
//         }

//         emoji
//     }

// pub fn to_number(keycap: String) -> i32 {
//     return match keycap.as_str() {
//         "0️⃣" => 0,
//         "1️⃣" => 1,
//         "2️⃣" => 2,
//         "3️⃣" => 3,
//         "4️⃣" => 4,
//         "5️⃣" => 5,
//         "6️⃣" => 6,
//         "7️⃣" => 7,
//         "8️⃣" => 8,
//         "9️⃣" => 9,
//         _ => -1
//     }
// }
// }

// pub fn flatten_choosables<T>(
//     choosables: &Vec<T>,
// ) -> Result<(Vec<ReactionType>, String), &'static str>
// where
//     T: Choosable,
// {
//     let mut text = String::from("");

//     let mut i: i32 = 0;
//     let mut emojis: Vec<ReactionType> = Vec::new();
//     let mut any_custom_emoji = false;

//     for choosable in choosables.iter() {
//         if i > 0 {
//             text.push_str("\n");
//         }

//         let emoji = choosable.get_emoji();
//         if emoji.is_some() {
//             any_custom_emoji = true;
//             let emoji = emoji.unwrap();
//             emojis.push(ReactionType::Unicode((*emoji).to_string()));
//             text.push_str(&emoji);
//         } else if any_custom_emoji {
//             return Err("All need to have emoji or none");
//         } else {
//             let emoji = &KeyCap::get(i + 1);
//             emojis.push(ReactionType::Unicode(String::from(emoji.as_str())));
//             text.push_str(&emoji);
//         }

//         text.push_str(" ");
//         text.push_str(&choosable.get_description());

//         i += 1;
//     }

//     Ok((emojis, text))
// }

pub async fn choose<T, F>(
    msg: &Message,
    ctx: &Context,
    choosables: &Vec<T>,
    f: F,
) -> Result<usize, &'static str>
where
    T: Choosable,
    F: FnOnce(&mut CreateEmbed, String) -> &mut CreateEmbed,
{
    let interactive_msg = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| f(e, "".into())).components(|c| {
                c.create_action_row(|f| {
                    let mut i = 0;
                    for choosable in choosables.iter() {
                        f.create_button(|b| {
                            b.style(ButtonStyle::Secondary)
                                .custom_id(i)
                                .label(choosable.get_description())
                                .emoji(ReactionType::Unicode(
                                    choosable.get_emoji().unwrap().to_string(),
                                ))
                        });
                        i += 1;
                    }
                    f
                })
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
