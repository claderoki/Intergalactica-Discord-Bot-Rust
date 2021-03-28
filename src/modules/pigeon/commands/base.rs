use serenity::framework::standard::macros::group;
use super::buy::*;

#[group]
#[prefix("pigeon")]
#[commands(buy)]
pub struct Pigeon;
