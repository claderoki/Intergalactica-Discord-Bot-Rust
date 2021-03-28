use super::buy::*;
use serenity::framework::standard::macros::group;

#[group]
#[prefix("pigeon")]
#[commands(buy)]
pub struct Pigeon;
