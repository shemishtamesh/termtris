use std::num::TryFromIntError;
use std::ops::{Add, AddAssign};

use crate::config::BOARD_SIZE;

const O_ROTATION_OFFSETS: [[(isize, isize); 5]; 4] = [
    [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(0, -1), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(-1, -1), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(-1, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
];
const I_ROTATION_OFFSETS: [[(isize, isize); 5]; 4] = [
    [(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)],
    [(-1, 0), (0, 0), (0, 0), (0, 1), (0, -2)],
    [(-1, 1), (1, 1), (-2, 1), (1, 0), (-2, 0)],
    [(0, 1), (0, 1), (0, 1), (0, -1), (0, 2)],
];
const JLSTZ_ROTATION_OFFSETS: [[(isize, isize); 5]; 4] = [
    [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
];

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
    rotation_index: usize,
}
impl Tetromino {
    pub fn new(shape: TetrominoShape) -> Tetromino {
        match shape {
            TetrominoShape::J => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 2),
                orientation: [(-1, -1), (-1, 0), (0, 0), (1, 0)],
                rotation_index: 0,
            },
            TetrominoShape::L => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 2),
                orientation: [(-1, 0), (0, 0), (1, 0), (1, -1)],
                rotation_index: 0,
            },
            TetrominoShape::S => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 2),
                orientation: [(-1, 0), (0, 0), (0, -1), (1, -1)],
                rotation_index: 0,
            },
            TetrominoShape::Z => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 2),
                orientation: [(-1, -1), (0, -1), (0, 0), (1, 0)],
                rotation_index: 0,
            },
            TetrominoShape::O => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 2),
                orientation: [(0, -1), (0, 0), (1, -1), (1, 0)],
                rotation_index: 0,
            },
            TetrominoShape::T => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 2),
                orientation: [(-1, 0), (0, 0), (0, -1), (1, 0)],
                rotation_index: 0,
            },
            TetrominoShape::I => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 2, 2),
                orientation: [(-1, 0), (0, 0), (1, 0), (2, 0)],
                rotation_index: 0,
            },
        }
    }

    pub fn get_shape(&self) -> TetrominoShape {
        self.shape
    }

    pub fn get_position(&self) -> Position {
        self.pos
    }

    pub fn calc_full_pos(
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

    pub fn rotate(
        &mut self,
        clockwise: bool,
        offset_index: usize,
    ) -> Result<(), TetrominoPositionError> {
        self.orientation = self.orientation.map(|(x, y)| {
            (
                y * -(!clockwise as isize) + y * (clockwise as isize),
                x * -(clockwise as isize) + x * (!clockwise as isize),
            )
        });

        let offset = self.get_rotation_offsets()[self.rotation_index][offset_index];
        usize::try_from(self.pos.x as isize + offset.0)?;
        usize::try_from(self.pos.y as isize + offset.1)?;

        self.rotation_index += clockwise as usize;
        self.rotation_index %= 4;
        Ok(())
    }

    fn get_rotation_offsets(&self) -> [[(isize, isize); 5]; 4] {
        match self.shape {
            TetrominoShape::O => O_ROTATION_OFFSETS,
            TetrominoShape::I => I_ROTATION_OFFSETS,
            _ => JLSTZ_ROTATION_OFFSETS,
        }
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
