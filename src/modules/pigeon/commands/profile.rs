use serenity::builder::CreateEmbed;
use serenity::builder::CreateEmbedFooter;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::model::prelude::User;

use crate::discord_helpers::embed_utils::EmbedExtension;
use crate::modules::pigeon::helpers::utils::PigeonWinnable;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::models::pigeon::PigeonProfile;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::exploration::ExplorationRepository;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::helpers::utils::TimeDelta;

#[command("profile")]
#[description("View your pigeons profile.")]
#[aliases("status")]
pub async fn profile(ctx: &Context, msg: &Message) -> CommandResult {
    let user: &User = msg.mentions.get(0).map_or(&msg.author, |u| u);

    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .other(!user.eq(&msg.author))
        .validate(&user)?;

    let profile = PigeonRepository::get_profile(human_id)?;

    let _ = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| create_profile_embed(e, human_id, &profile))
        })
        .await?;

    Ok(())
}

trait Fillable {
    fn fill(&self, threshhold: usize) -> Self;
}

impl Fillable for String {
    fn fill(&self, threshhold: usize) -> Self {
        if self.len() > threshhold {
            self.to_string()
        } else {
            let missing = threshhold - self.len();
            format!("{}{}", self, "-".repeat(missing))
        }
    }
}

fn create_profile_embed<'a>(
    embed: &'a mut CreateEmbed,
    human_id: i32,
    profile: &PigeonProfile,
) -> &'a mut CreateEmbed {
    embed
        .title(&profile.name.fill(15))
        .thumbnail(get_embed_thumbnail_url(&profile.status))
        .normal_embed(&profile.to_string())
        .footer(|f| create_status_footer(f, human_id, &profile.status, profile.jail_time_left_in_seconds))
}

fn create_status_footer<'a>(
    footer: &'a mut CreateEmbedFooter,
    human_id: i32,
    status: &PigeonStatus,
    jail_time_left: i64,
) -> &'a mut CreateEmbedFooter {
    match status {
        PigeonStatus::SpaceExploring => {
            let exploration =
                ExplorationRepository::get_exploration(human_id).expect("no exploration");
            let location =
                ExplorationRepository::get_location(exploration.location_id).expect("no location");
            footer.icon_url(location.image_url).text({
                if exploration.arrived {
                    format!("exploring {}", location.planet_name)
                } else {
                    format!("traveling to {}", location.planet_name)
                }
            });
        },
        PigeonStatus::Jailed => {
            let delta = TimeDelta::from_seconds(jail_time_left);
            footer.text(format!("In jail for another {}", delta.to_text()));
        }
        _ => {
            footer.text(status.get_friendly_verb());
        }
    };

    footer
}

fn get_embed_thumbnail_url(status: &PigeonStatus) -> String {
    match status {
        PigeonStatus::Idle           => "https://media.discordapp.net/attachments/744172199770062899/863422058154033162/idle.png",
        PigeonStatus::Mailing        => "",
        PigeonStatus::Exploring      => "https://media.discordapp.net/attachments/744172199770062899/863422074927317052/exploring.png",
        PigeonStatus::Fighting       => "",
        PigeonStatus::Jailed         => "https://cdn.discordapp.com/attachments/744172199770062899/868835590211264542/jailed_pigeon.png",
        PigeonStatus::Dating         => "",
        PigeonStatus::SpaceExploring => "https://media.discordapp.net/attachments/744172199770062899/863421831532511242/space_exploring.png",
    }.into()
}
