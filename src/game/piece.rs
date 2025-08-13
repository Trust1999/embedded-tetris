use rand::Rng;

const MAX_WIDTH: usize = 4;
const MAX_HEIGHT: usize = 4;
const MAX_SIZE: usize = MAX_WIDTH * MAX_HEIGHT;

#[derive(Clone)]
pub struct Piece {
    pub position_x: i16,
    pub position_y: i16,
    pub width: u8,
    pub height: u8,
    blocks: [bool; MAX_SIZE],
}

impl Piece {
    pub fn random() -> Self {
        const BASE_VARIANTS: [Piece; 5] = [
            // ● ●
            // ● ●
            Piece::new(
                &[true, true, true, true],
                (2, 2), // size
                (3, 2), // position
            ),
            // ● ◌
            // ● ◌
            // ● ●
            Piece::new(
                &[true, false, true, false, true, true],
                (2, 3), // size
                (3, 2), // position
            ),
            // ● ◌
            // ● ●
            // ● ◌
            Piece::new(
                &[true, false, true, true, true, false],
                (2, 3), // size
                (3, 2), // position
            ),
            // ● ◌
            // ● ●
            // ◌ ●
            Piece::new(
                &[true, false, true, true, false, true],
                (2, 3), // size
                (3, 2), // position
            ),
            // ● ● ● ●
            Piece::new(
                &[true, true, true, true],
                (4, 1), // size
                (2, 3), // position
            ),
        ];

        let mut rng = rand::rng();

        // Random base variant
        let variant_index = rng.random_range(0..BASE_VARIANTS.len());
        let mut piece = BASE_VARIANTS[variant_index].clone();

        // Mirror
        if rng.random::<bool>() {
            piece.mirror();
        }

        // Rotate by random amount
        let rotation = match rng.random_range(0..1) {
            0 => Rotation::Deg0,
            _ => Rotation::Deg180,
        };
        piece.rotate(rotation);

        piece
    }

    pub fn rotate(&mut self, by: Rotation) {
        let old_w = self.width as usize;
        let old_h = self.height as usize;

        // Temporary storage for previous state
        let prev = self.blocks;
        self.blocks = [false; MAX_SIZE];

        match by {
            Rotation::Deg0 => {
                // Copy unchanged
                self.blocks = prev;
            }
            Rotation::Deg90 => {
                // New width = old height, new height = old width
                for y in 0..old_h {
                    for x in 0..old_w {
                        let new_x = old_h - 1 - y;
                        let new_y = x;
                        self.blocks[new_y * old_h + new_x] = prev[y * old_w + x];
                    }
                }
                self.width = old_h as u8;
                self.height = old_w as u8;
            }
            Rotation::Deg180 => {
                for y in 0..old_h {
                    for x in 0..old_w {
                        let new_x = old_w - 1 - x;
                        let new_y = old_h - 1 - y;
                        self.blocks[new_y * old_w + new_x] = prev[y * old_w + x];
                    }
                }
            }
            Rotation::Deg270 => {
                // New width = old height, new height = old width
                for y in 0..old_h {
                    for x in 0..old_w {
                        let new_x = y;
                        let new_y = old_w - 1 - x;
                        self.blocks[new_y * old_h + new_x] = prev[y * old_w + x];
                    }
                }
                self.width = old_h as u8;
                self.height = old_w as u8;
            }
        }
    }

    pub fn mirror(&mut self) {
        for row in self.rows_mut() {
            row.reverse();
        }
    }

    pub fn move_to(&mut self, x: i16, y: i16) {
        self.position_x = x;
        self.position_y = y;
    }

    pub fn move_by(&mut self, delta_x: i16, delta_y: i16) {
        self.position_x += delta_x;
        self.position_y += delta_y;
    }

    pub fn rows(&self) -> impl Iterator<Item = &[bool]> {
        self.blocks
            .chunks_exact(self.width as usize)
            .take(self.height as usize)
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [bool]> {
        self.blocks
            .chunks_exact_mut(self.width as usize)
            .take(self.height as usize)
    }

    pub fn filled_positions(&self) -> impl Iterator<Item = (i16, i16)> {
        self.rows().enumerate().flat_map(move |(local_y, row)| {
            row.iter().enumerate().flat_map(move |(local_x, block)| {
                block.then_some((
                    self.position_x + local_x as i16,
                    self.position_y + local_y as i16,
                ))
            })
        })
    }

    const fn new<const SIZE: usize>(
        blocks: &[bool; SIZE],
        (width, height): (u8, u8),
        (position_x, position_y): (i16, i16),
    ) -> Self {
        let mut _blocks = [false; MAX_SIZE];
        let (used_chunk, _) = _blocks.split_at_mut(SIZE);
        used_chunk.copy_from_slice(blocks);
        Self {
            position_x,
            position_y,
            blocks: _blocks,
            width,
            height,
        }
    }
}

pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}
