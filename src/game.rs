use crate::config::{BOARD_SIZE, tetromino_color};
use ratatui::widgets::canvas::{Painter, Shape};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tetromino {
    J,
    L,
    S,
    Z,
    O,
    T,
    I,
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Occupied(Tetromino),
}

#[derive(Debug)]
pub struct Board {
    pub grid: [[Cell; BOARD_SIZE.0]; BOARD_SIZE.1],
}
impl Board {
    pub fn new() -> Board {
        Board::default()
    }
}
impl Default for Board {
    fn default() -> Self {
        Board {
            grid: [[Cell::Empty; 10]; 20],
        }
    }
}
impl Shape for Board {
    fn draw(&self, painter: &mut Painter) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Empty => {}
                    Cell::Occupied(tetromino) => {
                        painter.paint(x, y, tetromino_color(*tetromino))
                    }
                }
            }
        }
    }
}
