use std::sync::Arc;

use serenity::client::Context;

use crate::modules::pigeon::helpers::utils::PigeonWinningsBuilder;
use crate::modules::pigeon::models::pigeon::DecayingPigeon;
use crate::modules::pigeon::repository::pigeon::PigeonRepository;

pub async fn decay_pigeons(_ctx: Arc<Context>) {
    let pigeons_result = PigeonRepository::get_decaying_pigeons();
    match pigeons_result {
        Ok(pigeons) => {
            for pigeon in pigeons.iter() {
                decay_pigeon(pigeon);
            }
        },
        Err(e) => println!("{:?}", e),
    }
}

fn decay_pigeon(pigeon: &DecayingPigeon) {
    let mut builder = PigeonWinningsBuilder::new();

    builder
        .cleanliness(-1)
        .health(-1)
        .happiness(-1)
        .food(-1)
    ;

    let result = PigeonRepository::update_winnings(pigeon.human_id, &builder.build());
    match result {
        Ok(_) => {},
        Err(e) => {println!("{:?}", e)},
    }
}