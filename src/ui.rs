use ratatui::{
    prelude::{Alignment, Frame},
    symbols::Marker,
    widgets::{canvas, Block, BorderType, Borders, Paragraph},
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

    // score & level
    let score_text = "score: ";
    let level_text = "level: ";
    let lines_cleared_text = "lines cleared: ";
    let block_len = (num_of_digits(app.board.get_score()) + score_text.len() as u16)
        .max(num_of_digits(app.board.get_lines_cleared()) + lines_cleared_text.len() as u16)
        // calculating level text length is not needed since it's always lower than lines cleared
        .max(16);
    f.render_widget(
        // Paragraph::new(format!("score: {}", app.board.get_score())).block(
        Paragraph::new(format!(
            "{}{}\n{}{}\n{}{}",
            score_text,
            app.board.get_score(),
            level_text,
            app.board.get_level(),
            lines_cleared_text,
            app.board.get_lines_cleared()
        ))
        .block(
            Block::default()
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center),
        ratatui::prelude::Rect::new(
            (f.size().width / 2) - (BOARD_SIZE.0 / 2) as u16 - block_len - 2,
            (f.size().height / 2) - (BOARD_SIZE.1 / 4) as u16 + 8,
            block_len + 2,
            5,
        ),
    );
}

fn num_of_digits(num: u128) -> u16 {
    num.checked_ilog10().unwrap_or(0) as u16 + 1
}
