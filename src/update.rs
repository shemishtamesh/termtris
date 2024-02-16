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
        // exit
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }

        // restart
        KeyCode::Char('r') => app.reset(),

        // move to sides
        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => {
            app.board.move_current_piece(Direction::Right);
        }
        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => {
            app.board.move_current_piece(Direction::Left);
        }

        // rotate
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => {
            app.board.rotate_current_piece(true);
        }
        KeyCode::Char('z') => {
            app.board.rotate_current_piece(false);
        }

        // drop
        KeyCode::Char(' ') => match app.board.hard_drop() {
            Ok(_) => {}
            Err(_) => app.quit(),
        },

        _ => {}
    };
}
