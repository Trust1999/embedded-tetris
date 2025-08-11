use crate::{display::Display, time::Time, DISPLAY_HEIGHT};

mod piece;

pub enum GameState {
    StartMenu,
    InGame(InGameState),
    GameOverMenu,
}

pub struct InGameState {
    blocks: [u8; DISPLAY_HEIGHT],
}

impl InGameState {
    pub fn new() -> Self {
        Self {
            blocks: [0; DISPLAY_HEIGHT],
        }
    }
}

impl GameState {
    pub fn update(self, time: &Time) -> Self {
        match self {
            GameState::StartMenu => todo!(),
            GameState::InGame(state) => Self::update_in_game(state),
            GameState::GameOverMenu => todo!(),
        }
    }

    fn update_in_game(state: InGameState) -> Self {
        Self::InGame(state)
    }
}

pub fn render(game_state: &GameState, display: &mut impl Display) {
    match game_state {
        GameState::StartMenu => todo!(),
        GameState::InGame(state) => render_in_game(state, display),
        GameState::GameOverMenu => todo!(),
    }
}

fn render_in_game(state: &InGameState, display: &mut impl Display) {
    display.set_bitmap(&state.blocks);
    // TODO render piece
    // TODE render next piece
}
