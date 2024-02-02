use ratatui::{
    prelude::Frame,
    symbols::Marker,
    // style::Color,
    widgets::{canvas, Block, Borders},
};

use crate::app::App;
use crate::config::BLOCK_SIZE;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        canvas::Canvas::default()
            .block(Block::default().borders(Borders::ALL))
            .x_bounds([0.0, (BLOCK_SIZE * 10) as f64])
            .y_bounds([0.0, (BLOCK_SIZE * 20) as f64])
            .marker(Marker::HalfBlock)
            .paint(|ctx| ctx.draw(&app.board)),
        ratatui::prelude::Rect::new(
            f.size().width * 3 / 8,
            0,
            f.size().width / 4,
            f.size().height,
        ),
    );
}
