use crate::app::App;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_config(app: &mut App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Config",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));

    let mut text = String::new();
    text.push_str("Empty");

    let config = &app.get_selected_config();

    if let Some(config) = config {
        text = format!("Host {}", config.name);

        config.host_config.iter().for_each(|(key, value)| {
            text.push('\n');

            text.push_str(key.to_string().as_str());
            text.push(' ');
            text.push_str(value.as_str());
        });
    }

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}
