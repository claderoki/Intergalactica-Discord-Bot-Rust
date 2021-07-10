use super::buy::*;
use super::clean::*;
use super::explore::*;
use super::feed::*;
use super::heal::*;
use super::play::*;
use super::poop::*;
use super::profile::*;
use super::space::*;
use super::train::*;

use serenity::framework::standard::macros::group;

#[group]
#[prefix("pigeon")]
#[commands(buy, explore, space, profile, feed, heal, clean, play, poop, train)]
pub struct Pigeon;
