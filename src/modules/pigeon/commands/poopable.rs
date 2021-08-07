use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command("poopable")]
#[only_in(guild)]
#[description("Find a poopable pigeon.")]
pub async fn poopable(_ctx: &Context, _msg: &Message) -> CommandResult {
    Err("Use `/pigeon poop` without mentioning someone to poop on a random member.".into())
}
