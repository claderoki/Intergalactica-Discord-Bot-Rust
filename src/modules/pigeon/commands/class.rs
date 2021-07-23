use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::modules::pigeon::helpers::utils::PigeonWinnable;
use crate::modules::pigeon::helpers::utils::PigeonWinnings;
use crate::modules::pigeon::helpers::utils::PigeonWinningsBuilder;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;

enum PigeonClass {
    Vampire,  // /pigeon feed is disabled, you have to feed off of other pigeons.
    Farmer,   // maybe disable for a while
    Explorer, // faster exploration ?
    Doctor,   // heal other pigeons?
}
// Classes should have levels from 1-5.

#[command("class")]
#[description("Set your class.")]
pub async fn class(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}
