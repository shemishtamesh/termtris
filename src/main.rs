pub mod app;
pub mod board;
pub mod config;
pub mod tetromino;
pub mod tui;
pub mod ui;
pub mod update;

use anyhow::Result;
use app::App;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::time::{Duration, Instant};
use tui::Tui;
use update::{key_event_update, update};

fn main() -> Result<()> {
    // Create the application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.enter()?;

    // Start the main loop.
    let mut poll_time = Instant::now(); // initialize for tick delay
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        let delay_duration = Duration::from_millis(app.board.tick_delay);
        if crossterm::event::poll(delay_duration).expect("failed to poll event") {
            match crossterm::event::read().expect("failed to read event") {
                crossterm::event::Event::FocusGained => {
                    app.pause(false);
                }
                crossterm::event::Event::FocusLost => {
                    app.pause(true);
                }
                crossterm::event::Event::Resize(_width, _height) => {
                    // TODO: change board size
                }
                crossterm::event::Event::Key(key_event) => key_event_update(&mut app, key_event),
                _ => {}
            };
        }

        // make sure enough time has passed for update
        if poll_time.elapsed() >= delay_duration {
            if !app.paused {
                update(&mut app);
            }
            poll_time = Instant::now();
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
