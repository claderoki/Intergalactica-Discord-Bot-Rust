use std::time::Duration;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

use serenity::model::channel::ReactionType;
use serenity::model::interactions::ButtonStyle;
use serenity::model::interactions::InteractionData;
use serenity::model::interactions::InteractionResponseType;
use serenity::model::prelude::User;

extern crate rand;
use rand::thread_rng;
use rand::Rng;

use crate::discord_helpers::embed_utils::EmbedExtension;

const ROW_LENGTH: usize = 7;
const COLUMN_LENGTH: usize = 7;
const CENTER: usize = 3;

pub struct Grid {
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new() -> Self {
        let mut cells: Vec<Cell> = Vec::new();
        for i in 0..ROW_LENGTH*COLUMN_LENGTH {
            cells.push(Cell::new());
        }

        Self {
            cells: cells,
        }
    }
}

pub enum ShapeType {
    O,
    L,
    I,
    T,
    // S,
    // Z,
    // J,
}

pub enum ShapeDirection {
    Left,
    Right,
    Down,
}

pub struct Shape {
    shape_type: ShapeType,
    direction: ShapeDirection,
    indexes: [i32; 4],
    color: Color,
}

impl Shape {
    pub fn new(shape_type: ShapeType, color: Color) -> Self {
        let center = CENTER as i32;
        let col = COLUMN_LENGTH as i32;

        let indexes = match shape_type {
            ShapeType::O => [
                center,
                (center+1),
                ((center-col)),
                (((center+1)-col)),
            ],

            ShapeType::L => [
                (center),
                (center+(1)),
                ((center-col)),
                ((center-(col*2))),
            ],

            ShapeType::I => [
                (center),
                ((center-col)),
                ((center-(col*2))),
                ((center-(col*3))),
            ],

            ShapeType::T => [
                (center),
                ((center-col)),
                (((center-(1))-col)),
                (((center+(1))-col)),
            ],

            // ShapeType::S => [CENTER, ],
            // ShapeType::Z => [CENTER, ],
            // ShapeType::J => [CENTER, ],
        };

        Self {
            direction: ShapeDirection::Down,
            indexes: indexes,
            shape_type: shape_type,
            color,
        }
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        Shape::new(match rng.gen_range(0, 5) {
            0 => ShapeType::L,
            1 => ShapeType::T,
            2 => ShapeType::I,
            _ => ShapeType::O,
        },
        match rng.gen_range(0, 5) {
            0 => Color::Purple,
            1 => Color::Green,
            2 => Color::Red,
            3 => Color::Yellow,
            4 => Color::Orange,
            _ => Color::Blue,

        })
    }

    pub fn tick(&mut self, direction: &ShapeDirection) {
        let append_value = match direction {
            ShapeDirection::Down => COLUMN_LENGTH as i32,
            ShapeDirection::Right => 1,
            ShapeDirection::Left => -1,
        };

        for index in self.indexes.iter_mut() {
            *index += append_value;
        }
    }

    pub fn rotate(&mut self) {

    }

    pub fn solidify(&mut self, grid: &mut Grid) {
        for index in self.indexes {
            if index > 0 {
                if let Some(mut cell) = grid.cells.get_mut(index as usize) {
                    cell.color = match self.color {
                        Color::Purple => Color::Purple,
                        Color::Green => Color::Green,
                        Color::Red => Color::Red,
                        Color::Yellow => Color::Yellow,
                        Color::Orange => Color::Orange,
                        Color::Blue => Color::Blue,
                        _ => Color::Blue
                    }
                }
            }

        }
    }
}

pub enum Color {
    Blank,
    Purple,
    Green,
    Red,
    Blue,
    Orange,
    Yellow,
}

impl Color {
    pub fn get_unicode(&self) -> String {
        String::from(match self {
            Color::Blank => "⬛",
            Color::Purple => "🟪",
            Color::Green => "🟩",
            Color::Red => "🟧",
            Color::Yellow => "🟨",
            Color::Orange => "🟥",
            Color::Blue => "🟦",
        })
    }
}

pub struct Cell {
    pub color: Color,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            color: Color::Blank,
        }
    }
}

pub struct Game {
    pub grid: Grid,
    pub shape: Option<Shape>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(),
            shape: None,
        }
    }

    pub fn set_shape(&mut self, value: Shape) -> &mut Self {
        self.shape = Some(value);
        self
    }

    fn allowed_to_move(&self, direction: &ShapeDirection) -> bool {
        if let Some(shape) = &self.shape {
            let col = COLUMN_LENGTH as i32;
            for index in shape.indexes.iter() {
                let new_index = col + index;
                if new_index > 0 {
                    match self.grid.cells.get(new_index as usize) {
                        Some(cell) => {
                            match cell.color {
                                Color::Blank => {
                                    return true;
                                },
                                _ => {
                                    return false;
                                }
                            }
                            return true;
                        }, None => {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn tick(&mut self, direction: ShapeDirection) {
        let allowed = self.allowed_to_move(&direction);

        if let Some(ref mut shape) = self.shape {
            if allowed {
                shape.tick(&direction);
            } else {
                shape.solidify(&mut self.grid);
                self.shape = None;
            }
        }
    }
}

struct UI;

impl UI {
    pub fn new() -> Self {
        Self {}
    }

    fn get_message_contents(&self, game: &Game) -> String {
        let mut i = 0;
        let mut contents = String::from("");

        let shape_indexes: Option<[i32; 4]>;
        if let Some(shape) = &game.shape {
            shape_indexes = Some(shape.indexes);
        } else {
            shape_indexes = None;
        }

        for cell in game.grid.cells.iter() {
            if i != 0 && i % COLUMN_LENGTH == 0 {
                contents.push_str("\n");
            }

            if let Some(indexes) = shape_indexes {
                if indexes.contains(&(i as i32)) {
                    contents.push_str(&game.shape.as_ref().unwrap().color.get_unicode());
                } else {
                    contents.push_str(&cell.color.get_unicode());
                }
            } else {
                contents.push_str(&cell.color.get_unicode());
            }

            i += 1;
        }

        contents
    }

    pub async fn render_first(
        &mut self,
        ctx: &Context,
        msg: &Message,
        game: &Game,
    ) -> Result<Message, String> {
        let content = self.get_message_contents(game);

        match msg
            .channel_id
            .send_message(&ctx, |m|{
                    m.components(|c|
                    c.create_action_row(|a|
                    {
                        a.create_button(|b|
                            b.style(ButtonStyle::Secondary)
                            .custom_id("placeholder1")
                            .disabled(true)
                            .label(" ")
                        )
                        .create_button(|b|
                            b.style(ButtonStyle::Secondary)
                            .custom_id("arrow_up")
                            .emoji(ReactionType::Unicode("⬆️".into()))
                            .label(" ")
                        )
                        .create_button(|b|
                            b.style(ButtonStyle::Secondary)
                            .custom_id("placeholder2")
                            .disabled(true)
                            .label(" ")
                        )
                    })
                .create_action_row(|a|
                {
                    a.create_button(|b|
                        b.style(ButtonStyle::Secondary)
                        .custom_id("arrow_left")
                        .emoji(ReactionType::Unicode("⬅️".into()))
                        .label(" ")
                    )
                    .create_button(|b|
                        b.style(ButtonStyle::Secondary)
                        .custom_id("placeholder3")
                        .disabled(true)
                        .label(" ")
                    )
                    .create_button(|b|
                        b.style(ButtonStyle::Secondary)
                        .custom_id("arrow_right")
                        .emoji(ReactionType::Unicode("➡️".into()))
                        .label(" ")
                    )
                })
                .create_action_row(|a|
                    {
                        a.create_button(|b|
                            b.style(ButtonStyle::Secondary)
                            .custom_id("placeholder4")
                            .disabled(true)
                            .label(" ")
                        )
                        .create_button(|b|
                            b.style(ButtonStyle::Secondary)
                            .custom_id("arrow_down")
                            .emoji(ReactionType::Unicode("⬇️".into()))
                            .label(" ")
                        )
                        .create_button(|b|
                            b.style(ButtonStyle::Secondary)
                            .custom_id("placeholder5")
                            .disabled(true)
                            .label(" ")
                        )
                    })
                )
                .embed(|e| e.normal_embed(content))
            })
            .await
        {
            Ok(message) => Ok(message),
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    pub async fn render(
        &mut self,
        ctx: &Context,
        msg: &mut Message,
        game: &Game,
    ) -> Result<(), String> {
        let content = self.get_message_contents(game);

        match msg
            .edit(&ctx, |m| m.embed(|e| e.normal_embed(content)))
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}

async fn get_player_input(ctx: &Context, message: &Message, user: &User) -> ShapeDirection {
    let interaction_result = &message
    .await_component_interaction(&ctx)
    .author_id(user.id)
    .timeout(Duration::from_millis(1500))
    .await;

    match interaction_result {
        Some(interaction) => {
            let _ = interaction
                .create_interaction_response(&ctx, |f| {
                    f.kind(InteractionResponseType::DeferredUpdateMessage)
                })
                .await;
                if let Some(data) = interaction.data.as_ref() {
                    if let InteractionData::MessageComponent(value) = data {
                        return match value.custom_id.as_str() {
                            "arrow_left" => ShapeDirection::Left,
                            "arrow_up" => ShapeDirection::Down,
                            "arrow_down" => ShapeDirection::Down,
                            "arrow_right" => ShapeDirection::Right,
                            _ => ShapeDirection::Down,
                        }
                    }
                }
            ShapeDirection::Down
        },
        None => ShapeDirection::Down,
    }

}

#[command("tetris")]
#[description("Play Tetris.")]
pub async fn tetris(ctx: &Context, msg: &Message) -> CommandResult {
    let mut game = Game::new();
    let mut ui = UI::new();
    let mut message = ui.render_first(ctx, msg, &game).await?;

    for i in 0..5 {
        for i in 0..ROW_LENGTH {
            if game.shape.is_none() {
                game.set_shape(Shape::random());
            }
            let direction = get_player_input(ctx, &message, &msg.author).await;
            // tokio::time::sleep(Duration::from_millis(1000)).await;
            game.tick(direction);
            ui.render(ctx, &mut message, &game).await?;
        }
    }

    Ok(())
}
