use crate::{
    DISPLAY_HEIGHT, DISPLAY_WIDTH,
    display::Display,
    game::piece::{Piece, Rotation},
    time::Time,
};

mod piece;

pub enum GameState {
    StartMenu,
    InGame(InGameState),
    GameOverMenu,
}

pub struct InGameState {
    blocks: Blocks,
    collision_piece: Piece,
    current_piece: Piece,
    next_piece: Option<Piece>,
    time_last_move: u64,
}

impl GameState {
    pub fn update(self, button_actions: &[ButtonAction], time: &Time) -> Self {
        match self {
            GameState::StartMenu => todo!(),
            GameState::InGame(state) => state.update(button_actions, time),
            GameState::GameOverMenu => todo!(),
        }
    }
}

impl InGameState {
    pub fn new() -> Self {
        let current_piece = Piece::random();
        Self {
            blocks: Blocks {
                data: [0; DISPLAY_HEIGHT as usize],
            },
            collision_piece: current_piece.clone(),
            current_piece,
            next_piece: None,
            time_last_move: 0,
        }
    }

    fn update(mut self, button_actions: &[ButtonAction], time: &Time) -> GameState {
        for button_action in button_actions {
            match button_action {
                ButtonAction::MoveLeft => self.collision_piece.move_by(-1, 0),
                ButtonAction::MoveRight => self.collision_piece.move_by(1, 0),
                ButtonAction::MoveDown => todo!(),
                ButtonAction::Rotate => self.collision_piece.rotate(Rotation::Deg90),
            }

            if self.update_blocks() {
                return GameState::GameOverMenu;
            }
        }

        if (time.now_ms() - self.time_last_move) >= 250 {
            self.time_last_move = time.now_ms();
            self.collision_piece.move_by(0, 1);

            if self.update_blocks() {
                return GameState::GameOverMenu;
            }

            if self.next_piece.is_none() && self.current_piece.position_y > 8 {
                self.next_piece = Some(Piece::random());
            }
        }

        GameState::InGame(self)
    }

    /// Returns whether the game is over
    fn update_blocks(&mut self) -> bool {
        // Collissions with floor, walls and existing blocks
        let will_intersect = self.blocks.intersects(&self.collision_piece);
        if will_intersect {
            // Place piece on top of existing blocks
            self.blocks.place_piece(&self.current_piece);

            // Remove full rows of blocks
            let points = self.blocks.remove_full_rows() * 100;
            log::info!("Gained {points} points");

            // Check if game is over
            let game_over = !self.blocks.rows().take(8).all(|row| row == 0x00);
            if game_over {
                return true;
            }

            self.current_piece = self.next_piece.take().unwrap();
            self.collision_piece = self.current_piece.clone();
        } else {
            self.current_piece = self.collision_piece.clone();
        }

        false
    }
}

struct Blocks {
    data: [u8; DISPLAY_HEIGHT as usize],
}

impl Blocks {
    fn get(&self, x: i16, y: i16) -> bool {
        if x < 0 || x >= DISPLAY_WIDTH as i16 || y < 0 || y >= DISPLAY_HEIGHT as i16 {
            return true;
        }

        let mask = 0b1000_0000 >> x;
        self.data[y as usize] & mask != 0x00
    }

    fn set(&mut self, x: i16, y: i16) {
        if x < 0 || x >= DISPLAY_WIDTH as i16 || y < 0 || y >= DISPLAY_HEIGHT as i16 {
            return;
        }

        let mask = 0b1000_0000 >> x;
        self.data[y as usize] |= mask;
    }

    fn rows(&self) -> impl Iterator<Item = u8> {
        self.data.into_iter()
    }

    fn intersects(&self, piece: &Piece) -> bool {
        piece.filled_positions().any(|(x, y)| self.get(x, y))
    }

    fn place_piece(&mut self, piece: &Piece) {
        for (x, y) in piece.filled_positions() {
            self.set(x, y);
        }
    }

    /// Returns how many rows were removed
    fn remove_full_rows(&mut self) -> u32 {
        let mut rows_removed = 0;

        for (y, row) in self.data.into_iter().enumerate() {
            // Skip rows that are not filled
            if row != 0xff {
                continue;
            }

            // Move all blocks above it down by one
            for i in y..0 {
                self.data[i] = self.data[i - 1];
            }

            // Clear first row
            self.data[0] = 0xff;

            rows_removed += 1;
        }

        rows_removed
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
