use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;
use crate::tetromino::Direction;

pub fn update(app: &mut App) {
    match app.board.update() {
        Ok(_) => {}
        Err(_) => app.quit(),
    };
}

pub fn key_event_update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => {
            app.board.move_current_piece(Direction::Right);
        }
        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => {
            app.board.move_current_piece(Direction::Left);
        }
        _ => {}
    };
}
