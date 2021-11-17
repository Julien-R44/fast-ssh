use crate::app::App;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_config(app: &mut App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::LightMagenta))
        .title_alignment(tui::layout::Alignment::Center)
        .title(Span::styled(
            "Configuration",
            Style::default().fg(Color::White),
        ));

    let mut spans = vec![Spans::from(Span::styled(
        "No item selected.\n",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ))];

    let config = &app.get_selected_config();

    if let Some(config) = config {
        spans = vec![Spans::from(vec![
            Span::styled("Host ", Style::default().fg(Color::Magenta)),
            Span::styled(&config.full_name, Style::default().fg(Color::White)),
        ])];

        config.host_config.iter().for_each(|(key, value)| {
            spans.push(Spans::from(vec![
                Span::styled(key.to_string(), Style::default().fg(Color::Magenta)),
                Span::styled(" ", Style::default().fg(Color::White)),
                Span::styled(value, Style::default().fg(Color::White)),
            ]));
        });
    }

    let paragraph = Paragraph::new(spans).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}
