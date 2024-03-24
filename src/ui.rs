use ratatui::{
    prelude::{Alignment, Frame},
    symbols::Marker,
    widgets::{canvas, Block, BorderType, Borders, Paragraph},
};

use crate::{app::App, config::CONFIG, tetromino::Tetromino};

pub fn render(app: &mut App, frame: &mut Frame) {
    // paused status
    if app.paused {
        let paused_message = "\
            exit: ESC, q, CONTROL + c\n\
            restart: r,\n\
            hold: c,\n\
            move right: d, l, right arrow\n\
            move left: a, h, left arrow\n\
            rotate clockwise: w, k, up arrow\n\
            rotate counter-clockwise: z, j, down arrow\n\
            hard drop: space\n\
            soft drop: s, j, down arrow\n\
            pause: p\
        ";
        let message_width = paused_message.lines().map(|line| line.len()).max().unwrap() as u16;
        let message_height = paused_message.lines().count() as u16;
        frame.render_widget(
            Paragraph::new(paused_message)
                .block(
                    Block::default()
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title("paused"),
                )
                .alignment(Alignment::Left),
            ratatui::prelude::Rect::new(
                (frame.size().width / 2) - message_width / 2 - 1,
                (frame.size().height / 2) - message_height / 2 - 1,
                message_width + 2,
                message_height + 2,
            ),
        );
        return;
    }

    // board
    frame.render_widget(
        canvas::Canvas::default()
            .block(Block::default())
            .x_bounds([0.0, (CONFIG.board_size.0) as f64])
            .y_bounds([0.0, (CONFIG.board_size.1) as f64])
            .marker(Marker::HalfBlock)
            .paint(|ctx| ctx.draw(&app.board)),
        ratatui::prelude::Rect::new(
            (frame.size().width / 2) - (CONFIG.board_size.0 / 2) as u16,
            (frame.size().height / 2) - (CONFIG.board_size.1 / 4) as u16,
            (CONFIG.board_size.0 + 2) as u16,
            ((CONFIG.board_size.1 / 2) + 2) as u16,
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
    frame.render_widget(
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
                .border_type(BorderType::Rounded)
                .title("stats"),
        )
        .alignment(Alignment::Center),
        ratatui::prelude::Rect::new(
            (frame.size().width / 2) - (CONFIG.board_size.0 / 2) as u16 - block_len - 2,
            (frame.size().height / 2) - (CONFIG.board_size.1 / 4) as u16 + 8,
            block_len + 2,
            5,
        ),
    );

    // previews
    let bounds = [0.0, 4.0];
    let marker = Marker::HalfBlock;
    let borders = Borders::ALL;
    let border_type = BorderType::Rounded;

    // next
    let piece_preview = canvas::Canvas::default()
        .x_bounds(bounds)
        .y_bounds(bounds)
        .marker(marker);
    let next_preview_base_x = 2 + (frame.size().width / 2) + (CONFIG.board_size.0 / 2) as u16;
    let next_preview_base_y = 1 + (frame.size().height / 2) - (CONFIG.board_size.1 / 4) as u16;
    for i in 1..5 {
        let tetromino_to_preview = Tetromino::new(app.board.calc_next_piece(i));
        frame.render_widget(
            piece_preview
                .clone()
                .block(
                    Block::default()
                        .borders(borders)
                        .border_type(border_type)
                        .title(match i {
                                    1 => {format!("{}st", i)}
                                    2 => {format!("{}nd", i)}
                                    3 => {format!("{}rd", i)}
                                    _ => {format!("{}th", i)}
                                })
                        )
                .paint(move |ctx| ctx.draw(&tetromino_to_preview)),
            ratatui::prelude::Rect::new(
                next_preview_base_x,
                next_preview_base_y + (i - 1) as u16 * 3,
                6,
                3,
            ),
        );
    }

    // hold
    // TODO: find a way not to copy paste the definition above
    let piece_preview = canvas::Canvas::default()
        .block(
            Block::default()
                .borders(borders)
                .border_type(border_type)
                .title("hold"),
        )
        .x_bounds(bounds)
        .y_bounds(bounds)
        .marker(marker);
    match app.board.get_held_tetromino() {
        None => {}
        Some(tetromino_shape) => {
            let tetromino_to_preview = Tetromino::new(tetromino_shape);
            frame.render_widget(
                piece_preview
                    .clone()
                    .paint(|ctx| ctx.draw(&tetromino_to_preview)),
                // .paint(move |ctx: canvas::Context| ctx.draw()),
                ratatui::prelude::Rect::new(
                    (frame.size().width / 2) - 1 - (CONFIG.board_size.0) as u16,
                    1 + (frame.size().height / 2) - (CONFIG.board_size.1 / 4) as u16,
                    6,
                    3,
                ),
            )
        }
    };
}

fn num_of_digits(num: u128) -> u16 {
    num.checked_ilog10().unwrap_or(0) as u16 + 1
}
