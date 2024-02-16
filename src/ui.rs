use ratatui::{
    prelude::Frame,
    symbols::Marker,
    widgets::{canvas, Block},
};

use crate::app::App;
use crate::config::BOARD_SIZE;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        canvas::Canvas::default()
            .block(Block::default())
            .x_bounds([0.0, (BOARD_SIZE.0) as f64])
            .y_bounds([0.0, (BOARD_SIZE.1) as f64])
            .marker(Marker::HalfBlock)
            .paint(|ctx| ctx.draw(&app.board)),
        ratatui::prelude::Rect::new(
            (f.size().width / 2) - (BOARD_SIZE.0 / 2) as u16,
            (f.size().height / 2) - (BOARD_SIZE.1 / 4) as u16,
            (BOARD_SIZE.0 + 2) as u16,
            ((BOARD_SIZE.1 / 2) + 2) as u16,
        ),
    );
}
