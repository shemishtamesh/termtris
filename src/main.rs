pub mod app;
pub mod board;
pub mod config;
pub mod event;
pub mod tetromino;
pub mod tui;
pub mod ui;
pub mod update;

use anyhow::Result;
use app::App;
use config::BASE_TICK_RATE;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::{key_event_update, update};

fn main() -> Result<()> {
    // Create the application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(BASE_TICK_RATE);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => update(&mut app),
            Event::Key(key_event) => key_event_update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
