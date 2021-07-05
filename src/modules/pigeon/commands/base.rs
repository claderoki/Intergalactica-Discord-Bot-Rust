use super::buy::*;
use super::explore::*;
use super::feed::*;
use super::heal::*;
use super::profile::*;
use super::space::*;

use serenity::framework::standard::macros::group;

#[group]
#[prefix("pigeon")]
#[commands(buy, explore, space, profile, feed, heal)]
pub struct Pigeon;
