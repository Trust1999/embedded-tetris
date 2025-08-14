use std::time::{Duration, Instant};

use crate::display::Display;
use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

mod piece;
use piece::{Piece, Rotation};

pub enum GameState {
    StartMenu(InStartState),
    InGame(InGameState),
    GameOver(u32),
}

pub enum InStartState {
    Text,
    ButtonStart,
    ButtonPressed,
    ButtonReleased,
}

pub struct InGameState {
    blocks: Blocks,
    score: u32,
    current_piece: Piece,
    next_piece: Option<Piece>,
    time_last_move: Instant,
}

impl GameState {
    pub fn update(
        self,
        button_actions: &[ButtonAction],
        now: Instant,
        add_score: impl FnMut(u32),
    ) -> Self {
        match self {
            GameState::StartMenu(state) => {
                if button_actions.is_empty() {
                    let now = Instant::now();
                    let time = now + Duration::from_millis(500);
                    while Instant::now() < time {}
                    GameState::StartMenu(state)
                } else {
                    GameState::InGame(InGameState::new())
                }
            }
            GameState::InGame(state) => state.update(button_actions, now, add_score),
            GameState::GameOver(score) => {
                if button_actions.is_empty() {
                    GameState::GameOver(score)
                } else {
                    GameState::StartMenu(InStartState::Text)
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
        button_actions: &[ButtonAction],
        now: Instant,
        mut add_score: impl FnMut(u32),
    ) -> GameState {
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

struct Blocks {
    data: [u8; DISPLAY_HEIGHT as usize],
}

impl Blocks {
    fn get(&self, x: i16, y: i16) -> bool {
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

pub fn render(game_state: &mut GameState, display: &mut impl Display) {
    match game_state {
        GameState::StartMenu(state) => render_start(state, display),
        GameState::InGame(state) => render_in_game(state, display),
        GameState::GameOver(score) => render_score(*score, display),
    }
}

fn render_start(state: &mut InStartState, display: &mut impl Display) {
    match state {
        InStartState::Text => {
            for i in 0..4 {
                let offset = 8 * i as u8;
                render_letter(i, offset, display);
            }
            *state = InStartState::ButtonStart;
        }
        InStartState::ButtonStart => {
            for i in 0..4 {
                let offset = 8 * i as u8;
                render_button(false, offset, display);
            }
            *state = InStartState::ButtonPressed;
        }
        InStartState::ButtonPressed => {
            for i in 0..4 {
                let offset = 8 * i as u8;
                render_button(true, offset, display);
            }
            *state = InStartState::ButtonReleased;
        }
        InStartState::ButtonReleased => {
            for i in 0..4 {
                let offset = 8 * i as u8;
                render_button(false, offset, display);
            }
            *state = InStartState::Text;
        }
    }
}

fn render_letter(row: i32, offset: u8, display: &mut impl Display) {
    let bitmap = letter_bitmap(row);

    for (y, row) in bitmap.iter().enumerate() {
        for x in 0..8 {
            let mask = 1 << (7 - x); // leftmost pixel is the highest bit
            let pixel_on = (row & mask) != 0;
            if pixel_on {
                display.set_pixel(x, offset + y as u8, true);
            }
        }
    }
}

fn render_button(button: bool, offset: u8, display: &mut impl Display) {
    let bitmap = button_bitmap(button);

    for (y, row) in bitmap.iter().enumerate() {
        for x in 0..8 {
            let mask = 1 << (7 - x); // leftmost pixel is the highest bit
            let pixel_on = (row & mask) != 0;
            if pixel_on {
                display.set_pixel(x, offset + y as u8, true);
            }
        }
    }
}

const fn letter_bitmap(c: i32) -> [u8; 8] {
    match c {
        0 => [
            0b01111110, 0b00011000, 0b00011000, 0b00011000, 0b00000000, 0b01111110, 0b01100000,
            0b01111100,
        ],
        1 => [
            0b01100000, 0b01111110, 0b00000000, 0b01111110, 0b00011000, 0b00011000, 0b00011000,
            0b01111100,
        ],
        2 => [
            0b01101100, 0b01111110, 0b01101100, 0b01100110, 0b00000000, 0b00011000, 0b00011000,
            0b00011000,
        ],
        3 => [
            0b00011000, 0b00000000, 0b01111110, 0b01100000, 0b01111110, 0b00000110, 0b01111110,
            0b00000000,
        ],
        _ => [0; 8],
    }
}

const fn button_bitmap(b: bool) -> [u8; 8] {
    match b {
        true => [
            0b00000000, 0b00000000, 0b00011000, 0b00011000, 0b01111110, 0b01111110, 0b00000000,
            0b00000000,
        ],
        false => [
            0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b01111110, 0b01111110, 0b00000000,
            0b00000000,
        ],
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
        display.set_pixel(wrap_x(x), y as u8, true);
    }
}

fn wrap_x(x: i16) -> u8 {
    (if x < 0 { 8 - x.abs() % 8 } else { x % 8 }) as u8
}

fn render_score(score: u32, display: &mut impl Display) {
    display.fill(false);

    for (i, digit) in [
        score / 1000 % 10,
        score / 100 % 10,
        score / 10 % 10,
        score % 10,
    ]
    .iter()
    .enumerate()
    {
        let offset = 8 * i as u8;
        render_digit(*digit, offset, display);
    }
}

fn render_digit(digit: u32, offset: u8, display: &mut impl Display) {
    let bitmap = digit_bitmap(digit);

    for (y, row) in bitmap.iter().enumerate() {
        for x in 0..8 {
            let mask = 1 << (7 - x); // leftmost pixel is the highest bit
            let pixel_on = (row & mask) != 0;
            if pixel_on {
                display.set_pixel(x, offset + y as u8, true);
            }
        }
    }
}

const fn digit_bitmap(digit: u32) -> [u8; 8] {
    match digit {
        0 => [
            0b00111100, 0b01100110, 0b01101110, 0b01110110, 0b01100110, 0b01100110, 0b00111100,
            0b00000000,
        ],
        1 => [
            0b00011000, 0b00111000, 0b00011000, 0b00011000, 0b00011000, 0b00011000, 0b00111100,
            0b00000000,
        ],
        2 => [
            0b00111100, 0b01100110, 0b00000110, 0b00001100, 0b00011000, 0b01100000, 0b01111110,
            0b00000000,
        ],
        3 => [
            0b00111100, 0b01100110, 0b00000110, 0b00011100, 0b00000110, 0b01100110, 0b00111100,
            0b00000000,
        ],
        4 => [
            0b00001100, 0b00011100, 0b00101100, 0b01001100, 0b01111110, 0b00001100, 0b00001100,
            0b00000000,
        ],
        5 => [
            0b01111110, 0b01100000, 0b01111100, 0b00000110, 0b00000110, 0b01100110, 0b00111100,
            0b00000000,
        ],
        6 => [
            0b00111100, 0b01100110, 0b01100000, 0b01111100, 0b01100110, 0b01100110, 0b00111100,
            0b00000000,
        ],
        7 => [
            0b01111110, 0b01100110, 0b00000110, 0b00001100, 0b00011000, 0b00011000, 0b00011000,
            0b00000000,
        ],
        8 => [
            0b00111100, 0b01100110, 0b01100110, 0b00111100, 0b01100110, 0b01100110, 0b00111100,
            0b00000000,
        ],
        9 => [
            0b00111100, 0b01100110, 0b01100110, 0b00111110, 0b00000110, 0b01100110, 0b00111100,
            0b00000000,
        ],
        _ => unreachable!(),
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
