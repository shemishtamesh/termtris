use crate::board::Board;

/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// game state
    pub board: Board,
    /// is the game paused
    pub paused: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new instance of [`Board`].
    pub fn reset(&mut self) {
        self.board = Board::new();
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn pause(&mut self, paused: bool) {
        self.paused = paused;
    }
}
