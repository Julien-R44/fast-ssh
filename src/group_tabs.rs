use std::io::Stdout;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Tabs};

use tui::{backend::CrosstermBackend, Frame};

use crate::App;

pub fn render_group_tabs(app: &App, chunks: &[Rect], frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let block = Block::default().title("Groups").borders(Borders::ALL);

    let titles = app
        .groups
        .iter()
        .map(|t| {
            Spans::from(Span::styled(
                t.name.to_string(),
                Style::default().fg(Color::Yellow),
            ))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(block)
        .select(app.selected_group)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );

    frame.render_widget(tabs, chunks[0]);
}
