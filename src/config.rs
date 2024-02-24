use crate::tetromino::TetrominoShape;
use ratatui::style::Color;

pub const fn tetromino_color(tetromino_shape: &TetrominoShape) -> Color {
    match tetromino_shape {
        TetrominoShape::I => Color::Rgb(0, 255, 255),
        TetrominoShape::L => Color::Rgb(255, 127, 0),
        TetrominoShape::J => Color::Rgb(0, 0, 255),
        TetrominoShape::O => Color::Rgb(255, 255, 0),
        TetrominoShape::S => Color::Rgb(0, 255, 0),
        TetrominoShape::T => Color::Rgb(128, 0, 128),
        TetrominoShape::Z => Color::Rgb(255, 0, 0),
    }
}

pub const fn tetromino_color_ghost(tetromino_shape: &TetrominoShape) -> Color {
    match tetromino_shape {
        TetrominoShape::I => Color::Rgb(0, 127, 128),
        TetrominoShape::L => Color::Rgb(128, 64, 0),
        TetrominoShape::J => Color::Rgb(0, 0, 128),
        TetrominoShape::O => Color::Rgb(128, 127, 0),
        TetrominoShape::S => Color::Rgb(0, 128, 0),
        TetrominoShape::T => Color::Rgb(61, 0, 61),
        TetrominoShape::Z => Color::Rgb(128, 0, 0),
    }
}

pub const fn tetromino_color_border(tetromino_shape: &TetrominoShape) -> Color {
    match tetromino_shape {
        TetrominoShape::I => Color::Rgb(64, 191, 191),
        TetrominoShape::L => Color::Rgb(191, 127, 64),
        TetrominoShape::J => Color::Rgb(64, 64, 191),
        TetrominoShape::O => Color::Rgb(191, 191, 64),
        TetrominoShape::S => Color::Rgb(64, 191, 64),
        TetrominoShape::T => Color::Rgb(96, 32, 96),
        TetrominoShape::Z => Color::Rgb(191, 64, 64),
    }
}

pub const fn tick_delay(level: u8) -> u64 {
    match level {
        1 => 800, // 48 frames between updates assuming 60 fps in miliseconds, rounded
        2 => 717,
        3 => 633,
        4 => 550,
        5 => 467,
        6 => 383,
        7 => 300,
        8 => 217,
        9 => 133,
        10 => 100,
        11..=13 => 83,
        12..=16 => 67,
        17..=19 => 50,
        20..=29 => 33,
        _ => 1,
    }
}

pub const BLOCK_SIZE: u16 = 2;
pub const BOARD_SIZE: (usize, usize) = (10, 24);

pub const LOCK_DELAY: u8 = 3;
