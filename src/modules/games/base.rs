use super::tictactoe::command::*;
use super::trivia::command::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(tictactoe, trivia)]
struct Games;
