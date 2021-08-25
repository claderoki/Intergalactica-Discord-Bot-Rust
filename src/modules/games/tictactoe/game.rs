use std::convert::TryInto;
use std::time::Duration;

use serenity::builder::CreateButton;
use serenity::builder::CreateComponents;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::interactions::message_component::ButtonStyle;

pub struct Player {
    pub number: i32,
    pub symbol: String,
    pub user_id: u64,
}
impl Player {
    fn new(number: i32, symbol: &'static str, user_id: u64) -> Self {
        Self {
            number,
            symbol: symbol.into(),
            user_id,
        }
    }

    pub fn first(user_id: u64) -> Self {
        Self::new(0, "🇽", user_id)
    }

    pub fn second(user_id: u64) -> Self {
        Self::new(1, "🇴", user_id)
    }
}

pub struct Grid {
    pub cells: [Cell; 9],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
                Cell::new(),
            ],
        }
    }

    pub fn check_cells(
        &self,
        index1: usize,
        index2: usize,
        index3: usize,
    ) -> Option<(String, [usize; 3])> {
        let cell1 = &self.cells[index1];
        let cell2 = &self.cells[index2];
        let cell3 = &self.cells[index3];

        if cell1.symbol.is_none() || cell2.symbol.is_none() || cell3.symbol.is_none() {
            return None;
        }

        let symbol1 = &cell1.symbol.as_ref().unwrap();
        let symbol2 = &cell2.symbol.as_ref().unwrap();
        let symbol3 = &cell3.symbol.as_ref().unwrap();

        if symbol1 == symbol2 && symbol2 == symbol3 {
            return Some((symbol1.to_string(), [index1, index2, index3]));
        }

        None
    }
}
pub struct Cell {
    pub symbol: Option<String>,
}

impl Cell {
    pub fn new() -> Self {
        Self { symbol: None }
    }
}

pub struct Game {
    pub players: [Player; 2],
    pub grid: Grid,
    pub winner_user_id: Option<u64>,
    pub over: bool,
}

impl Game {
    pub fn new(user_id1: u64, user_id2: u64) -> Self {
        Self {
            players: [Player::first(user_id1), Player::second(user_id2)],
            grid: Grid::new(),
            winner_user_id: None,
            over: false,
        }
    }

    pub fn get_player_user_id(&self, symbol: String) -> Result<u64, &'static str> {
        for player in self.players.iter() {
            if player.symbol.eq(&symbol) {
                return Ok(player.user_id);
            }
        }
        Err("Could not find player.")
    }

    pub fn get_winning_data(&self) -> Option<(String, [usize; 3])> {
        for i in 0..3 {
            for j in 0..2 {
                let checked: Option<(String, [usize; 3])>;

                if j == 0 {
                    checked = self.grid.check_cells(3 * i, (3 * i) + 1, (3 * i) + 2);
                } else {
                    checked = self.grid.check_cells(i, 3 + i, (3 + i) + ((3 + i) - i));
                }

                if checked.is_some() {
                    return checked;
                }
            }
        }

        let checked = self.grid.check_cells(0, 4, 8);
        if checked.is_some() {
            return checked;
        }

        self.grid.check_cells(2, 4, 6)
    }
}

trait CreateTicTacToeGrid {
    fn create_grid(
        &mut self,
        grid: &Grid,
        over: bool,
        winning_indexes: Option<[usize; 3]>,
    ) -> &mut Self;
}
impl CreateTicTacToeGrid for CreateComponents {
    fn create_grid(
        &mut self,
        grid: &Grid,
        over: bool,
        winning_indexes: Option<[usize; 3]>,
    ) -> &mut Self {
        self.create_action_row(|f| {
            for i in 0..3 {
                let cell = &grid.cells[i];

                f.create_button(|b| {
                    b.create_cell(
                        i.try_into().unwrap(),
                        cell,
                        over,
                        winning_indexes.map_or(false, |indexes| indexes.contains(&i)),
                    )
                });
            }
            f
        })
        .create_action_row(|f| {
            for i in 3..6 {
                let cell = &grid.cells[i];

                f.create_button(|b| {
                    b.create_cell(
                        i.try_into().unwrap(),
                        cell,
                        over,
                        winning_indexes.map_or(false, |indexes| indexes.contains(&i)),
                    )
                });
            }
            f
        })
        .create_action_row(|f| {
            for i in 6..9 {
                let cell = &grid.cells[i];

                f.create_button(|b| {
                    b.create_cell(
                        i.try_into().unwrap(),
                        cell,
                        over,
                        winning_indexes.map_or(false, |indexes| indexes.contains(&i)),
                    )
                });
            }
            f
        })
    }
}

trait CreateTicTacToeCell {
    fn create_cell(&mut self, index: i32, cell: &Cell, over: bool, winning_cell: bool)
        -> &mut Self;
}
impl CreateTicTacToeCell for CreateButton {
    fn create_cell(
        &mut self,
        index: i32,
        cell: &Cell,
        over: bool,
        winning_cell: bool,
    ) -> &mut Self {
        self.custom_id(index)
            .disabled(cell.symbol.is_some() || over)
            .style(if winning_cell {
                ButtonStyle::Success
            } else {
                ButtonStyle::Secondary
            })
            .label(" ");
        if cell.symbol.is_some() {
            self.emoji(ReactionType::Unicode(
                cell.symbol.as_ref().unwrap().to_string(),
            ));
        }
        self
    }
}

async fn show_grid(ctx: &Context, msg: &Message, game: &Game) -> Result<Message, &'static str> {
    let interactive_msg = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                let p1 = &game.players[0];
                let p2 = &game.players[1];
                e.description(format!(
                    "<@{}>\n<@{}>\n\n<@{}>s turn",
                    p1.user_id, p2.user_id, p1.user_id
                ))
            })
            .components(|c| c.create_grid(&game.grid, false, None))
        })
        .await
        .map_err(|_| "Can't send for some reason")?;
    Ok(interactive_msg)
}

async fn edit_grid(
    ctx: &Context,
    interactive_msg: &mut Message,
    game: &Game,
    user_id: u64,
    won: bool,
    winning_indexes: Option<[usize; 3]>,
) -> Result<(), &'static str> {
    let _ = interactive_msg
        .edit(&ctx, |m| {
            m.embed(|e| {
                if won {
                    e.description(format!("<@{}> won!", user_id));
                } else {
                    e.description(format!("<@{}>s turn", user_id));
                }
                e
            })
            .components(|c| c.create_grid(&game.grid, won, winning_indexes))
        })
        .await
        .map_err(|_| "Can't send for some reason")?;
    Ok(())
}

pub async fn wait_for_response(
    ctx: &Context,
    user_id: u64,
    interactive_msg: &Message,
) -> Result<usize, &'static str> {
    let interaction = &interactive_msg
        .await_component_interaction(&ctx)
        .author_id(user_id)
        .timeout(Duration::from_secs(60))
        .await
        .ok_or("Timed out")?;

    let _ = interaction
        .create_interaction_response(&ctx, |f| {
            f.kind(InteractionResponseType::DeferredUpdateMessage)
        })
        .await;

    match interaction.data.custom_id.parse::<usize>() {
        Ok(index) => Ok(index),
        Err(_) => Err("Can't convert to int"),
    }
}

pub async fn run_game(ctx: &Context, msg: &Message, game: &mut Game) -> Result<(), &'static str> {
    let mut interactive_msg = show_grid(ctx, msg, &game).await?;
    for _ in 0..9 {
        for player in &mut game.players.iter() {
            edit_grid(
                &ctx,
                &mut interactive_msg,
                &game,
                player.user_id,
                false,
                None,
            )
            .await?;
            let index = wait_for_response(ctx, player.user_id, &interactive_msg).await?;
            let mut cell = &mut game.grid.cells[index];
            cell.symbol = Some(player.symbol.to_string());
            let winning_data = game.get_winning_data();
            match winning_data {
                Some(data) => {
                    let winner_user_id = game.get_player_user_id(data.0)?;
                    edit_grid(
                        &ctx,
                        &mut interactive_msg,
                        &game,
                        winner_user_id,
                        true,
                        Some(data.1),
                    )
                    .await?;
                    return Ok(());
                }
                None => {}
            }
        }
    }
    Ok(())
}
