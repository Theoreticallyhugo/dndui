use ratatui::{
    prelude::{Layout, Direction, Rect},
    layout::{Alignment, Constraint},
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

use crate::app::App;
use crate::app::InputMode;

pub fn help_screen(frame: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    frame.render_widget(
        Paragraph::new("help_screen")
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .left_aligned(),
        layout[0],
    );

    // TODO: r for short rest, R for long. highlight respective word and press again to lock in
    frame.render_widget(
        Paragraph::new("short rest   long rest   edit ")
        .style(Style::default().fg(Color::Cyan).bg(Color::Reset))
        .right_aligned(),
        layout[1],
    );
}
