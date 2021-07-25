use super::tictactoe::command::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(tictactoe)]
struct Games;
