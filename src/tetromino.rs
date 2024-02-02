use crate::config::BOARD_SIZE;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TetrominoShape {
    J,
    L,
    S,
    Z,
    O,
    T,
    I,
}

#[derive(Debug)]
pub struct Tetromino {
    shape: TetrominoShape,
    pos: (usize, usize),
    orientation: [(usize, usize); 4],
}
impl Tetromino {
    pub fn new(shape: TetrominoShape) -> Tetromino {
        match shape {
            TetrominoShape::I => Tetromino {
                shape,
                pos: (BOARD_SIZE.0 / 2, 0),
                orientation: [(0, 1), (1, 1), (2, 1), (3, 1)],
            },
            _ => unimplemented!(),
        }
    }

    pub fn get_shape(&self) -> TetrominoShape {
        self.shape
    }

    pub fn get_full_position(&self) -> [(usize, usize); 4] {
        self.orientation.map(|(x, y)| (self.pos.0 + x, self.pos.1 + y))
    }

    pub fn update(&mut self) {
        self.pos.1 += 1;
    }
}
