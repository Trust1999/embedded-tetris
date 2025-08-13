use std::fmt::Write;

use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub struct TextDisplay {
    pub data: [u8; DISPLAY_HEIGHT as usize],
}

impl TextDisplay {
    pub fn new() -> Self {
        Self {
            data: [0; DISPLAY_HEIGHT as usize],
        }
    }

    /// Resets all pixels to 0
    pub fn reset(&mut self) {
        self.data = [0; DISPLAY_HEIGHT as usize]
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        assert!(Self::has_position(x, y));

        let line = self.data[y as usize];
        let mask = 0b1000_0000 >> x;
        return line & mask == mask;
    }

    fn has_position(x: u8, y: u8) -> bool {
        x < DISPLAY_WIDTH && y < DISPLAY_HEIGHT
    }
}

impl std::fmt::Display for TextDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let value = self.get_pixel(x, y);
                f.write_char(if value { '●' } else { '◌' })?;
                f.write_char(' ')?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl super::Display for TextDisplay {
    fn fill(&mut self, value: bool) {
        self.data.fill(if value { 0xff } else { 0x00 });
    }

    fn set_pixel(&mut self, x: u8, y: u8, value: bool) {
        assert!(Self::has_position(x, y));

        let line = &mut self.data[y as usize];
        let mask = 0b1000_0000 >> x;

        if value {
            *line |= mask;
        } else {
            *line &= !mask;
        }
    }
}
