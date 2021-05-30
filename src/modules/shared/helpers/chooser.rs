use std::{time::Duration};

use serenity::{builder::CreateEmbed, client::Context, model::channel::{Message, ReactionType}};


pub trait Choosable {
    fn get_identifier(&self) -> i32;
    fn get_description(&self) -> String;
    fn get_emoji(&self) -> Option<String>;
}


pub struct KeyCap;

impl KeyCap {
    pub fn get(number: i32) -> String {
        let mut emoji = String::from("");

        for char in number.to_string().chars() {
            emoji.push_str(match char {
                '0' => "0️⃣",
                '1' => "1️⃣",
                '2' => "2️⃣",
                '3' => "3️⃣",
                '4' => "4️⃣",
                '5' => "5️⃣",
                '6' => "6️⃣",
                '7' => "7️⃣",
                '8' => "8️⃣",
                '9' => "9️⃣",
                _ => ""
            });
        }

        emoji
    }

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
}

pub async fn choose<T, F>(msg: &Message, ctx: &Context, choosables: &Vec<T>, f: F) -> Result<usize, &'static str>
where T: Choosable, F: FnOnce(&mut CreateEmbed, String) -> &mut CreateEmbed,
{
    let mut text = String::from("");

    let mut i: i32 = 0;
    let mut emojis: Vec<ReactionType> = Vec::new();
    let mut any_custom_emoji = false;

    for choosable in choosables.iter() {
        if i > 0 {
            text.push_str("\n");
        }

        let emoji = choosable.get_emoji();
        if emoji.is_some() {
            any_custom_emoji = true;
            let emoji = emoji.unwrap();
            emojis.push(ReactionType::Unicode((*emoji).to_string()));
            text.push_str(&emoji);
        } else if any_custom_emoji {
            return Err("All need to have emoji or none");
        } else {
            let emoji = &KeyCap::get(i+1);
            emojis.push(ReactionType::Unicode(String::from(emoji.as_str())));
            text.push_str(&emoji);
        }

        text.push_str(" ");
        text.push_str(&choosable.get_description());

        i += 1;
    }

    let embed_message = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e|f(e, text))
        })
        .await.or(Err("Oops"))?;

    for emoji in emojis.iter() {
        let _ = embed_message.react(ctx, emoji.clone()).await;
    }

    let reaction = &msg
        .author
        .await_reaction(&ctx)
        .author_id(msg.author.id)
        .message_id(embed_message.id)
        .filter(|_r| true) // emojis.contains(&r.emoji)
        .timeout(Duration::from_secs(60))
        .await
        .ok_or("No emoji given")?;

    let index = emojis.iter().position(|e| e == &reaction.as_inner_ref().emoji).ok_or("err")?;
    Ok(index)
}
