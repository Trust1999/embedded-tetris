use std::time::{Duration, Instant};

use crate::display::render::wrap_x;
use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
pub mod piece;
use piece::{Piece, Rotation};

pub enum GameState {
    StartMenu(InStartState),
    InGame(InGameState),
    GameOver(u32),
}

pub struct InStartState {
    pub phase: StartMenuPhase,
    pub last_update: Instant,
}

#[derive(Debug)]
pub enum StartMenuPhase {
    Text,
    ButtonStart,
    ButtonPressed,
    ButtonReleased,
}

pub struct InGameState {
    pub(crate) blocks: Blocks,
    score: u32,
    pub(crate) current_piece: Piece,
    pub(crate) next_piece: Option<Piece>,
    time_last_move: Instant,
}

impl GameState {
    pub fn update(
        self,
        button_actions: Option<ButtonAction>,
        now: Instant,
        add_score: impl FnMut(u32),
    ) -> Self {
        match self {
            GameState::StartMenu(state) => {
                if button_actions.is_none() {
                    GameState::StartMenu(state)
                } else {
                    GameState::InGame(InGameState::new())
                }
            }
            GameState::InGame(state) => state.update(button_actions, now, add_score),
            GameState::GameOver(score) => {
                if button_actions.is_none() {
                    GameState::GameOver(score)
                } else {
                    GameState::StartMenu(InStartState {
                        phase: StartMenuPhase::ButtonReleased,
                        last_update: Instant::now() - Duration::from_millis(1000),
                    })
                }
            }
        }
    }
}

impl InGameState {
    pub fn new() -> Self {
        Self {
            blocks: Blocks {
                data: [0; DISPLAY_HEIGHT as usize],
            },
            score: 0,
            current_piece: Piece::random(),
            next_piece: None,
            time_last_move: Instant::now(),
        }
    }

    fn update(
        mut self,
        button_action: Option<ButtonAction>,
        now: Instant,
        mut add_score: impl FnMut(u32),
    ) -> GameState {
        let piece_events = button_action
            .map(|button_action| match button_action {
                ButtonAction::MoveLeft => PieceEvent::MoveBy(-1, 0),
                ButtonAction::MoveRight => PieceEvent::MoveBy(1, 0),
                ButtonAction::MoveDown => PieceEvent::Drop,
                ButtonAction::Rotate => PieceEvent::Rotate(Rotation::Deg90),
            })
            .into_iter()
            .chain({
                let should_move =
                    (now.duration_since(self.time_last_move)) >= Duration::from_millis(500);
                should_move.then(|| {
                    self.time_last_move = now;
                    PieceEvent::MoveBy(0, 1)
                })
            });

        for piece_event in piece_events {
            if self.update_piece_and_blocks(piece_event) {
                add_score(self.score);
                return GameState::GameOver(self.score);
            }
        }

        GameState::InGame(self)
    }

    /// Returns whether the game is over
    fn update_piece_and_blocks(&mut self, piece_event: PieceEvent) -> bool {
        let mut collision_piece = self.current_piece.clone();
        match piece_event {
            PieceEvent::Drop => {
                while !self.blocks.intersects(&collision_piece) {
                    collision_piece.move_by(0, 1);
                }
                collision_piece.move_by(0, -1);
            }
            PieceEvent::MoveBy(dx, dy) => collision_piece.move_by(dx, dy),
            PieceEvent::Rotate(rotation) => collision_piece.rotate(rotation),
        }

        // Collissions with floor, walls and existing blocks
        let will_intersect = self.blocks.intersects(&collision_piece);
        if will_intersect {
            // Place piece on top of existing blocks
            self.blocks.place_piece(&self.current_piece);

            // Remove full rows of blocks
            self.score += self.blocks.remove_full_rows() * 10;
            log::info!("Current highscore {}", self.score);

            // Check if game is over
            let game_over = self.blocks.data[7] != 0x00;
            if game_over {
                return true;
            }

            self.current_piece = self.next_piece.take().unwrap_or(Piece::random());
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

pub struct Blocks {
    data: [u8; DISPLAY_HEIGHT as usize],
}

impl Blocks {
    pub(crate) fn get(&self, x: i16, y: i16) -> bool {
        if x < 0 || x >= DISPLAY_WIDTH as i16 {
            return false;
        }
        if y < 0 || y >= DISPLAY_HEIGHT as i16 {
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

    fn intersects(&self, piece: &Piece) -> bool {
        piece
            .block_positions()
            .any(|(x, y)| self.get(wrap_x(x) as i16, y as i16))
    }

    fn place_piece(&mut self, piece: &Piece) {
        for (x, y) in piece.block_positions() {
            self.set(wrap_x(x) as i16, y as i16);
        }
    }

    /// Returns how many rows were removed
    fn remove_full_rows(&mut self) -> u32 {
        let mut removed = 0;
        let height = self.data.len();

        // Start from the bottom row and work upward
        let mut y = height as isize - 1;
        while y >= 0 {
            if self.data[y as usize] == 0xff {
                removed += 1;

                // Shift all rows above down by one
                for row in (1..=y as usize).rev() {
                    self.data[row] = self.data[row - 1];
                }
                self.data[0] = 0x00;

                // Stay on same y index to check the shifted row
            } else {
                y -= 1;
            }
        }

        removed
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
