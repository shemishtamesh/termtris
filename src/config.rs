use crate::tetromino::TetrominoShape;
use ratatui::style::Color;

pub const fn tetromino_color(tetromino: &TetrominoShape) -> Color {
    match tetromino {
        TetrominoShape::I => Color::Cyan,
        TetrominoShape::L => Color::Blue,
        TetrominoShape::J => Color::LightRed,
        TetrominoShape::O => Color::Yellow,
        TetrominoShape::S => Color::Green,
        TetrominoShape::T => Color::Magenta,
        TetrominoShape::Z => Color::Red,
    }
}
pub const BLOCK_SIZE: u16 = 2;
pub const BOARD_SIZE: (usize, usize) = (10, 24);
