use super::buy::*;
use super::space_explore::*;
use super::space_action::*;
use serenity::framework::standard::macros::group;

#[group]
#[prefix("pigeon")]
#[commands(buy, space_explore, space)]
pub struct Pigeon;
