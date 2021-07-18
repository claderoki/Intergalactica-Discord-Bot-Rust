use chrono::Duration;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use crate::modules::pigeon::helpers::utils::PigeonWinningsBuilder;
use crate::modules::pigeon::helpers::validation::PigeonValidation;
use crate::modules::pigeon::helpers::winning_message::winnings_message;
use crate::modules::pigeon::models::pigeon::PigeonStatus;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;
use crate::modules::shared::caching::flag::FlagCache;
use crate::modules::shared::caching::flag::FlagValidator;
use crate::modules::shared::caching::flag::PigeonLastCleaned;

pub struct GenericFlag<T> where T: ToString {
    pub when: NaiveDateTime,
    pub identifier: T,
}

impl Flag for GenericFlag {
    fn get_key(&self) -> String {
        self.identifier
    }

    fn new(when: NaiveDateTime) -> Self {
        Self {
            when,
            identifier
        }
    }
}

enum BucketType {
    Guild(u64),
    Member(u64, u64),
    User(u64),
}

struct Bucket {
    pub identifier: String,
    pub bucket_type: BucketType,
    pub cooldown: std::time::Duration,
}

impl Bucket {
    pub fn new(identifier: &'static str, bucket_type: BucketType, cooldown: std::time::Duration) -> Self {
        Self {
            identifier: identifier.into()
            bucket_type,
            cooldown
        }
    }

    pub fn validate() -> Result<(), String> {
        Ok(())
    }

    pub fn spend() {

    }

}

#[command("clean")]
#[description("Clean your pigeon.")]
pub async fn clean(ctx: &Context, msg: &Message) -> CommandResult {
    let cost = 15;
    let increase = 25;

    let human_id = PigeonValidation::new()
        .needs_active_pigeon(true)
        .gold_needed(cost)
        .required_pigeon_status(PigeonStatus::Idle)
        .validate(&msg.author)?;

    let stat = PigeonRepository::get_stat_value(human_id, "cleanliness")?;
    if stat.value >= 100 {
        return Err("You already have max cleanliness!".into());
    }

    let bucket = Bucket::new("pigeon_clean", BucketType::User(msg.author.id.into()), std::time::Duration::from_mins(45));
    // let now = FlagValidator::validate::<PigeonLastCleaned>(human_id, Duration::minutes(45))?;
    bucket.validate()?;

    let winnings = PigeonWinningsBuilder::new()
        .cleanliness(increase)
        .gold(-cost)
        .build();

    winnings_message(
        ctx,
        msg,
        &winnings,
        "Your pigeon leaves dirty food prints on the floor! You decide to give it a bath.".into(),
    )
    .await?;
    PigeonRepository::update_winnings(human_id, &winnings)?;

    // FlagCache::add::<PigeonLastCleaned>(human_id, now);
    bucket.spend();
    Ok(())
}
