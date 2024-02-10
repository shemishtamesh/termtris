use crate::tetromino::TetrominoShape;
use ratatui::style::Color;

pub const fn tetromino_color(tetromino: &TetrominoShape) -> Color {
    match tetromino {
        TetrominoShape::I => Color::Rgb(0, 255, 255),
        TetrominoShape::L => Color::Rgb(255, 127, 0),
        TetrominoShape::J => Color::Rgb(0, 0, 255),
        TetrominoShape::O => Color::Rgb(255, 255, 0),
        TetrominoShape::S => Color::Rgb(0, 255, 0),
        TetrominoShape::T => Color::Rgb(128, 0, 128),
        TetrominoShape::Z => Color::Rgb(255, 0, 0),
    }
}

pub const BLOCK_SIZE: u16 = 2;
pub const BOARD_SIZE: (usize, usize) = (10, 24);

pub const BASE_TICK_RATE: u64 = 250;
