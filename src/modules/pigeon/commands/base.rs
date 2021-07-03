use super::buy::*;
use super::explore::*;
use super::space::*;
use super::profile::*;

use serenity::framework::standard::macros::group;

#[group]
#[prefix("pigeon")]
#[commands(buy, explore, space, profile)]
pub struct Pigeon;
