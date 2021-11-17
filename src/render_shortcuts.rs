use crate::App;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_shortcuts(_app: &App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(tui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(Color::LightMagenta))
        .title_alignment(tui::layout::Alignment::Center)
        .title(Span::styled("Shortcuts", Style::default().fg(Color::White)));

    let text = vec![
        Spans::from("'c' to alternate between configuration display mode"),
        Spans::from("'g' to alternate between groups mode"),
        Spans::from("'Enter' to SSH to your server"),
    ];

    let paragraph = Paragraph::new(text)
        .alignment(tui::layout::Alignment::Left)
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area)
}
