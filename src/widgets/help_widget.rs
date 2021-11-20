use crate::app::App;
use crate::THEME;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Style,
    text::Spans,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct HelpWidget {}

impl HelpWidget {
    pub fn render(_app: &mut App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(tui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(THEME.primary_color))
            .title_alignment(tui::layout::Alignment::Center);

        let help_span = Spans::from("'h' Show help");

        let paragraph = Paragraph::new(help_span)
            .block(block)
            .style(Style::default().fg(THEME.secondary_color))
            .alignment(tui::layout::Alignment::Center);

        frame.render_widget(paragraph, area);
    }
}
