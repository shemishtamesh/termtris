use ratatui::{
    prelude::{Alignment, Frame, Rect},
    symbols::Marker,
    widgets::{canvas, Block, BorderType, Borders, Paragraph},
};

use crate::{app::App, config::CONFIG, tetromino::Tetromino};

pub fn render(app: &App, frame: &mut Frame) {
    let bounds = [0.0, 4.0];
    let marker = Marker::HalfBlock;
    let borders = Borders::ALL;
    let border_type = BorderType::Rounded;

    if app.paused {
        render_pause(frame);
        return;
    }
    render_board(app, frame);
    render_stats(app, frame);
    render_next_previews(app, frame, bounds, marker, borders, border_type);
    render_hold(app, frame, bounds, marker, borders, border_type);
}

fn render_pause(frame: &mut Frame) {
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
}

fn render_board(app: &App, frame: &mut Frame) {
    frame.render_widget(
        canvas::Canvas::default()
            .block(Block::default())
            .x_bounds([0.0, (CONFIG.board_size.0) as f64])
            .y_bounds([0.0, (CONFIG.board_size.1) as f64])
            .marker(Marker::HalfBlock)
            .paint(|ctx| ctx.draw(&app.board)),
        ratatui::prelude::Rect::new(
            (frame.size().width / 2) - (CONFIG.board_size.0 / 2 + 1) as u16,
            (frame.size().height / 2) - (CONFIG.board_size.1 / 4 + 1) as u16,
            (CONFIG.board_size.0 + 2) as u16,
            ((CONFIG.board_size.1 / 2) + 2) as u16,
        ),
    );
}

fn render_stats(app: &App, frame: &mut Frame) {
    let score_text = "score: ";
    let level_text = "level: ";
    let lines_cleared_text = "lines cleared: ";
    let block_width = (num_of_digits(app.board.get_score()) + score_text.len() as u16)
        .max(num_of_digits(app.board.get_lines_cleared()) + lines_cleared_text.len() as u16)
        // calculating level text length is not needed since it's always lower than lines cleared
        .max(16)
        + 2; // + 2 for the border
    let block_height = 5;
    frame.render_widget(
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
            (frame.size().width / 2) - (CONFIG.board_size.0 / 2) as u16 - block_width - 1,
            (frame.size().height / 2) + (CONFIG.board_size.1 / 4) as u16 - block_height
                + (((CONFIG.board_size.1 as u16 + 2) % 4) + 1) % 3, // keeps as close to bottom of board without passing it
            block_width,
            block_height,
        ),
    );
}

fn render_next_previews(
    app: &App,
    frame: &mut Frame,
    bounds: [f64; 2],
    marker: Marker,
    borders: Borders,
    border_type: BorderType,
) {
    // next pieces
    let piece_preview = canvas::Canvas::default()
        .x_bounds(bounds)
        .y_bounds(bounds)
        .marker(marker);
    let block_width = 6;
    let block_height = 3;
    let preview_positions = calculate_next_preview_positions(
        CONFIG.number_of_previews,
        frame.size(),
        block_width,
        block_height,
    );
    for (i, position) in preview_positions.iter().enumerate() {
        let i = i + 1; // show the first piece ahead, not current one
        let tetromino_to_preview = Tetromino::new(app.board.calc_next_piece(i));
        frame.render_widget(
            piece_preview
                .clone()
                .block(
                    Block::default()
                        .borders(borders)
                        .border_type(border_type)
                        .title(match i {
                            1 => {
                                format!("{}st", i)
                            }
                            2 => {
                                format!("{}nd", i)
                            }
                            3 => {
                                format!("{}rd", i)
                            }
                            _ => {
                                format!("{}th", i)
                            }
                        }),
                )
                .paint(move |ctx| ctx.draw(&tetromino_to_preview)),
            ratatui::prelude::Rect::new(position.0, position.1, block_width, block_height),
        );
    }
}

fn calculate_next_preview_positions(
    num_of_previews: usize,
    frame_size: Rect,
    block_width: u16,
    block_height: u16,
) -> Vec<(u16, u16)> {
    let next_preview_base_x = 1 + (frame_size.width / 2) + (CONFIG.board_size.0 / 2) as u16;
    let next_preview_base_y = (frame_size.height / 2) - (CONFIG.board_size.1 / 4) as u16;
    let drawable_screen_height = frame_size.height - block_height;
    let drawable_screen_after_base = drawable_screen_height - next_preview_base_y;
    (1..num_of_previews + 1)
        .map(|i| {
            let spaces_for_previus = (i - 1) as u16 * block_height;
            let y_before_wrap = next_preview_base_y + spaces_for_previus;
            let how_many_wraps = spaces_for_previus / drawable_screen_after_base;
            let y_before_wrap_adjustment =
                (y_before_wrap + how_many_wraps * next_preview_base_y) % (drawable_screen_height);
            (
                next_preview_base_x + block_width * how_many_wraps,
                y_before_wrap_adjustment
                    - ((y_before_wrap_adjustment - next_preview_base_y) % block_height),
            )
        })
        .collect()
}

fn render_hold(
    app: &App,
    frame: &mut Frame,
    bounds: [f64; 2],
    marker: Marker,
    borders: Borders,
    border_type: BorderType,
) {
    let block_width = 6;
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
                ratatui::prelude::Rect::new(
                    (frame.size().width / 2) - (CONFIG.board_size.0 / 2) as u16 - block_width - 1,
                    (frame.size().height / 2) - (CONFIG.board_size.1 / 4) as u16,
                    block_width,
                    3,
                ),
            )
        }
    };
}

fn num_of_digits(num: u128) -> u16 {
    num.checked_ilog10().unwrap_or(0) as u16 + 1
}

mod tests {
    #[test]
    fn test_calculate_next_preview_positions() {
        use super::{calculate_next_preview_positions, Rect};
        calculate_next_preview_positions(7, Rect::new(0, 0, 40, 40), 4, 4);
    }
}
