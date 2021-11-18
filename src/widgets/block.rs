use tui::{
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders},
};

pub fn new(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightMagenta))
        .title_alignment(tui::layout::Alignment::Center)
        .border_type(tui::widgets::BorderType::Rounded)
        .title(Span::styled(title, Style::default().fg(Color::White)))
}
