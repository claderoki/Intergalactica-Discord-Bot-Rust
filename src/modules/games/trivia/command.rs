use std::collections::HashMap;
extern crate htmlescape;
use rand::thread_rng;
use rand::Rng;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::futures::StreamExt;
use serenity::model::channel::Message;
use serenity::model::interactions::message_component::ButtonStyle;
use serenity::model::interactions::InteractionResponseType;
use serenity::Error;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::wrappers::opentrivia::api::ApiCall;
use crate::wrappers::opentrivia::api::TriviaCall;

use crate::wrappers::opentrivia::models::Kind;
use crate::wrappers::opentrivia::models::Trivia;

struct Answer {
    value: String,
    correct: bool,
}

#[command("newtrivia")]
#[description("Play Trivia.")]
pub async fn newtrivia(ctx: &Context, msg: &Message) -> CommandResult {
    let mut scores: HashMap<u64, i32> = HashMap::new();

    let call = TriviaCall::new();
    let response = call.call().await?;

    let mut i: usize = 0;
    let total_rounds = response.results.len();
    for trivia in response.results.iter() {
        let answers = get_randomized_answers(&trivia);
        let menu = answer_menu(&ctx, &msg, &trivia, &answers, i + 1, total_rounds).await?;
        let user_answers = {
            let user_answers = collect_user_answers(&ctx, &menu, &trivia.kind).await;

            let mut flattened: HashMap<usize, Vec<u64>> = HashMap::new();
            for (user_id, answer_index) in user_answers.iter() {
                let entry = flattened.entry(*answer_index).or_insert(Vec::new());
                entry.push(*user_id);
            }

            flattened
        };

        let mut message = String::new();
        let mut winning_ids: Vec<u64> = Vec::new();
        let mut losing_ids: Vec<u64> = Vec::new();

        let mut j: usize = 0;
        for answer in answers.iter() {
            if answer.correct {
                message.push_str("Correct answer: ");
                message.push_str(&answer.value);
            }
            if let Some(users) = user_answers.get(&j) {
                for user_id in users.iter() {
                    if answer.correct {
                        let entry = scores.entry(*user_id).or_insert(0);
                        *entry += 1;
                        winning_ids.push(*user_id);
                    } else {
                        losing_ids.push(*user_id);
                    }
                }
            }
            j += 1;
        }

        let _ = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    if total_rounds - 1 != i {
                        e.footer(|f| f.text("Next round will start soon."));
                    }
                    e.normal_embed(message);

                    if !winning_ids.is_empty() {
                        e.field(
                            if winning_ids.len() == 1 {
                                "Winner"
                            } else {
                                "Winners"
                            },
                            winning_ids
                                .iter()
                                .map(|id| format!("<@{}>", id))
                                .collect::<Vec<String>>()
                                .join("\n"),
                            false,
                        );
                    }
                    if !losing_ids.is_empty() {
                        e.field(
                            if winning_ids.len() == 1 {
                                "Loser"
                            } else {
                                "Losers"
                            },
                            losing_ids
                                .iter()
                                .map(|id| format!("<@{}>", id))
                                .collect::<Vec<String>>()
                                .join("\n"),
                            false,
                        );
                    }
                    e
                })
            })
            .await;

        i += 1;
    }

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.normal_embed(format!("Game has ended. Total scores:{}", {
                    let mut message = String::new();
                    for (user_id, score) in scores.iter() {
                        message.push_str(&format!("\n<@{}>: {}", user_id, score));
                    }
                    message
                }))
            })
        })
        .await;

    Ok(())
}

fn get_randomized_answers(trivia: &Trivia) -> Vec<Answer> {
    let mut answers: Vec<Answer> = Vec::new();

    match trivia.kind {
        Kind::Multiple => {
            let mut i = 0;
            let correct_index = {
                let mut rng = thread_rng();
                rng.gen_range(0, trivia.incorrect_answers.len())
            };
            for incorrect_answer in trivia.incorrect_answers.iter() {
                if i == correct_index {
                    answers.push(Answer {
                        value: htmlescape::decode_html(&trivia.correct_answer.clone())
                            .unwrap_or(trivia.correct_answer.clone()),
                        correct: true,
                    });
                }
                answers.push(Answer {
                    value: htmlescape::decode_html(&incorrect_answer.clone())
                        .unwrap_or(incorrect_answer.clone()),
                    correct: false,
                });
                i += 1;
            }
        }
        Kind::Boolean => {
            for incorrect_answer in trivia.incorrect_answers.iter() {
                let correct = Answer {
                    value: htmlescape::decode_html(&trivia.correct_answer.clone())
                        .unwrap_or(trivia.correct_answer.clone()),
                    correct: true,
                };
                let incorrect = Answer {
                    value: htmlescape::decode_html(&incorrect_answer.clone())
                        .unwrap_or(incorrect_answer.clone()),
                    correct: false,
                };

                if trivia.correct_answer == "True" {
                    answers.push(correct);
                    answers.push(incorrect);
                } else {
                    answers.push(incorrect);
                    answers.push(correct);
                }
            }
        }
    }

    answers
}

pub async fn collect_user_answers(
    ctx: &Context,
    menu: &Message,
    kind: &Kind,
) -> HashMap<u64, usize> {
    let interactions = &mut menu
        .await_component_interactions(&ctx)
        .collect_limit(10)
        .timeout(std::time::Duration::from_secs(30))
        .await;

    let mut answers: HashMap<u64, usize> = HashMap::new();

    loop {
        match interactions.next().await {
            Some(interaction) => {
                let _ = interaction
                    .create_interaction_response(&ctx, |f| {
                        f.kind(InteractionResponseType::DeferredUpdateMessage)
                    })
                    .await;

                match kind {
                    Kind::Multiple => {
                        if let Some(value) = interaction.data.values.get(0) {
                            if let Ok(index) = value.parse::<usize>() {
                                answers.insert(interaction.user.id.into(), index);
                            }
                        }
                    }
                    Kind::Boolean => {
                        if let Ok(index) = interaction.data.custom_id.parse::<usize>() {
                            answers.insert(interaction.user.id.into(), index);
                        }
                    }
                }
            }
            None => break,
        }
    }

    answers
}

async fn answer_menu(
    ctx: &Context,
    msg: &Message,
    trivia: &Trivia,
    answers: &Vec<Answer>,
    current_round: usize,
    total_rounds: usize,
) -> Result<Message, Error> {
    let interactive_msg = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.normal_embed(
                    &htmlescape::decode_html(&trivia.question).unwrap_or(trivia.question.clone()),
                )
                .title(format!("Round {}/{}", current_round, total_rounds))
                .footer(|f| f.text(format!("Category: {}", &trivia.category)))
            })
            .components(|c| {
                c.create_action_row(|f| match trivia.kind {
                    Kind::Multiple => f.create_select_menu(|s| {
                        s.min_values(1)
                            .max_values(1)
                            .placeholder("--Select an answer--")
                            .custom_id("answer_index")
                            .options(|m| {
                                let mut i = 0;
                                for answer in answers.iter() {
                                    m.create_option(|o| {
                                        o.description("")
                                            .label(&answer.value)
                                            .value(i)
                                            .default_selection(false)
                                    });
                                    i += 1;
                                }
                                m
                            })
                    }),
                    Kind::Boolean => {
                        let mut i = 0;
                        for answer in answers.iter() {
                            if answer.value == "True" {
                                f.create_button(|b| {
                                    b.style(ButtonStyle::Success).custom_id(i).label("Yes")
                                });
                            } else {
                                f.create_button(|b| {
                                    b.style(ButtonStyle::Danger).custom_id(i).label("No")
                                });
                            }
                            i += 1;
                        }
                        f
                    }
                })
            })
        })
        .await;

    interactive_msg
}

//     if let Some(guild) = msg.guild(&ctx).await {
//         let mut roles: Vec<&Role> = Vec::new();
//         for id in vec![880159827165335583, 880159850523406357].iter() {
//             if let Some(role) = guild.roles.get(&RoleId { 0: *id }) {
//                 roles.push(role);
//             }
//         }
//         let menu = role_menu(ctx, msg, roles).await?;
//         let interaction = &menu
//             .await_component_interaction(&ctx)
//             .author_id(msg.author.id)
//             .timeout(std::time::Duration::from_secs(60))
//             .await
//             .ok_or("Timed out...")?;

//         let _ = interaction
//             .create_interaction_response(&ctx, |f| {
//                 f.kind(InteractionResponseType::DeferredUpdateMessage)
//             })
//             .await;

//         if interaction.data.custom_id == "role_id" {
//             for value in interaction.data.values.iter() {
//                 if let Ok(role_id) = value.parse::<u64>() {
//                     if let Ok(mut member) = guild.member(&ctx, msg.author.id).await {
//                         let _ = member.add_role(&ctx, role_id).await?;
//                     }
//                 }
//             }
//         };
//     }

//     Ok(())
// }

// pub async fn role_menu(ctx: &Context, msg: &Message, roles: Vec<&Role>) -> Result<Message, Error> {
//     let interactive_msg = msg
//         .channel_id
//         .send_message(&ctx, |m| {
//             m.embed(|e| e.normal_embed("Choose a role."))
//                 .components(|c| {
//                     c.create_action_row(|f| {
//                         f.create_select_menu(|s| {
//                             s.min_values(1)
//                                 .max_values(1)
//                                 .placeholder("--Choose a role--")
//                                 .custom_id("role_id")
//                                 .options(|m| {
//                                     for role in roles.iter() {
//                                         m.create_option(|o| {
//                                             o.description("")
//                                                 .label(&role.name)
//                                                 .value(role.id)
//                                                 .default_selection(false)
//                                         });
//                                     }
//                                     m
//                                 })
//                         })
//                     })
//                 })
//         })
//         .await;

//     interactive_msg
// }
