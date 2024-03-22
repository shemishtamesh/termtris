use crate::tetromino::TetrominoShape;
use ratatui::style::Color;
use serde::Deserialize;
use toml::from_str;
use std::collections::HashMap;

const CONFIG: Config = ConfigBuilder::build_from_file();

struct Config {
    pub board_size: (u16, u16),
    pub lock_delay: u8,
    pub tick_delay: HashMap<u8, u64>,
    pub tetromino_color: HashMap<TetrominoShape, Color>,
    pub ghost_color: HashMap<TetrominoShape, Color>,
    pub border_color: HashMap<TetrominoShape, Color>,
}

#[derive(Deserialize)]
struct ConfigBuilder {
    board_size: Option<(u16, u16)>,
    lock_delay: Option<u8>,
    tick_delay: Option<HashMap<u8, u64>>,
    tetromino_color: Option<HashMap<TetrominoShape, Color>>,
    ghost_color: Option<HashMap<TetrominoShape, Color>>,
    border_color: Option<HashMap<TetrominoShape, Color>>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            board_size: None,
            lock_delay: None,
            tick_delay: None,
            tetromino_color: None,
            ghost_color: None,
            border_color: None,
        }
    }

    pub fn board_size(mut self, board_size: (u16, u16)) -> Self {
        self.board_size = Some(board_size);
        self
    }

    pub fn lock_delay(mut self, lock_delay: u8) -> Self {
        self.lock_delay = Some(lock_delay);
        self
    }

    pub fn tick_delay(mut self, tick_delay: HashMap<u8, u64>) -> Self {
        self.tick_delay = Some(tick_delay);
        self
    }

    pub fn tetromino_color(mut self, tetromino_color: HashMap<TetrominoShape, Color>) -> Self {
        self.tetromino_color = Some(tetromino_color);
        self
    }

    pub fn ghost_color(mut self, ghost_color: HashMap<TetrominoShape, Color>) -> Self {
        self.ghost_color = Some(ghost_color);
        self
    }

    pub fn border_color(mut self, border_color: HashMap<TetrominoShape, Color>) -> Self {
        self.border_color = Some(border_color);
        self
    }

    pub fn build_from_file() -> Config {
        let config_path = match std::env::var("XDG_CONFIG_HOME") {
            Ok(path) => {
                path + "/termtris/config.ron"
            }
            Err(_) => {
                match std::env::var("HOME") {
                    Ok(path) => {
                        path + "/.config/termtris/config.ron"
                    }
                    Err(_) => {
                        "./termtris/config.ron".to_string()
                    }
                }
            }
        };

        let config_builder = match &std::fs::read_to_string(config_path) {
            Ok(config) => {
                from_str(config).expect("Failed to parse config")
            }
            Err(_) => {
                ConfigBuilder::new()
            }
        };

        config_builder.build()
    }

    pub fn build(self) -> Config {
        Config {
            board_size: self.board_size.unwrap_or((10, 24)),
            lock_delay: self.lock_delay.unwrap_or(3),
            tick_delay: self.tick_delay.unwrap_or(HashMap::from([
                (1, 800), // 48 frames between updates assuming 60 fps in miliseconds, rounded
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
            ])),
            tetromino_color: self.tetromino_color.unwrap_or(HashMap::from([
                (TetrominoShape::I, Color::Rgb(0, 255, 255)),
                (TetrominoShape::L, Color::Rgb(255, 127, 0)),
                (TetrominoShape::J, Color::Rgb(0, 0, 255)),
                (TetrominoShape::O, Color::Rgb(255, 255, 0)),
                (TetrominoShape::S, Color::Rgb(0, 255, 0)),
                (TetrominoShape::T, Color::Rgb(128, 0, 128)),
                (TetrominoShape::Z, Color::Rgb(255, 0, 0)),
            ])),
            ghost_color: self.ghost_color.unwrap_or(HashMap::from([
                (TetrominoShape::I, Color::Rgb(0, 127, 128)),
                (TetrominoShape::L, Color::Rgb(128, 64, 0)),
                (TetrominoShape::J, Color::Rgb(0, 0, 128)),
                (TetrominoShape::O, Color::Rgb(128, 127, 0)),
                (TetrominoShape::S, Color::Rgb(0, 128, 0)),
                (TetrominoShape::T, Color::Rgb(61, 0, 61)),
                (TetrominoShape::Z, Color::Rgb(128, 0, 0)),
            ])),
            border_color: self.border_color.unwrap_or(HashMap::from([
                (TetrominoShape::I, Color::Rgb(64, 191, 191)),
                (TetrominoShape::L, Color::Rgb(191, 127, 64)),
                (TetrominoShape::J, Color::Rgb(64, 64, 191)),
                (TetrominoShape::O, Color::Rgb(191, 191, 64)),
                (TetrominoShape::S, Color::Rgb(64, 191, 64)),
                (TetrominoShape::T, Color::Rgb(96, 32, 96)),
                (TetrominoShape::Z, Color::Rgb(191, 64, 64)),
            ])),
        }
    }
}
