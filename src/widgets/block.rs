use crate::THEME;
use tui::{
    style::Style,
    text::Span,
    widgets::{Block, Borders},
};

pub fn new(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(THEME.primary_color))
        .title_alignment(tui::layout::Alignment::Center)
        .border_type(tui::widgets::BorderType::Rounded)
        .title(Span::styled(
            title,
            Style::default().fg(THEME.secondary_color),
        ))
}
