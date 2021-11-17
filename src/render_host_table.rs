use std::{
    io::Stdout,
    time::{Duration, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::app::App;

pub fn render_host_table(app: &mut App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightMagenta))
        .title_alignment(tui::layout::Alignment::Center)
        .border_type(tui::widgets::BorderType::Rounded)
        .title(Span::styled(" Hosts ", Style::default().fg(Color::White)));

    let normal_style = Style::default();
    let selected_style = Style::default().fg(Color::Magenta);

    let header_cells = ["Host", "Last Used", "Nb Connection"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));

    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let rows = app.get_selected_group().items.iter().map(|item| {
        let mut timestamp_str = "Never".to_string();
        if item.last_used != 0 {
            let d = UNIX_EPOCH + Duration::from_secs(item.last_used as u64);
            let dt = DateTime::<Utc>::from(d);
            timestamp_str = dt.format("%D %R").to_string();
        }

        let cells = [
            Cell::from(item.name.to_string()).style(normal_style),
            Cell::from(timestamp_str).style(normal_style),
            Cell::from(item.connection_count.to_string()).style(normal_style),
        ];

        Row::new(cells).height(1).bottom_margin(1)
    });

    let t = Table::new(rows)
        .header(header)
        .block(block)
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ]);

    frame.render_stateful_widget(t, area, &mut app.host_state);
}
