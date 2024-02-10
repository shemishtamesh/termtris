use crate::config::{tetromino_color, BOARD_SIZE};
use crate::tetromino::{Direction, Tetromino, TetrominoPositionError, TetrominoShape};
use rand::{seq::SliceRandom, thread_rng};
use ratatui::widgets::canvas::{Painter, Shape};

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
        match self.current_tetromino.calc_full_pos((0, 1)) {
            Ok(full_position) => {
                if self.check_collision(full_position) {
                    self.next_piece()?;
                }
            }
            Err(_) => {
                self.next_piece()?;
            }
        }
        self.current_tetromino.update();
        Ok(())
    }

    pub fn move_current_piece(&mut self, direction: Direction) {
        match self.current_tetromino.calc_full_pos((direction.into(), 0)) {
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
        self.current_tetromino.rotate(clockwise, 0);
    }

    fn next_piece(&mut self) -> Result<(), TetrominoPositionError> {
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
        // TODO: draw the border here, it should be only bottom and sides, and the sides should
        // be lower than the new tetromino
        let tetromino_positions = self
            .current_tetromino
            .get_full_position()
            .expect("negative tetromino position while drawing");
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if tetromino_positions.contains(&(x, y)) {
                    painter.paint(x, y, tetromino_color(&self.current_tetromino.get_shape()));
                    continue;
                }
                match cell {
                    Cell::Empty => {}
                    Cell::Occupied(shape) => painter.paint(x, y, tetromino_color(shape)),
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
