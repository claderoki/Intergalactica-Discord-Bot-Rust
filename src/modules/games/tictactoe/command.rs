use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use super::game::{run_game, Game};

#[command("ttt")]
#[description("Play TicTacToe.")]
pub async fn tictactoe(ctx: &Context, msg: &Message) -> CommandResult {
    let mut game = Game::new(120566758091259906, 841255759978954763);
    run_game(ctx, msg, &mut game).await?;

    Ok(())
}
