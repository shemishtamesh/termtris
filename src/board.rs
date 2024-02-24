use crate::config::{
    tetromino_color, tetromino_color_border, tetromino_color_ghost, tick_delay, BOARD_SIZE,
    LOCK_DELAY,
};
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
    next_bag: [TetrominoShape; 7],
    current_tetromino: Tetromino,
    held_tetromino: Option<TetrominoShape>,
    already_held: bool,
    score: u128,
    lines_cleared: u128,
    level: u8,
    pub tick_delay: u64,
}
impl Board {
    pub fn new() -> Board {
        Board::default()
    }

    pub fn update(&mut self) -> Result<(), TetrominoPositionError> {
        match self.current_tetromino.calc_horizontal_move((0, 1)) {
            Ok(full_position) => {
                if self.check_collision(full_position) {
                    self.piece_on_ground()?;
                    return Ok(());
                }
            }
            Err(_) => {
                self.piece_on_ground()?;
            }
        }
        self.current_tetromino.update();

        if self.tick_delay != tick_delay(self.level) {
            // soft dropping
            self.score += 1;
        }

        Ok(())
    }

    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
        for y in 0..BOARD_SIZE.1 {
            if self.grid[y]
                .iter()
                .all(|cell| matches!(cell, Cell::Occupied(_)))
            {
                // clear line
                self.grid[y] = [Cell::Empty; BOARD_SIZE.0];

                // move all lines above down
                for y_to_move in (1..y + 1).rev() {
                    self.grid[y_to_move] = self.grid[y_to_move - 1];
                }

                // update lines cleared counter
                lines_cleared += 1;
            }
        }

        // update score & level
        // self.score += lines_cleared * 100 * (self.level + 1) as u128;
        self.score += self.level as u128
            * match lines_cleared {
                0 => 0,
                1 => 100,
                2 => 300,
                3 => 500,
                4 => 800,
                _ => {
                    panic!(
                    "please file an issue at https://github.com/shemishtamesh/termtris/issues/new describing how you've cleared {} lines in one tick",
                    lines_cleared
                )
                }
            };
        self.lines_cleared += lines_cleared;
        if self.lines_cleared >= self.level as u128 * 10 + 10 {
            self.level += 1;
            self.tick_delay = tick_delay(self.level);
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

    pub fn hold(&mut self) -> Result<(), TetrominoPositionError> {
        if self.already_held {
            return Ok(());
        }
        self.already_held = true;

        match self.held_tetromino.clone() {
            Some(held_tetromino) => {
                self.held_tetromino = Some(self.current_tetromino.get_shape());
                self.spawn_tetromino(held_tetromino)?;
            }
            None => {
                self.held_tetromino = Some(self.current_tetromino.get_shape());
                self.spawn_next_piece()?;
            }
        }

        Ok(())
    }

    pub fn soft_drop(&mut self, activate: bool) {
        if activate {
            self.tick_delay = tick_delay(self.level) / 8;
            return;
        }
        self.tick_delay = tick_delay(self.level);
    }

    pub fn hard_drop(&mut self) -> Result<(), TetrominoPositionError> {
        let height = self.calc_relative_height()?;

        // increase score
        self.score += height as u128 as u128 * 2;

        for _ in 0..(self.calc_relative_height()? + LOCK_DELAY as usize) {
            let _ = self.update();
        }
        Ok(())
    }

    pub fn calc_relative_height(&self) -> Result<usize, TetrominoPositionError> {
        let height = self
            .current_tetromino
            .get_full_position()?
            .iter()
            .map(|(x_pos, y_pos)| {
                for y in (y_pos + 1)..BOARD_SIZE.1 {
                    if matches!(self.grid[y][*x_pos], Cell::Occupied(_)) {
                        return y - y_pos - 1;
                    }
                }
                BOARD_SIZE.1 - y_pos - 1
            })
            .min()
            .expect("current tetromino doesn't have a position");
        Ok(height)
    }

    fn spawn_tetromino(&mut self, shape: TetrominoShape) -> Result<(), TetrominoPositionError> {
        self.current_tetromino = Tetromino::new(shape);
        if self.check_collision(self.current_tetromino.get_full_position()?) {
            return Err(TetrominoPositionError::Collision);
        }
        Ok(())
    }

    fn spawn_next_piece(&mut self) -> Result<(), TetrominoPositionError> {
        self.bag_index += 1;
        if self.bag_index >= self.bag.len() {
            self.fill_bag();
            self.bag_index = 0;
        }
        self.spawn_tetromino(self.bag[self.bag_index])?;

        Ok(())
    }

    fn piece_on_ground(&mut self) -> Result<(), TetrominoPositionError> {
        // do not yet spawn next piece if current piece should not be locked yet
        if !self.current_tetromino.update_lock_delay() {
            return Ok(());
        }

        // lock current piece on the board
        self.current_tetromino
            .get_full_position()?
            .iter()
            .for_each(|(x, y)| {
                self.grid[*y][*x] = Cell::Occupied(self.current_tetromino.get_shape());
            });

        // spawn new piece
        self.spawn_next_piece()?;

        // clear lines
        self.clear_lines();

        // reenable holding
        self.already_held = false;

        Ok(())
    }

    // calculates a future tetromino's shape, can only return up to self.bag.len() pieces ahead
    pub fn calc_next_piece(&self, n: usize) -> TetrominoShape {
        assert!(
            n <= self.bag.len(),
            "can see only {} pieces ahead",
            self.bag.len()
        );

        if self.bag_index + n >= self.bag.len() {
            self.next_bag[self.bag_index + n - self.bag.len()]
        } else {
            self.bag[self.bag_index + n]
        }
    }

    fn fill_bag(&mut self) {
        self.bag = self.next_bag;
        self.next_bag = new_bag();
    }

    pub fn get_score(&self) -> u128 {
        self.score
    }

    pub fn get_lines_cleared(&self) -> u128 {
        self.lines_cleared
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }

    pub fn get_held_tetromino(&self) -> Option<TetrominoShape> {
        self.held_tetromino
    }
}
impl Default for Board {
    fn default() -> Self {
        let starting_bag = new_bag();
        Board {
            grid: [[Cell::Empty; BOARD_SIZE.0]; BOARD_SIZE.1],
            bag: starting_bag,
            bag_index: 0,
            next_bag: new_bag(),
            current_tetromino: starting_bag[0].into(),
            held_tetromino: None,
            already_held: false,
            tick_delay: tick_delay(1),
            score: 0,
            lines_cleared: 0,
            level: 1,
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
        let tetromino_height = self
            .calc_relative_height()
            .expect("could not calculate tetromino height");

        // draw borders
        let preview_piece = self.calc_next_piece(1);
        let start_continuous = 5;
        for y in 0..start_continuous {
            let color;
            if let Some(held) = self.held_tetromino {
                color = tetromino_color_border(&held);
            } else {
                color = tetromino_color_border(&preview_piece);
            }
            if y % 2 != 0 {
                painter.paint(0, y + 1, color);
                painter.paint(BOARD_SIZE.0 + 1, y + 1, color);
            }
        }
        for y in start_continuous..BOARD_SIZE.1 {
            painter.paint(0, y + 1, tetromino_color_border(&preview_piece));
            painter.paint(
                BOARD_SIZE.0 + 1,
                y + 1,
                tetromino_color_border(&preview_piece),
            );
        }
        for x in 0..BOARD_SIZE.0 + 2 {
            painter.paint(x, BOARD_SIZE.1 + 1, tetromino_color_border(&preview_piece));
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

                // draw ghost
                if tetromino_positions
                    .iter()
                    .any(|(x_pos, y_pos)| *x_pos == x && *y_pos + tetromino_height == y)
                {
                    painter.paint(
                        x + 1,
                        y + 1,
                        tetromino_color_ghost(&self.current_tetromino.get_shape()),
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
