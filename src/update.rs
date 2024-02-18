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
    match key_event {
        // exit
        KeyEvent {
            code: KeyCode::Esc | KeyCode::Char('q'),
            ..
        }
        | KeyEvent {
            code: KeyCode::Char('c') | KeyCode::Char('C'),
            modifiers: KeyModifiers::CONTROL,
            ..
        } => app.quit(),

        // restart
        KeyEvent {
            code: KeyCode::Char('r'),
            kind: KeyEventKind::Press,
            ..
        } => app.reset(),

        // hold
        KeyEvent {
            code: KeyCode::Char('c'),
            kind: KeyEventKind::Press,
            ..
        } => match app.board.hold() {
            Ok(_) => {}
            Err(_) => app.quit(),
        },

        // move to sides
        KeyEvent {
            code: KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l'),
            kind: KeyEventKind::Press,
            ..
        } => app.board.move_current_piece(Direction::Right),
        KeyEvent {
            code: KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h'),
            kind: KeyEventKind::Press,
            ..
        } => app.board.move_current_piece(Direction::Left),

        // rotate
        KeyEvent {
            code: KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k'),
            kind: KeyEventKind::Press,
            ..
        } => app.board.rotate_current_piece(true),
        KeyEvent {
            code: KeyCode::Char('z'),
            kind: KeyEventKind::Press,
            ..
        } => app.board.rotate_current_piece(false),

        // drop
        KeyEvent {
            code: KeyCode::Char(' '),
            kind: KeyEventKind::Press,
            ..
        } => match app.board.hard_drop() {
            Ok(_) => {}
            Err(_) => app.quit(),
        },
        KeyEvent {
            code: KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j'),
            kind: KeyEventKind::Press,
            ..
        } => app.board.soft_drop(true),
        KeyEvent {
            code: KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j'),
            kind: KeyEventKind::Release,
            ..
        } => app.board.soft_drop(false),

        _ => {}
    };
}
