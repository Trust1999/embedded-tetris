use std::time::{Duration, Instant};

use crate::display::Display;
use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

mod piece;
use piece::{Piece, Rotation};

pub enum GameState {
    StartMenu,
    InGame(InGameState),
    GameOverMenu,
}

pub struct InGameState {
    blocks: Blocks,
    current_piece: Piece,
    next_piece: Option<Piece>,
    time_last_move: Instant,
}

impl GameState {
    pub fn update(self, button_actions: &[ButtonAction], now: Instant) -> Self {
        match self {
            GameState::StartMenu => GameState::StartMenu,
            GameState::InGame(state) => state.update(button_actions, now),
            GameState::GameOverMenu => GameState::GameOverMenu,
        }
    }
}

impl InGameState {
    pub fn new() -> Self {
        Self {
            blocks: Blocks {
                data: [0; DISPLAY_HEIGHT as usize],
            },
            current_piece: Piece::random(),
            next_piece: None,
            time_last_move: Instant::now(),
        }
    }

    fn update(mut self, button_actions: &[ButtonAction], now: Instant) -> GameState {
        let piece_events = button_actions
            .iter()
            .map(|button_action| match button_action {
                ButtonAction::MoveLeft => PieceEvent::MoveBy(-1, 0),
                ButtonAction::MoveRight => PieceEvent::MoveBy(1, 0),
                ButtonAction::MoveDown => PieceEvent::Drop,
                ButtonAction::Rotate => PieceEvent::Rotate(Rotation::Deg90),
            })
            .chain({
                let should_move =
                    (now.duration_since(self.time_last_move)) >= Duration::from_millis(250);
                should_move.then(|| {
                    self.time_last_move = now;
                    PieceEvent::MoveBy(0, 1)
                })
            });

        for piece_event in piece_events {
            if self.update_piece_and_blocks(piece_event) {
                return GameState::GameOverMenu;
            }
        }

        GameState::InGame(self)
    }

    /// Returns whether the game is over
    fn update_piece_and_blocks(&mut self, piece_event: PieceEvent) -> bool {
        let mut collision_piece = self.current_piece.clone();
        match piece_event {
            PieceEvent::Drop => todo!(),
            PieceEvent::MoveBy(dx, dy) => collision_piece.move_by(dx, dy),
            PieceEvent::Rotate(rotation) => collision_piece.rotate(rotation),
        }

        // Collissions with floor, walls and existing blocks
        let will_intersect = self.blocks.intersects(&collision_piece);
        if will_intersect {
            // Place piece on top of existing blocks
            self.blocks.place_piece(&self.current_piece);

            // Remove full rows of blocks
            let points = self.blocks.remove_full_rows() * 100;
            log::info!("Gained {points} points");

            // Check if game is over
            let game_over = self.blocks.data[7] != 0x00;
            if game_over {
                return true;
            }

            self.current_piece = self.next_piece.take().unwrap();
        } else {
            self.current_piece = collision_piece;
        }

        let ((_, min_y), _) = self.current_piece.aabb();
        if self.next_piece.is_none() && min_y > 8 {
            self.next_piece = Some(Piece::random());
        }

        false
    }
}

struct Blocks {
    data: [u8; DISPLAY_HEIGHT as usize],
}

impl Blocks {
    fn get(&self, x: i16, y: i16) -> bool {
        if self.is_invalid_position(x, y) {
            return true;
        }

        let mask = 0b1000_0000 >> x;
        self.data[y as usize] & mask != 0x00
    }

    fn set(&mut self, x: i16, y: i16) {
        if self.is_invalid_position(x, y) {
            return;
        }

        let mask = 0b1000_0000 >> x;
        self.data[y as usize] |= mask;
    }

    fn rows(&self) -> impl Iterator<Item = u8> {
        self.data.into_iter()
    }

    fn intersects(&self, piece: &Piece) -> bool {
        piece
            .block_positions()
            .any(|(x, y)| self.get(x as i16, y as i16))
    }

    fn place_piece(&mut self, piece: &Piece) {
        for (x, y) in piece.block_positions() {
            self.set(x as i16, y as i16);
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

    fn is_invalid_position(&self, x: i16, y: i16) -> bool {
        x < 0 || x >= DISPLAY_WIDTH as i16 || y < 0 || y >= DISPLAY_HEIGHT as i16
    }
}

pub fn render(game_state: &GameState, display: &mut impl Display) {
    match game_state {
        GameState::StartMenu => display.fill(false),
        GameState::InGame(state) => render_in_game(state, display),
        GameState::GameOverMenu => display.fill(true),
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
    for (x, y) in piece.block_positions() {
        if (0..DISPLAY_WIDTH as i16).contains(&x) && (0..DISPLAY_HEIGHT as i16).contains(&y) {
            display.set_pixel(x as u8, y as u8, true);
        }
    }
}

pub enum PieceEvent {
    Drop,
    MoveBy(i16, i16),
    Rotate(Rotation),
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonAction {
    MoveLeft,
    MoveRight,
    MoveDown,
    Rotate,
}
