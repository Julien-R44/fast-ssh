use crate::{
    app::{App, ConfigDisplayMode},
    ssh_config_store::SshGroupItem,
};
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

    let paragraph = match app.config_display_mode {
        ConfigDisplayMode::Selected => get_paragraph_for_selected_mode(app, block),
        ConfigDisplayMode::Global => get_paragraph_for_global_mode(app, block),
    };

    frame.render_widget(paragraph, area);
}

fn get_paragraph_for_global_mode<'a>(app: &'a App, block: Block<'a>) -> Paragraph<'a> {
    Paragraph::new("Hello")
        .block(block)
        .wrap(Wrap { trim: true })
}

fn get_paragraph_for_selected_mode<'a>(app: &'a App, block: Block<'a>) -> Paragraph<'a> {
    let mut spans = vec![Spans::from(Span::styled(
        "No item selected.\n",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ))];

    let config = &app.get_selected_config();

    if let Some(config) = config {
        spans = ssh_group_item_to_spans(config);
    }

    Paragraph::new(spans).block(block).wrap(Wrap { trim: true })
}

fn ssh_group_item_to_spans(config: &SshGroupItem) -> Vec<Spans> {
    let mut spans = vec![Spans::from(vec![
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

    spans
}
