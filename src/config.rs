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

pub const fn tetromino_color_ghost(tetromino: &TetrominoShape) -> Color {
    match tetromino {
        TetrominoShape::I => Color::Rgb(0, 127, 128),
        TetrominoShape::L => Color::Rgb(128, 64, 0),
        TetrominoShape::J => Color::Rgb(0, 0, 128),
        TetrominoShape::O => Color::Rgb(128, 127, 0),
        TetrominoShape::S => Color::Rgb(0, 128, 0),
        TetrominoShape::T => Color::Rgb(61, 0, 61),
        TetrominoShape::Z => Color::Rgb(128, 0, 0),
    }
}

pub const fn tetromino_color_border(tetromino: &TetrominoShape) -> Color {
    match tetromino {
        TetrominoShape::I => Color::Rgb(64, 191, 191),
        TetrominoShape::L => Color::Rgb(191, 127, 64),
        TetrominoShape::J => Color::Rgb(64, 64, 191),
        TetrominoShape::O => Color::Rgb(191, 191, 64),
        TetrominoShape::S => Color::Rgb(64, 191, 64),
        TetrominoShape::T => Color::Rgb(96, 32, 96),
        TetrominoShape::Z => Color::Rgb(191, 64, 64),
    }
}

pub const BLOCK_SIZE: u16 = 2;
pub const BOARD_SIZE: (usize, usize) = (10, 24);

pub const TICK_DELAY: u64 = 250;
pub const SOFT_DROP_TICK_DELAY: u64 = TICK_DELAY / 4;

pub const LOCK_DELAY: u8 = 3;
