use crate::config::{BagType, CONFIG};
use crate::tetromino::{Direction, Tetromino, TetrominoShape};
use rand::rngs::ThreadRng;
use rand::{seq::SliceRandom, thread_rng};
use ratatui::widgets::canvas::{Painter, Shape};
use std::num::TryFromIntError;

#[derive(Debug, PartialEq, Eq)]
enum DifficultClear {
    Tetris,
    TSpinDouble,
    TspinTriple,
}

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
    grid: Vec<Vec<Cell>>,
    bag: Vec<TetrominoShape>,
    bag_index: usize,
    next_bag: Vec<TetrominoShape>,
    current_tetromino: Tetromino,
    held_tetromino: Option<TetrominoShape>,
    already_held: bool,
    last_rotation_check: Option<usize>, // last rotation check index, if there was any rotation
    last_difficult_clear: Option<DifficultClear>,
    combo_count: u8,
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

        if self.tick_delay != CONFIG.tick_delay[&self.level] {
            // soft dropping
            self.score += 1;
        }

        // reset the last rotation check
        self.last_rotation_check = None;

        Ok(())
    }

    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
        for y in 0..CONFIG.board_size.1 {
            if self.grid[y]
                .iter()
                .all(|cell| matches!(cell, Cell::Occupied(_)))
            {
                // clear line
                self.grid[y] = vec![Cell::Empty; CONFIG.board_size.0];

                // move all lines above down
                for y_to_move in (1..y + 1).rev() {
                    self.grid[y_to_move] = self.grid[y_to_move - 1].clone();
                }

                // update lines cleared counter
                lines_cleared += 1;
            }
        }

        // update score
        let mut difficult_clear = None;
        let mut additional_score = self.level as u128
            * match (
                lines_cleared,
                self.current_tetromino.get_shape(),
                self.last_rotation_check,
            ) {
                (2, TetrominoShape::T, Some(_)) => {
                    difficult_clear = Some(DifficultClear::TSpinDouble);
                    1_200
                }
                (3, TetrominoShape::T, Some(_)) => {
                    difficult_clear = Some(DifficultClear::TspinTriple);
                    1_600
                }
                (0, _, _) => 0,
                (1, _, _) => 100,
                (2, _, _) => 300,
                (3, _, _) => 500,
                (4, _, _) => {
                    difficult_clear = Some(DifficultClear::Tetris);
                    800
                }
                _ => {
                    panic!(
                        "please file an issue at https://github.com/shemishtamesh/termtris/issues/new describing how you've cleared {} lines in one tick",
                        lines_cleared
                    )
                }
            };
        if difficult_clear.is_some() && difficult_clear == self.last_difficult_clear {
            // back to back
            additional_score = (additional_score as f32 * 1.5) as u128;
        }
        if additional_score > 0 {
            // lines were cleared
            // combo
            additional_score += self.combo_count as u128 * 50 * self.level as u128;
            self.combo_count += 1;

            // set the last difficult_clear to the current
            self.last_difficult_clear = difficult_clear;
        }
        if self.grid[CONFIG.board_size.1 - 1]
            .iter()
            .all(|cell| matches!(cell, Cell::Empty))
        {
            // perfect clear
            additional_score += 5_000;
        }
        self.score += additional_score;

        // update lines cleared count
        self.lines_cleared += lines_cleared;

        // update level
        if self.lines_cleared >= self.level as u128 * 10 + 10 {
            self.level += 1;
            self.tick_delay = CONFIG.tick_delay[&self.level];
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
            *x >= CONFIG.board_size.0
                || *y >= CONFIG.board_size.1
                || matches!(self.grid[*y][*x], Cell::Occupied(_))
        })
    }

    pub fn rotate_current_piece(&mut self, clockwise: bool) {
        for offset_index in 0..5 {
            let full_position_rotated = self.current_tetromino.calc_rotate(clockwise, offset_index);
            self.last_rotation_check = Some(offset_index);
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
            self.tick_delay = CONFIG.tick_delay[&self.level] / 8;
            return;
        }
        self.tick_delay = CONFIG.tick_delay[&self.level];
    }

    pub fn hard_drop(&mut self) -> Result<(), TetrominoPositionError> {
        let height = self.calc_relative_height()?;

        // increase score
        self.score += height as u128 as u128 * 2;

        for _ in 0..(self.calc_relative_height()? + CONFIG.lock_delay as usize) {
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
                for y in (y_pos + 1)..CONFIG.board_size.1 {
                    if matches!(self.grid[y][*x_pos], Cell::Occupied(_)) {
                        return y - y_pos - 1;
                    }
                }
                CONFIG.board_size.1 - y_pos - 1
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

        // clear lines
        self.clear_lines();

        // spawn new piece
        self.spawn_next_piece()?;

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
        self.bag = self.next_bag.clone();
        self.next_bag = new_bag(&CONFIG.bag_type);
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
        let starting_bag = new_bag(&CONFIG.bag_type);
        Board {
            grid: vec![vec![Cell::Empty; CONFIG.board_size.0]; CONFIG.board_size.1],
            bag: starting_bag.clone(),
            bag_index: 0,
            next_bag: new_bag(&CONFIG.bag_type),
            current_tetromino: starting_bag[0].into(),
            held_tetromino: None,
            already_held: false,
            last_rotation_check: None, // last rotation check index, if there was any rotation
            last_difficult_clear: None,
            combo_count: 0,
            tick_delay: CONFIG.tick_delay[&1],
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
                color = CONFIG.border_color[&held];
            } else {
                color = CONFIG.border_color[&preview_piece];
            }
            if y % 2 != 0 {
                painter.paint(0, y + 1, color);
                painter.paint(CONFIG.board_size.0 + 1, y + 1, color);
            }
        }
        for y in start_continuous..CONFIG.board_size.1 {
            painter.paint(0, y + 1, CONFIG.border_color[&preview_piece]);
            painter.paint(
                CONFIG.board_size.0 + 1,
                y + 1,
                CONFIG.border_color[&preview_piece],
            );
        }
        for x in 0..CONFIG.board_size.0 + 2 {
            painter.paint(
                x,
                CONFIG.board_size.1 + 1,
                CONFIG.border_color[&preview_piece],
            );
        }

        // draw the board
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                // draw the current tetromino
                if tetromino_positions.contains(&(x, y)) {
                    painter.paint(
                        x + 1,
                        y + 1,
                        CONFIG.tetromino_color[&self.current_tetromino.get_shape()],
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
                        CONFIG.ghost_color[&self.current_tetromino.get_shape()],
                    );
                    continue;
                }

                // draw the existing board
                match cell {
                    Cell::Empty => {}
                    Cell::Occupied(shape) => {
                        painter.paint(x + 1, y + 1, CONFIG.tetromino_color[shape])
                    }
                }
            }
        }
    }
}

fn new_bag(bag_type: &BagType) -> Vec<TetrominoShape> {
    match bag_type {
        BagType::Classic => classic_bag(),
        BagType::Fourteen => fourteen_bag(),
        BagType::Seven => seven_bag(),
        BagType::Pairs => pairs_bag(),
    }
}

fn seven_bag() -> Vec<TetrominoShape> {
    let mut bag = vec![
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

fn pairs_bag() -> Vec<TetrominoShape> {
    let mut bag: Vec<TetrominoShape> = Vec::with_capacity(14);
    let mut rng: ThreadRng = thread_rng();

    for _ in 0..7 {
        let random_number = rand::Rng::gen_range(&mut rng, 0..7);
        let random_tetromino = match random_number {
            0 => TetrominoShape::I,
            1 => TetrominoShape::J,
            2 => TetrominoShape::L,
            3 => TetrominoShape::O,
            4 => TetrominoShape::S,
            5 => TetrominoShape::T,
            6 => TetrominoShape::Z,
            _ => unreachable!(),
        };
        bag.push(random_tetromino); // Push the randomly selected Tetromino into the Vec twice
        bag.push(random_tetromino);
    }
    bag
}

fn fourteen_bag() -> Vec<TetrominoShape> {
    let mut bag: Vec<TetrominoShape> = vec![
        TetrominoShape::I,
        TetrominoShape::J,
        TetrominoShape::L,
        TetrominoShape::O,
        TetrominoShape::S,
        TetrominoShape::T,
        TetrominoShape::Z,
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

fn classic_bag() -> Vec<TetrominoShape> {
    let mut bag: Vec<TetrominoShape> = Vec::with_capacity(7); // Create a Vec with a capacity of 7
    let mut rng: ThreadRng = thread_rng();

    for _ in 0..7 {
        let random_number = rand::Rng::gen_range(&mut rng, 0..7); // Changed to use the rng
        let random_tetromino = match random_number {
            0 => TetrominoShape::I,
            1 => TetrominoShape::J,
            2 => TetrominoShape::L,
            3 => TetrominoShape::O,
            4 => TetrominoShape::S,
            5 => TetrominoShape::T,
            6 => TetrominoShape::Z,
            _ => unreachable!(),
        };
        bag.push(random_tetromino); // Push the randomly selected Tetromino into the Vec
    }
    bag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bag() {
        let bag = new_bag(&CONFIG.bag_type);
        match CONFIG.bag_type {
            BagType::Classic | BagType::Seven => assert_eq!(bag.len(), 7),
            BagType::Fourteen | BagType::Pairs => assert_eq!(bag.len(), 14),
        }
    }
}
