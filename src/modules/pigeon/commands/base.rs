use super::buy::*;
use super::explore::*;
use super::space::*;
use serenity::framework::standard::macros::group;

#[group]
#[prefix("pigeon")]
#[commands(buy, explore, space)]
pub struct Pigeon;
