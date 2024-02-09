use std::num::TryFromIntError;
use std::ops::{Add, AddAssign};

use crate::config::BOARD_SIZE;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}
impl Into<isize> for Direction {
    fn into(self) -> isize {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

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
impl Into<Tetromino> for TetrominoShape {
    fn into(self) -> Tetromino {
        Tetromino::new(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}
impl Add<Direction> for Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Position {
        Position {
            x: self.x + (rhs == Direction::Right) as usize - (rhs == Direction::Left) as usize,
            y: self.y,
        }
    }
}
impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs
    }
}

#[derive(Debug)]
pub struct Tetromino {
    shape: TetrominoShape,
    pos: Position,
    orientation: [(isize, isize); 4],
}
impl Tetromino {
    pub fn new(shape: TetrominoShape) -> Tetromino {
        match shape {
            TetrominoShape::J => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 0),
                orientation: [(0, 0), (0, 1), (1, 1), (2, 1)],
            },
            TetrominoShape::L => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 0),
                orientation: [(0, 1), (1, 1), (2, 1), (2, 0)],
            },
            TetrominoShape::S => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 0),
                orientation: [(0, 1), (1, 1), (1, 0), (2, 0)],
            },
            TetrominoShape::Z => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 0),
                orientation: [(0, 0), (1, 0), (1, 1), (2, 1)],
            },
            TetrominoShape::O => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 0),
                orientation: [(0, 0), (0, 1), (1, 1), (1, 0)],
            },
            TetrominoShape::T => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 0),
                orientation: [(0, 1), (1, 1), (1, 0), (2, 1)],
            },
            TetrominoShape::I => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 0),
                orientation: [(0, 1), (1, 1), (2, 1), (3, 1)],
            },
        }
    }

    pub fn get_shape(&self) -> TetrominoShape {
        self.shape
    }

    pub fn get_position(&self) -> Position {
        self.pos
    }

    pub fn calculate_full_position(
        &self,
        diff: (isize, isize),
    ) -> Result<[(usize, usize); 4], TetrominoPositionError> {
        let mut full_position = [(0, 0); 4];
        for (i, (x, y)) in self.orientation.iter().enumerate() {
            full_position[i] = (
                usize::try_from(self.pos.x as isize + x + diff.0)?,
                usize::try_from(self.pos.y as isize + y + diff.1)?,
            )
        }
        return Ok(full_position);
    }

    pub fn get_full_position(&self) -> Result<[(usize, usize); 4], TetrominoPositionError> {
        let mut full_position = [(0, 0); 4];
        for (i, (x, y)) in self.orientation.iter().enumerate() {
            full_position[i] = (
                usize::try_from(self.pos.x as isize + x)?,
                usize::try_from(self.pos.y as isize + y)?,
            )
        }
        return Ok(full_position);
    }

    pub fn update(&mut self) {
        self.pos.y += 1;
    }

    pub fn horizontal_move(&mut self, direction: Direction) {
        self.pos += direction
    }
}

#[derive(Debug)]
pub enum TetrominoPositionError {
    NegativePosition,
}
impl From<TryFromIntError> for TetrominoPositionError {
    fn from(_: TryFromIntError) -> Self {
        TetrominoPositionError::NegativePosition
    }
}
