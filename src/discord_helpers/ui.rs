use serenity::client::Context;
use serenity::model::channel::Message;

use crate::modules::pigeon::helpers::utils::winning_to_string;
use crate::modules::shared::helpers::chooser::confirm;

pub struct GoldConfirmation {
    threshhold: i32,
}

// struct UI;
// impl UI {
//     pub fn confirm() {

//     }
// }

impl GoldConfirmation {
    pub fn new() -> Self {
        Self { threshhold: 0 }
    }

    // pub fn threshhold(&mut self, value: i32) -> &mut Self {
    //     self.threshhold = value;
    //     self
    // }

    pub async fn confirm(
        &self,
        ctx: &Context,
        msg: &Message,
        cost: i32,
    ) -> Result<bool, &'static str> {
        if cost <= self.threshhold {
            Ok(true)
        } else {
            confirm(
                ctx,
                msg,
                format!(
                    "This action will cost {}. Are you sure you want to continue?",
                    winning_to_string(cost, "gold", false)
                ),
            )
            .await
        }
    }
}
