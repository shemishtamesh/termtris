use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::App;
use crate::tetromino::Direction;

pub fn update(app: &mut App) {
    match app.board.update() {
        Ok(_) => {}
        Err(_) => app.quit(),
    };
}

pub fn key_event_update(app: &mut App, key_event: KeyEvent) {
    match (key_event.code, key_event.kind) {
        // exit
        (KeyCode::Esc | KeyCode::Char('q'), _) => app.quit(),
        (KeyCode::Char('c') | KeyCode::Char('C'), _) => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }

        // restart
        (KeyCode::Char('r'), KeyEventKind::Press) => app.reset(),

        // move to sides
        (KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l'), KeyEventKind::Press) => {
            app.board.move_current_piece(Direction::Right);
        }
        (KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h'), KeyEventKind::Press) => {
            app.board.move_current_piece(Direction::Left);
        }

        // rotate
        (KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k'), KeyEventKind::Press) => {
            app.board.rotate_current_piece(true);
        }
        (KeyCode::Char('z'), KeyEventKind::Press) => {
            app.board.rotate_current_piece(false);
        }

        // drop
        (KeyCode::Char(' '), KeyEventKind::Press) => match app.board.hard_drop() {
            Ok(_) => {}
            Err(_) => app.quit(),
        },
        (KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j'), _) => {
            match key_event.kind {
                KeyEventKind::Press => {
                    app.board.soft_drop(true);
                }
                KeyEventKind::Release => {
                    app.board.soft_drop(false);
                }
                _ => {}
            }
        }

        _ => {}
    };
}
