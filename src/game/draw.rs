use std::fmt::Write;

pub struct MatrixDisplay {
    pub data: [u8; Self::HEIGHT as usize],
}

impl MatrixDisplay {
    pub const HEIGHT: u8 = 8 * 4;
    pub const WIDTH: u8 = 8;

    pub fn new() -> Self {
        Self {
            data: [0; Self::HEIGHT as usize],
        }
    }

    /// Resets all pixels to 0
    pub fn reset(&mut self) {
        self.data = [0; Self::HEIGHT as usize]
    }

    pub fn set_pixel(&mut self, pos: Point2d, value: bool) {
        assert!(Self::has_position(pos));

        let line = &mut self.data[pos.y as usize];
        let mask = 0b1000_0000 >> pos.x;

        if value {
            *line |= mask;
        } else {
            *line &= !mask;
        }
    }

    pub fn get_pixel(&self, pos: Point2d) -> bool {
        assert!(Self::has_position(pos));

        let line = self.data[pos.y as usize];
        let mask = 0b1000_0000 >> pos.x;
        return line & mask == mask;
    }

    fn has_position(pos: Point2d) -> bool {
        pos.x < Self::WIDTH && pos.y < Self::HEIGHT
    }
}

impl std::fmt::Display for MatrixDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..(Self::HEIGHT as u8) {
            for x in 0..(Self::WIDTH as u8) {
                let value = self.get_pixel(Point2d::new(x, y));
                f.write_char(if value { '●' } else { '◌' })?;
                f.write_char(' ')?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point2d {
    pub x: u8,
    pub y: u8,
}

impl Point2d {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}
