use rand::Rng;

const MAX_WIDTH: usize = 4;
const MAX_HEIGHT: usize = 4;
const MAX_SIZE: usize = MAX_WIDTH * MAX_HEIGHT;

#[derive(Clone)]
pub struct Piece {
    pub position_x: i16,
    pub position_y: i16,
    blocks: [bool; 16],
    width: u8,
    height: u8,
}

impl Piece {
    pub fn random() -> Self {
        const BASE_VARIANTS: [Piece; 5] = [
            // ● ●
            // ● ●
            Piece::new(&[true, true, true, true], 2, 2),
            // ◌ ◌ ●
            // ● ● ●
            Piece::new(&[false, false, true, true, true, true], 3, 2),
            // ◌ ● ◌
            // ● ● ●
            Piece::new(&[false, true, false, true, true, true], 3, 2),
            // ◌ ● ●
            // ● ● ◌
            Piece::new(&[false, true, true, true, true, false], 3, 2),
            // ● ● ● ●
            Piece::new(&[true, true, true, true], 4, 1),
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
        let rotation = match rng.random_range(0..3) {
            0 => Rotation::Deg0,
            1 => Rotation::Deg90,
            2 => Rotation::Deg180,
            _ => Rotation::Deg270,
        };
        piece.rotate(rotation);

        piece
    }

    pub fn rotate(&mut self, by: Rotation) {
        // TODO
    }

    pub fn mirror(&mut self) {
        // TODO
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

    const fn new<const SIZE: usize>(blocks: &[bool; SIZE], width: u8, height: u8) -> Self {
        let mut _blocks = [false; MAX_SIZE];
        let (used_chunk, _) = _blocks.split_at_mut(SIZE);
        used_chunk.copy_from_slice(blocks);
        Self {
            position_x: 0,
            position_y: 0,
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
