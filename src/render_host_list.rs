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
    let block = Block::default().title("Hosts").borders(Borders::ALL);
    frame.render_widget(block, area);

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
        .block(Block::default().borders(Borders::ALL).title("Hosts"))
        .highlight_style(
            Style::default()
                .bg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(items, area, &mut app.host_state);
}
