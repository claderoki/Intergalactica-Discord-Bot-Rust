use super::tictactoe::command::*;
use super::trivia::command::*;


use serenity::framework::standard::macros::group;

#[group]
#[commands(tictactoe, newtrivia)]
struct Games;
