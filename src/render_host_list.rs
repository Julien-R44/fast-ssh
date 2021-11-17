use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::app::App;

pub fn render_host_list(app: &mut App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
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
        let cells = [
            Cell::from(item.name.to_string()).style(normal_style),
            Cell::from(item.last_used.to_string()).style(normal_style),
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
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ]);

    frame.render_stateful_widget(t, area, &mut app.host_state);
}
