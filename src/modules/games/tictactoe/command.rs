use serenity::{client::Context, framework::standard::{CommandResult, macros::command}, model::channel::Message};

use super::game::{Game, run_game};

#[command("ttt")]
#[description("Play TicTacToe.")]
pub async fn tictactoe(ctx: &Context, msg: &Message) -> CommandResult {
    let mut game = Game::new(120566758091259906, 841255759978954763);
    run_game(ctx, msg, &mut game).await?;

    Ok(())
}
