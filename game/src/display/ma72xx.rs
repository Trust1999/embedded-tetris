use embedded_hal::spi::SpiDevice;

pub struct Max72xx<SPI> {
    spi: SPI,
    bitmap: Vec<u8>,
    displays: usize,
    rotations: Vec<Rotation>,
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

#[allow(unused)]
mod op {
    pub const NOOP: u8 = 0x0;
    pub const DIGIT0: u8 = 0x1;
    pub const DIGIT7: u8 = 0x8;
    pub const DECODEMODE: u8 = 0x9;
    pub const INTENSITY: u8 = 0xa;
    pub const SCANLIMIT: u8 = 0xb;
    pub const SHUTDOWN: u8 = 0xc;
    pub const DISPLAYTEST: u8 = 0xf;
}

#[allow(unused)]
#[repr(u8)]
enum DecodeMode {
    DecodeNo = 0x00,
    Decode0 = 0x01,
    Decode3_0 = 0x0f,
    Decode7_0 = 0xff,
}

impl<E, SPI: SpiDevice<Error = E>> Max72xx<SPI> {
    pub fn new(spi: SPI, displays: usize) -> Self {
        Self {
            spi,
            bitmap: vec![0x00; displays * 8],
            displays,
            rotations: vec![Rotation::Deg0; displays],
        }
    }

    pub fn reset(&mut self) -> Result<(), E> {
        // Make sure we are not in test mode
        self.transfer_single_op(op::DISPLAYTEST, 0x00)?;

        // We need the multiplexer to scan all segments
        self.transfer_single_op(op::SCANLIMIT, 7)?;

        // We don't want the multiplexer to decode segments for us
        self.transfer_single_op(op::DECODEMODE, DecodeMode::DecodeNo as u8)?;

        // Enable display
        self.set_shutdown(false)?;

        // Set the brightness to a medium value
        self.set_intensity(2)?;

        Ok(())
    }

    pub fn set_shutdown(&mut self, value: bool) -> Result<(), E> {
        self.transfer_single_op(op::SHUTDOWN, if value { 0x00 } else { 0x01 })
    }

    pub fn set_intensity(&mut self, intensity: u8) -> Result<(), E> {
        self.transfer_single_op(op::INTENSITY, intensity)
    }

    pub fn transfer_bitmap(&mut self) -> Result<(), E> {
        for row in 0u8..8 {
            self.transfer_row(row)?;
        }
        Ok(())
    }

    fn transfer_single_op(&mut self, opcode: u8, data: u8) -> Result<(), E> {
        for _ in 0..self.displays {
            self.spi.write(&[opcode, data])?;
        }
        Ok(())
    }

    fn transfer_row(&mut self, row: u8) -> Result<(), E> {
        assert!(row < 8);

        let opcode = op::DIGIT0 + row;

        let mut buffer = Vec::with_capacity(self.displays * 2);
        for display in (0..self.displays).rev() {
            let data = self.bitmap[display * 8 + row as usize];
            buffer.extend_from_slice(&[opcode, data]);
        }

        self.spi.write(&buffer)
    }
}

impl<SPI> super::Display for Max72xx<SPI> {
    fn fill(&mut self, value: bool) {
        let line = if value { 0xff } else { 0x00 };
        self.bitmap.fill(line);
    }

    fn set_pixel(&mut self, x: u8, y: u8, value: bool) {
        let width_per_display = 8u8;
        let height_per_display = 8u8;
        let total_height = self.displays * height_per_display as usize;

        if x >= width_per_display || y as usize >= total_height {
            return;
        }

        // Identify which vertical display block we are in
        let display = (y / height_per_display) as usize;

        // Compute local coordinates within the 8x8 block
        let mut local_x = x % width_per_display;
        let mut local_y = y % height_per_display;

        // Get the rotation of this display
        let rotation = self.rotations[display];

        // Apply rotation transformations
        match rotation {
            Rotation::Deg0 => {
                // No change needed
            }
            Rotation::Deg90 => {
                // Rotate 90 degrees clockwise:
                // Swap x and y, then flip y
                let temp = local_x;
                local_x = local_y;
                local_y = height_per_display - 1 - temp;
            }
            Rotation::Deg180 => {
                // Rotate 180 degrees clockwise:
                // Flip both x and y
                local_x = width_per_display - 1 - local_x;
                local_y = height_per_display - 1 - local_y;
            }
            Rotation::Deg270 => {
                // Rotate 270 degrees clockwise:
                // Swap x and y, then flip x
                let temp = local_x;
                local_x = height_per_display - 1 - local_y;
                local_y = temp;
            }
        }

        // Convert local_y back to global y position
        let global_y = local_y + (display as u8) * height_per_display;

        // Calculate the index into the bitmap vector
        // Each byte represents one vertical column of 8 pixels,
        // so row index = global_y divided by 8,
        // column index = local_x
        let row = (global_y / height_per_display) as usize;
        let col = local_x as usize;
        let index = col + width_per_display as usize * row;

        // Calculate which bit within the byte to set/clear
        let bit_position = global_y % height_per_display;

        let mask = 1 << bit_position;
        let line = &mut self.bitmap[index];
        if value {
            *line |= mask;
        } else {
            *line &= !mask;
        }
    }
}

impl<SPI> std::fmt::Display for Max72xx<SPI> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.bitmap {
            writeln!(f, "{:08b}", line)?;
        }
        Ok(())
    }
}
