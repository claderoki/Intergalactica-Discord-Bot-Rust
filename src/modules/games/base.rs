use serenity::framework::standard::macros::group;
use super::tictactoe::command::*;

#[group]
#[commands(tictactoe)]
struct Games;
