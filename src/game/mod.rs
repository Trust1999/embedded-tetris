use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH, display::Display, game::piece::Piece, time::Time};

mod piece;

pub enum GameState {
    StartMenu,
    InGame(InGameState),
    GameOverMenu,
}

pub struct InGameState {
    blocks: Blocks,
    current_piece: Piece,
    next_piece: Option<Piece>,
    time_last_move: u64,
}

impl InGameState {
    pub fn new() -> Self {
        Self {
            blocks: Blocks {
                data: [0; DISPLAY_HEIGHT as usize],
            },
            current_piece: Piece::random(),
            next_piece: None,
            time_last_move: 0,
        }
    }
}

impl GameState {
    pub fn update(self, time: &Time) -> Self {
        match self {
            GameState::StartMenu => todo!(),
            GameState::InGame(state) => Self::update_in_game(state, time),
            GameState::GameOverMenu => todo!(),
        }
    }

    fn update_in_game(mut state: InGameState, time: &Time) -> Self {
        if (time.now_ms() - state.time_last_move) < 250 {
            return Self::InGame(state);
        }

        state.time_last_move = time.now_ms();
        state.current_piece.move_by(0, 1);

        if state.next_piece.is_none() && state.current_piece.position_y > 8 {
            state.next_piece = Some(Piece::random());
        }

        // Collissions with floor, walls and existing blocks
        let will_intersect = state.blocks.intersects(&state.current_piece);
        if state.next_piece.is_some() && will_intersect {
            // Place piece ontop of existing blocks
            for (x, y) in state.current_piece.filled_positions() {
                state.blocks.set(x, y);
            }
            state.current_piece = state.next_piece.take().unwrap();
        }

        // Check if game is over
        let blocks_reached_top = !state.blocks.rows().take(8).all(|row| row == 0x00);
        if blocks_reached_top {
            log::info!("Game over");
            return Self::GameOverMenu;
        }

        Self::InGame(state)
    }
}

struct Blocks {
    data: [u8; DISPLAY_HEIGHT as usize],
}

impl Blocks {
    fn intersects(&self, piece: &Piece) -> bool {
        piece.filled_positions().any(|(x, y)| self.get(x, y))
    }

    fn set(&mut self, x: i16, y: i16) {
        if x < 0 || x >= DISPLAY_WIDTH as i16 || y < 0 || y >= DISPLAY_HEIGHT as i16 {
            return;
        }

        let mask = 0b1000_0000 >> x;
        self.data[y as usize] |= mask;
    }

    fn get(&self, x: i16, y: i16) -> bool {
        if x < 0 || x >= DISPLAY_WIDTH as i16 || y < 0 || y >= DISPLAY_HEIGHT as i16 {
            return true;
        }

        let mask = 0b1000_0000 >> x;
        self.data[y as usize] & mask != 0x00
    }

    fn rows(&self) -> impl Iterator<Item = u8> {
        self.data.into_iter()
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
    for x in 0..DISPLAY_WIDTH {
        for y in 0..DISPLAY_HEIGHT {
            display.set_pixel(x, y, state.blocks.get(x as i16, y as i16));
        }
    }

    render_piece(&state.current_piece, display);

    // Divider for next piece
    for i in 0..DISPLAY_WIDTH {
        display.set_pixel(i, 7, true);
    }

    if let Some(next_piece) = &state.next_piece {
        render_piece(next_piece, display);
    }
}

fn render_piece(piece: &Piece, display: &mut impl Display) {
    for (row, blocks) in piece.rows().enumerate() {
        for (col, block) in blocks.iter().enumerate() {
            let x = col as i16 + piece.position_x;
            let y = row as i16 + piece.position_y;

            if *block
                && (0..DISPLAY_WIDTH as i16).contains(&x)
                && (0..DISPLAY_HEIGHT as i16).contains(&y)
            {
                display.set_pixel(x as u8, y as u8, true);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonAction {
    MoveLeft,
    MoveRight,
    MoveDown,
    Rotate,
}
