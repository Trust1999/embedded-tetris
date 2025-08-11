use crate::display::Display;

mod piece;

pub struct TetrisGame {}

impl TetrisGame {
    pub fn new() -> Self {
        Self {}
    }

    pub fn step(&mut self, i: usize, display: &mut impl Display) {
        display.set_pixel((i % 8) as u8, (i / 8) as u8, true);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonAction {
    MoveLeft,
    MoveRight,
    MoveDown,
    Rotate,
}
