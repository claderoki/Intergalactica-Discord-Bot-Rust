use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::modules::pigeon::helpers::utils::winning_to_string;
use crate::modules::pigeon::helpers::utils::PigeonWinningsBuilder;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use rand::thread_rng;
use rand::Rng;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command("rob")]
#[description("Rob other pigeons.")]
pub async fn rob(ctx: &Context, msg: &Message) -> CommandResult {
    let recipient = msg.mentions.get(0).ok_or("No one mentioned")?;
    let initiator = &msg.author;

    let mut validator = PigeonValidation::new();
    validator.needs_active_pigeon(true).needs_pvp_enabled(true);

    let recipient_human_id = validator
        .other(true)
        .needs_available_pvp_action(false)
        .gold_needed(100)
        .validate(recipient)?;

    let initiator_human_id = validator
        .required_pigeon_status(PigeonStatus::Idle)
        .other(false)
        .gold_needed(0)
        .needs_available_pvp_action(true)
        .validate(initiator)?;

    let success = {
        let mut rng = thread_rng();
        rng.gen_range(0..3) != 0
    };

    match success {
        true => {
            success_scenario(ctx, msg, initiator_human_id, recipient_human_id).await?;
            PigeonRepository::set_pvp_action_used(initiator_human_id)?;
        }
        false => {
            PigeonRepository::update_status(initiator_human_id, PigeonStatus::Jailed)?;
            let hours = 3;
            PigeonRepository::set_pvp_action_used(initiator_human_id)?;
            PigeonRepository::jail(initiator_human_id, hours)?;
            return Err(format!(
                "You fail to rob {} and are put in jail for {} hours.",
                recipient, hours
            )
            .into());
        }
    };

    Ok(())
}

async fn success_scenario(
    ctx: &Context,
    msg: &Message,
    initiator_human_id: i32,
    recipient_human_id: i32,
) -> Result<(), String> {
    let gold_stolen = {
        let mut rng = thread_rng();
        rng.gen_range(10..101)
    };

    let initiator_winnings = PigeonWinningsBuilder::new().gold(gold_stolen).build();
    let recipient_winnings = PigeonWinningsBuilder::new().gold(-gold_stolen).build();

    let recipient_name = PigeonRepository::get_name(recipient_human_id)?;

    PigeonRepository::update_winnings(initiator_human_id, &initiator_winnings)?;
    PigeonRepository::update_winnings(recipient_human_id, &recipient_winnings)?;

    let text = format!(
        "Your pigeon successfully steals {} from {}.",
        winning_to_string(gold_stolen, "gold", false),
        recipient_name.value,
    );

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| m.embed(|e| e.normal_embed(&text)))
        .await
        .or(Err("Failed to send success_scenario"));

    Ok(())
}
