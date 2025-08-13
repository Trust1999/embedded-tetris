use rand::Rng;

#[derive(Clone, Copy)]
enum PieceKind {
    // ● ●
    // ● ●
    O,
    // ● ◌
    // ● ●
    // ● ◌
    T,
    // ● ◌
    // ● ●
    // ◌ ●
    S,
    // ◌ ●
    // ● ●
    // ● ◌
    Z,
    // ◌ ●
    // ◌ ●
    // ● ●
    J,
    // ● ◌
    // ● ◌
    // ● ●
    L,
    // ● ● ● ●
    I,
}

impl PieceKind {
    const fn block_offsets(self) -> &'static [(i8, i8)] {
        match self {
            PieceKind::O => &[(0, 0), (1, 0), (0, 1), (1, 1)],
            PieceKind::T => &[(0, 0), (0, 1), (1, 1), (0, 2)],
            PieceKind::S => &[(0, 0), (0, 1), (1, 1), (1, 2)],
            PieceKind::Z => &[(1, 0), (0, 1), (1, 1), (0, 2)],
            PieceKind::J => &[(1, 0), (1, 1), (0, 2), (1, 2)],
            PieceKind::L => &[(0, 0), (0, 1), (0, 2), (1, 2)],
            PieceKind::I => &[(0, 0), (1, 0), (2, 0), (3, 0)],
        }
    }

    const fn offset_for_rotation(self, rotation: Rotation) -> (i16, i16) {
        match self {
            PieceKind::O => match rotation {
                Rotation::Deg0 => (0, 0),
                Rotation::Deg90 => (0, -1),
                Rotation::Deg180 => (-1, -1),
                Rotation::Deg270 => (-1, 0),
            },
            PieceKind::T => match rotation {
                Rotation::Deg0 => (0, 0),
                Rotation::Deg90 => (-1, -1),
                Rotation::Deg180 => (-2, -1),
                Rotation::Deg270 => (-2, 1),
            },
            PieceKind::S => match rotation {
                Rotation::Deg0 => (0, 0),
                Rotation::Deg90 => (0, 0),
                Rotation::Deg180 => (-1, -1),
                Rotation::Deg270 => (-1, 1),
            },
            PieceKind::Z => match rotation {
                Rotation::Deg0 => (0, 0),
                Rotation::Deg90 => (-1, 0),
                Rotation::Deg180 => (-1, -1),
                Rotation::Deg270 => (-2, 1),
            },
            PieceKind::L | PieceKind::J => match rotation {
                Rotation::Deg0 => (0, 0),
                Rotation::Deg90 => (-1, 0),
                Rotation::Deg180 => (-1, 0),
                Rotation::Deg270 => (-1, 1),
            },
            PieceKind::I => match rotation {
                Rotation::Deg0 => (0, 0),
                Rotation::Deg90 => (2, -2),
                Rotation::Deg180 => (-1, -1),
                Rotation::Deg270 => (0, -1),
            },
        }
    }
}

#[derive(Clone)]
pub struct Piece {
    x: i16,
    y: i16,
    kind: PieceKind,
    rotation: Rotation,
}

impl Piece {
    pub fn random() -> Self {
        const BASE_VARIANTS: [Piece; 7] = [
            Piece::new(3, 3, PieceKind::O),
            Piece::new(3, 2, PieceKind::T),
            Piece::new(3, 2, PieceKind::S),
            Piece::new(3, 2, PieceKind::Z),
            Piece::new(3, 2, PieceKind::J),
            Piece::new(3, 2, PieceKind::L),
            Piece::new(2, 3, PieceKind::I),
        ];

        let mut rng = rand::rng();
        let variant_index = rng.random_range(0..BASE_VARIANTS.len());
        BASE_VARIANTS[variant_index].clone()
    }

    const fn new(x: i16, y: i16, kind: PieceKind) -> Self {
        Self {
            x,
            y,
            kind,
            rotation: Rotation::Deg0,
        }
    }

    pub fn rotate(&mut self, by: Rotation) {
        self.rotation = Rotation::from_u16((self.rotation.to_u16() + by.to_u16()) % 360);
    }

    pub fn move_to(&mut self, x: i16, y: i16) {
        self.x = x;
        self.y = y;
    }

    pub fn move_by(&mut self, dx: i16, dy: i16) {
        self.x += dx;
        self.y += dy;
    }

    pub fn aabb(&self) -> ((i16, i16), (i16, i16)) {
        (
            self.block_positions().min().unwrap(),
            self.block_positions().max().unwrap(),
        )
    }

    pub fn block_positions(&self) -> impl Iterator<Item = (i16, i16)> {
        let (width, height) = match self.kind {
            PieceKind::I => (4, 1),
            PieceKind::O => (2, 2),
            PieceKind::T => (2, 3),
            PieceKind::S => (2, 3),
            PieceKind::Z => (2, 3),
            PieceKind::J => (2, 3),
            PieceKind::L => (2, 3),
        };

        let (mx, my) = self.kind.offset_for_rotation(self.rotation);

        self.kind.block_offsets().iter().map(move |&(ox, oy)| {
            let (rx, ry) = match self.rotation {
                Rotation::Deg0 => (ox, oy),
                Rotation::Deg90 => (oy, width - ox),
                Rotation::Deg180 => (width - ox, height - oy),
                Rotation::Deg270 => (height - oy, ox),
            };

            (self.x + rx as i16 + mx, self.y + ry as i16 + my)
        })
    }
}

#[derive(Clone, Copy)]
pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Rotation {
    const fn from_u16(int: u16) -> Self {
        match int {
            0 => Rotation::Deg0,
            90 => Rotation::Deg90,
            180 => Rotation::Deg180,
            270 => Rotation::Deg270,
            _ => panic!(),
        }
    }

    const fn to_u16(self) -> u16 {
        match self {
            Rotation::Deg0 => 0,
            Rotation::Deg90 => 90,
            Rotation::Deg180 => 180,
            Rotation::Deg270 => 270,
        }
    }
}
