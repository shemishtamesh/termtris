use ratatui::style::Color;
use crate::game::Tetromino;

pub const fn tetromino_color(tetromino: Tetromino) -> Color {
    match tetromino {
        Tetromino::I => Color::Cyan,
        Tetromino::L => Color::Blue,
        Tetromino::J => Color::LightRed,
        Tetromino::O => Color::Yellow,
        Tetromino::S => Color::Green,
        Tetromino::T => Color::Magenta,
        Tetromino::Z => Color::Red,
    }
}
pub const BLOCK_SIZE: u16 = 2;
pub const BOARD_SIZE: (usize, usize) = (10, 20);
