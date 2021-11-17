use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

pub fn render_host_list(app: &mut App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightMagenta))
        .title_alignment(tui::layout::Alignment::Center)
        .border_type(tui::widgets::BorderType::Rounded)
        .title(Span::styled("Hosts", Style::default().fg(Color::White)));

    let items: Vec<ListItem> = app.groups[app.selected_group]
        .items
        .iter()
        .map(|i| {
            ListItem::new(Spans::from(Span::styled(
                i.name.to_string(),
                Style::default(),
            )))
            .style(Style::default().fg(Color::White))
        })
        .collect();

    let items = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(items, area, &mut app.host_state);
}
