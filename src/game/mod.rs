mod draw;
pub use draw::{MatrixDisplay, Point2d};

mod piece;

pub struct TetrisGame {
    display: MatrixDisplay,
}

impl TetrisGame {
    pub fn new() -> Self {
        Self {
            display: MatrixDisplay::new(),
        }
    }

    pub fn step(&mut self, i: usize) -> &MatrixDisplay {
        if i % MatrixDisplay::HEIGHT as usize == 0 {
            self.display.reset();
        }

        let pos = Point2d::new(
            (i % MatrixDisplay::WIDTH as usize) as u8,
            (i % MatrixDisplay::HEIGHT as usize) as u8,
        );
        self.display.set_pixel(pos, true);

        &self.display
    }
}
