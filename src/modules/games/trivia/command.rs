use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::guild::Role;
use serenity::model::id::RoleId;

#[command("newtrivia")]
#[description("Play Trivia.")]
pub async fn newtrivia(ctx: &Context, msg: &Message) -> CommandResult {
    if let Some(guild) = msg.guild(&ctx).await {
        let mut roles: Vec<&Role> = Vec::new();
        for id in vec![862447164197961759, 851207185807441991, 862398598724583424].iter() {
            if let Some(role) = guild.roles.get(&RoleId { 0: *id }) {
                roles.push(role);
            }
        }
        _role_menu(ctx, msg, roles).await;
    }

    Ok(())
}

pub async fn _role_menu(ctx: &Context, msg: &Message, roles: Vec<&Role>) {
    let interactive_msg = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| e.description("hi")).components(|c| {
                c.create_action_row(|f| {
                    f.create_select_menu(|s| {
                        s.min_values(1)
                            .max_values(1)
                            .placeholder("--Choose a role--")
                            .custom_id("role_id")
                            .options(|m| {
                                for role in roles.iter() {
                                    m.create_option(|o| {
                                        o.description("Role")
                                            .label(&role.name)
                                            .value(role.id)
                                            .default_selection(false)
                                    });
                                }
                                m
                            })
                    })
                })
            })
        })
        .await;

    if let Err(e) = interactive_msg {
        println!("{:?}", e);
    }
}
