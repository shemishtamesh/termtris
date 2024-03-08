use ratatui::{
    prelude::{Buffer, Rect},
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Painter, Shape},
        Block, BorderType, Borders, Widget,
    },
};

use std::ops::{Add, AddAssign};

use crate::{
    board::TetrominoPositionError,
    config::{tetromino_color, BOARD_SIZE, LOCK_DELAY},
};

// y component is inverted because (0, 0) is in the top left
const O_ROTATION_OFFSETS: [[(isize, isize); 5]; 4] = [
    // values in columns other than the first don't matter
    [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 1), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(-1, 1), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(-1, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
];
const I_ROTATION_OFFSETS: [[(isize, isize); 5]; 4] = [
    [(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)],
    [(-1, 0), (0, 0), (0, 0), (0, -1), (0, 2)],
    [(-1, -1), (1, -1), (-2, -1), (1, 0), (-2, 0)],
    [(0, -1), (0, -1), (0, -1), (0, 1), (0, -2)],
];
const JLSTZ_ROTATION_OFFSETS: [[(isize, isize); 5]; 4] = [
    [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
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

#[derive(Debug, Clone)]
pub struct Tetromino {
    shape: TetrominoShape,
    pos: Position,
    orientation: [(isize, isize); 4],
    rotation_index: usize,
    lock_delay: u8,
}
impl Tetromino {
    pub fn new(shape: TetrominoShape) -> Tetromino {
        match shape {
            TetrominoShape::J => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 1, 2),
                orientation: [(-1, -1), (-1, 0), (0, 0), (1, 0)],
                rotation_index: 0,
                lock_delay: LOCK_DELAY,
            },
            TetrominoShape::L => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 1, 2),
                orientation: [(-1, 0), (0, 0), (1, 0), (1, -1)],
                rotation_index: 0,
                lock_delay: LOCK_DELAY,
            },
            TetrominoShape::S => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 1, 2),
                orientation: [(-1, 0), (0, 0), (0, -1), (1, -1)],
                rotation_index: 0,
                lock_delay: LOCK_DELAY,
            },
            TetrominoShape::Z => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 1, 2),
                orientation: [(-1, -1), (0, -1), (0, 0), (1, 0)],
                rotation_index: 0,
                lock_delay: LOCK_DELAY,
            },
            TetrominoShape::O => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 1, 4),
                orientation: [(0, -1), (0, 0), (1, -1), (1, 0)],
                rotation_index: 0,
                lock_delay: LOCK_DELAY,
            },
            TetrominoShape::T => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 1, 2),
                orientation: [(-1, 0), (0, 0), (0, -1), (1, 0)],
                rotation_index: 0,
                lock_delay: LOCK_DELAY,
            },
            TetrominoShape::I => Tetromino {
                shape,
                pos: Position::new(BOARD_SIZE.0 / 2 - 1, 2),
                orientation: [(-1, 0), (0, 0), (1, 0), (2, 0)],
                rotation_index: 0,
                lock_delay: LOCK_DELAY,
            },
        }
    }

    pub fn get_shape(&self) -> TetrominoShape {
        self.shape
    }

    pub fn get_position(&self) -> Position {
        self.pos
    }

    pub fn calc_horizontal_move(
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

    pub fn update_lock_delay(&mut self) -> bool {
        self.lock_delay -= 1;
        self.lock_delay == 0
    }

    pub fn horizontal_move(&mut self, direction: Direction) {
        self.pos += direction
    }

    pub fn calc_rotate(
        &mut self,
        clockwise: bool,
        offset_index: usize,
    ) -> Result<[(usize, usize); 4], TetrominoPositionError> {
        let original_pos = self.pos;
        let original_orientation = self.orientation;
        let original_rotation_index = self.rotation_index;

        let return_value;
        match self.rotate(clockwise, offset_index) {
            Ok(_) => {
                return_value = self.get_full_position();
            }
            Err(_) => {
                return_value = Err(TetrominoPositionError::NegativePosition);
            }
        }

        self.pos = original_pos;
        self.orientation = original_orientation;
        self.rotation_index = original_rotation_index;

        return return_value;
    }

    pub fn rotate(
        &mut self,
        clockwise: bool,
        offset_index: usize,
    ) -> Result<(), TetrominoPositionError> {
        // rotate
        self.orientation = self.orientation.map(|(x, y)| {
            (
                y * -(clockwise as isize) + y * (!clockwise as isize),
                x * -(!clockwise as isize) + x * (clockwise as isize),
            )
        });

        // update rotation_index
        let prev_rotation_index = self.rotation_index;
        // the +4 is there to avoid casting into isize and back
        self.rotation_index =
            (4 + self.rotation_index + (clockwise as usize) - (!clockwise as usize)) % 4;

        // apply offset
        let first_offset = self.get_rotation_offsets()[prev_rotation_index][offset_index];
        let second_offset = self.get_rotation_offsets()[self.rotation_index][offset_index];
        self.pos.x = usize::try_from(self.pos.x as isize + first_offset.0 - second_offset.0)?;
        self.pos.y = usize::try_from(self.pos.y as isize + first_offset.1 - second_offset.1)?;

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
impl Shape for Tetromino {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.orientation {
            painter.paint(
                (x + 1) as usize,
                (y + 1) as usize,
                tetromino_color(&self.shape),
            );
        }
    }
}
impl Widget for Tetromino {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .x_bounds([0.0, 4.0])
            .y_bounds([0.0, 4.0])
            .marker(Marker::HalfBlock)
            .paint(|ctx| ctx.draw(&self))
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut tetromino = Tetromino::new(TetrominoShape::Z);
        tetromino.rotate(false, 0).expect("failed to rotate Z");
        assert_eq!(tetromino.orientation, [(-1, 1), (-1, 0), (0, 0), (0, -1)]);

        let mut tetromino = Tetromino::new(TetrominoShape::O);
        tetromino.rotate(false, 0).expect("failed to rotate O");
        assert_eq!(tetromino.orientation, [(-1, 0), (0, 0), (-1, -1), (0, -1)]);
    }

    #[test]
    fn test_get_full_position() {
        let tetromino = Tetromino::new(TetrominoShape::I);
        let full_position = tetromino
            .get_full_position()
            .expect("failed to get I full position");
        assert_eq!(full_position, [(3, 2), (4, 2), (5, 2), (6, 2)]);

        let tetromino = Tetromino::new(TetrominoShape::J);
        let full_position = tetromino
            .get_full_position()
            .expect("failed to get J full position");
        assert_eq!(full_position, [(3, 1), (3, 2), (4, 2), (5, 2)]);

        let tetromino = Tetromino::new(TetrominoShape::T);
        let full_position = tetromino
            .get_full_position()
            .expect("failed to get T full position");
        assert_eq!(full_position, [(3, 2), (4, 2), (4, 1), (5, 2)]);

        let tetromino = Tetromino::new(TetrominoShape::Z);
        let full_position = tetromino
            .get_full_position()
            .expect("failed to get Z full position");
        assert_eq!(full_position, [(3, 1), (4, 1), (4, 2), (5, 2)]);

        let tetromino = Tetromino::new(TetrominoShape::L);
        let full_position = tetromino
            .get_full_position()
            .expect("failed to get L full position");
        assert_eq!(full_position, [(3, 2), (4, 2), (5, 2), (5, 1)]);

        let tetromino = Tetromino::new(TetrominoShape::S);
        let full_position = tetromino
            .get_full_position()
            .expect("failed to get S full position");
        assert_eq!(full_position, [(3, 2), (4, 2), (4, 1), (5, 1)]);

        let tetromino = Tetromino::new(TetrominoShape::O);
        let full_position = tetromino
            .get_full_position()
            .expect("failed to get O full position");
        assert_eq!(full_position, [(4, 3), (4, 4), (5, 3), (5, 4)]);
    }
}
