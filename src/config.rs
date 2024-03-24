use crate::tetromino::TetrominoShape;
use ratatui::style::Color;
use serde::Deserialize;
// use toml::from_str;
use lazy_static::lazy_static;
use ron::from_str;
use std::collections::HashMap;

lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}

fn default_board_size() -> (usize, usize) {
    (10, 24)
}

fn default_lock_delay() -> u8 {
    3
}

fn default_tick_delay() -> HashMap<u8, u64> {
    HashMap::from([
        (1, 800),
        (2, 717),
        (3, 633),
        (4, 550),
        (5, 467),
        (6, 383),
        (7, 300),
        (8, 217),
        (9, 133),
        (10, 100),
        (13, 83),
        (16, 67),
        (19, 50),
        (29, 33),
    ])
}

fn default_tetromino_color() -> HashMap<TetrominoShape, Color> {
    HashMap::from([
        (TetrominoShape::I, Color::Rgb(0, 255, 255)),
        (TetrominoShape::L, Color::Rgb(255, 127, 0)),
        (TetrominoShape::J, Color::Rgb(0, 0, 255)),
        (TetrominoShape::O, Color::Rgb(255, 255, 0)),
        (TetrominoShape::S, Color::Rgb(0, 255, 0)),
        (TetrominoShape::T, Color::Rgb(128, 0, 128)),
        (TetrominoShape::Z, Color::Rgb(255, 0, 0)),
    ])
}

fn default_ghost_color() -> HashMap<TetrominoShape, Color> {
    HashMap::from([
        (TetrominoShape::I, Color::Rgb(0, 127, 128)),
        (TetrominoShape::L, Color::Rgb(128, 64, 0)),
        (TetrominoShape::J, Color::Rgb(0, 0, 128)),
        (TetrominoShape::O, Color::Rgb(128, 127, 0)),
        (TetrominoShape::S, Color::Rgb(0, 128, 0)),
        (TetrominoShape::T, Color::Rgb(61, 0, 61)),
        (TetrominoShape::Z, Color::Rgb(128, 0, 0)),
    ])
}

fn default_border_color() -> HashMap<TetrominoShape, Color> {
    HashMap::from([
        (TetrominoShape::I, Color::Rgb(64, 191, 191)),
        (TetrominoShape::L, Color::Rgb(191, 127, 64)),
        (TetrominoShape::J, Color::Rgb(64, 64, 191)),
        (TetrominoShape::O, Color::Rgb(191, 191, 64)),
        (TetrominoShape::S, Color::Rgb(64, 191, 64)),
        (TetrominoShape::T, Color::Rgb(96, 32, 96)),
        (TetrominoShape::Z, Color::Rgb(191, 64, 64)),
    ])
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_board_size")]
    pub board_size: (usize, usize),
    #[serde(default = "default_lock_delay")]
    pub lock_delay: u8,
    #[serde(default = "default_tick_delay")]
    pub tick_delay: HashMap<u8, u64>,
    #[serde(default = "default_tetromino_color")]
    pub tetromino_color: HashMap<TetrominoShape, Color>,
    #[serde(default = "default_ghost_color")]
    pub ghost_color: HashMap<TetrominoShape, Color>,
    #[serde(default = "default_border_color")]
    pub border_color: HashMap<TetrominoShape, Color>,
}

impl Default for Config {
    fn default() -> Self {
        // find the config file
        let config_path = match std::env::var("XDG_CONFIG_HOME") {
            Ok(path) => path.to_owned() + "/termtris/config.ron",
            Err(_) => match std::env::var("HOME") {
                Ok(path) => path + "/.config/termtris/config.ron",
                Err(_) => "./termtris/config.ron".to_string(),
            },
        };

        // read config file
        match std::fs::read_to_string(config_path) {
            Ok(config) => from_str(&config).expect("Failed to parse config"),
            Err(_) => Config {
                board_size: default_board_size(),
                lock_delay: default_lock_delay(),
                tick_delay: default_tick_delay(),
                tetromino_color: default_tetromino_color(),
                ghost_color: default_ghost_color(),
                border_color: default_border_color(),
            },
        }
    }
}
