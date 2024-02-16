use crate::config::{tetromino_color, BOARD_SIZE};
use crate::tetromino::{Direction, Tetromino, TetrominoShape};
use rand::{seq::SliceRandom, thread_rng};
use ratatui::widgets::canvas::{Painter, Shape};
use std::num::TryFromIntError;

#[derive(Debug)]
pub enum TetrominoPositionError {
    NegativePosition,
    Collision,
}
impl From<TryFromIntError> for TetrominoPositionError {
    fn from(_: TryFromIntError) -> Self {
        TetrominoPositionError::NegativePosition
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Occupied(TetrominoShape),
}

#[derive(Debug)]
pub struct Board {
    grid: [[Cell; BOARD_SIZE.0]; BOARD_SIZE.1],
    bag: [TetrominoShape; 7],
    bag_index: usize,
    pub current_tetromino: Tetromino,
}
impl Board {
    pub fn new() -> Board {
        Board::default()
    }

    pub fn update(&mut self) -> Result<(), TetrominoPositionError> {
        match self.current_tetromino.calc_horizontal_move((0, 1)) {
            Ok(full_position) => {
                if self.check_collision(full_position) {
                    self.next_piece()?;
                    return Ok(());
                }
            }
            Err(_) => {
                self.next_piece()?;
            }
        }
        self.current_tetromino.update();
        Ok(())
    }

    fn clear_lines(&mut self) {
        for y in 0..BOARD_SIZE.1 {
            if self.grid[y]
                .iter()
                .all(|cell| matches!(cell, Cell::Occupied(_)))
            {
                self.grid[y] = [Cell::Empty; BOARD_SIZE.0];
                for y_to_move in (1..y + 1).rev() {
                    self.grid[y_to_move] = self.grid[y_to_move - 1];
                }
            }
        }
    }

    pub fn move_current_piece(&mut self, direction: Direction) {
        match self
            .current_tetromino
            .calc_horizontal_move((direction.into(), 0))
        {
            Ok(full_position) => {
                if !self.check_collision(full_position) {
                    self.current_tetromino.horizontal_move(direction);
                }
            }
            Err(_) => {}
        }
    }

    fn check_collision(&self, new_full_position: [(usize, usize); 4]) -> bool {
        new_full_position.iter().any(|(x, y)| {
            *x >= BOARD_SIZE.0
                || *y >= BOARD_SIZE.1
                || matches!(self.grid[*y][*x], Cell::Occupied(_))
        })
    }

    pub fn rotate_current_piece(&mut self, clockwise: bool) {
        for offset_index in 0..5 {
            let full_position_rotated = self.current_tetromino.calc_rotate(clockwise, offset_index);
            match full_position_rotated {
                Ok(full_position) => {
                    if !self.check_collision(full_position) {
                        let _ = self.current_tetromino.rotate(clockwise, offset_index);
                        return;
                    }
                }
                Err(_) => {}
            }
        }
    }

    fn next_piece(&mut self) -> Result<(), TetrominoPositionError> {
        if !self.current_tetromino.update_lock_delay() {
            return Ok(());
        }

        // fix current piece on the board
        self.current_tetromino
            .get_full_position()?
            .iter()
            .for_each(|(x, y)| {
                self.grid[*y][*x] = Cell::Occupied(self.current_tetromino.get_shape());
            });

        // spawn new piece
        self.bag_index += 1;
        if self.bag_index >= self.bag.len() {
            self.fill_bag();
            self.bag_index = 0;
        }
        self.current_tetromino = Tetromino::new(self.bag[self.bag_index]);
        if self.check_collision(self.current_tetromino.get_full_position()?) {
            return Err(TetrominoPositionError::Collision);
        }

        // clear lines
        self.clear_lines();

        Ok(())
    }

    fn fill_bag(&mut self) {
        self.bag = new_bag();
    }
}
impl Default for Board {
    fn default() -> Self {
        let starting_bag = new_bag();
        Board {
            grid: [[Cell::Empty; BOARD_SIZE.0]; BOARD_SIZE.1],
            bag: starting_bag,
            bag_index: 0,
            current_tetromino: starting_bag[0].into(),
        }
    }
}
impl Shape for Board {
    fn draw(&self, painter: &mut Painter) {
        // get current tetromino positions
        let tetromino_positions = self
            .current_tetromino
            .get_full_position()
            .expect("negative tetromino position while drawing");

        // draw borders
        let start_continuous = 5;
        for y in 0..start_continuous {
            if y % 2 != 0 {
                painter.paint(0, y + 1, tetromino_color(&self.current_tetromino.get_shape()));
                painter.paint(BOARD_SIZE.0 + 1, y + 1, tetromino_color(&self.current_tetromino.get_shape()));
            }
        }
        for y in start_continuous..BOARD_SIZE.1 {
            painter.paint(0, y + 1, tetromino_color(&self.current_tetromino.get_shape()));
            painter.paint(BOARD_SIZE.0 + 1, y + 1, tetromino_color(&self.current_tetromino.get_shape()));
        }
        for x in 0..BOARD_SIZE.0 + 2 {
            painter.paint(x, BOARD_SIZE.1 + 1, tetromino_color(&self.current_tetromino.get_shape()));
        }

        // draw the board
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                // draw the current tetromino
                if tetromino_positions.contains(&(x, y)) {
                    painter.paint(
                        x + 1,
                        y + 1,
                        tetromino_color(&self.current_tetromino.get_shape()),
                    );
                    continue;
                }

                // draw the existing board
                match cell {
                    Cell::Empty => {}
                    Cell::Occupied(shape) => painter.paint(x + 1, y + 1, tetromino_color(shape)),
                }
            }
        }
    }
}

fn new_bag() -> [TetrominoShape; 7] {
    let mut bag = [
        TetrominoShape::I,
        TetrominoShape::J,
        TetrominoShape::L,
        TetrominoShape::O,
        TetrominoShape::S,
        TetrominoShape::T,
        TetrominoShape::Z,
    ];
    bag.shuffle(&mut thread_rng());
    bag
}
