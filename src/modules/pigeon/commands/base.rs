use super::buy::*;
use super::space_explore::*;
use serenity::framework::standard::macros::group;

#[group]
#[prefix("pigeon")]
#[commands(buy, space_explore)]
pub struct Pigeon;
