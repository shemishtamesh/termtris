use crate::config::{tetromino_color, BOARD_SIZE};
use crate::tetromino::{Tetromino, TetrominoShape};
use ratatui::widgets::canvas::{Painter, Shape};

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Occupied(TetrominoShape),
}

#[derive(Debug)]
pub struct Board {
    grid: [[Cell; BOARD_SIZE.0]; BOARD_SIZE.1],
    current_tetromino: Tetromino,
    bag: [TetrominoShape; 7],
    bag_index: usize,
}
impl Board {
    pub fn new() -> Board {
        Board::default()
    }

    pub fn update(&mut self) {
        if self
            .current_tetromino
            .get_full_position()
            .iter()
            .any(|(x, y)| {
                *y == BOARD_SIZE.1 - 1 || matches!(self.grid[y + 1][*x], Cell::Occupied(_))
            })
        {
            self.next_piece();
        }

        self.current_tetromino.update();
    }

    fn next_piece(&mut self) {
        // set current piece
        self.current_tetromino.get_full_position()
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
    }

    fn fill_bag(&mut self) {
        self.bag = new_bag();
    }
}
impl Default for Board {
    fn default() -> Self {
        Board {
            grid: [[Cell::Empty; BOARD_SIZE.0]; BOARD_SIZE.1],
            current_tetromino: Tetromino::new(TetrominoShape::I),
            bag: new_bag(),
            bag_index: 0,
        }
    }
}
impl Shape for Board {
    fn draw(&self, painter: &mut Painter) {
        let tetromino_positions = self.current_tetromino.get_full_position();
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
    [TetrominoShape::I; 7]
    // TODO: make this random
}
