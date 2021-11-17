use std::io::Stdout;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Tabs};

use tui::{backend::CrosstermBackend, Frame};

use crate::App;

pub fn render_group_tabs(app: &App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let block = Block::default()
        .title(Span::styled(" Groups ", Style::default().fg(Color::White)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightMagenta))
        .title_alignment(tui::layout::Alignment::Center)
        .border_type(tui::widgets::BorderType::Rounded);

    let titles = app
        .groups
        .iter()
        .map(|t| {
            Spans::from(Span::styled(
                t.name.to_string(),
                Style::default().fg(Color::White),
            ))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(block)
        .select(app.selected_group)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Magenta),
        );

    frame.render_widget(tabs, area);
}
