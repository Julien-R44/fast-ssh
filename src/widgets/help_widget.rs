use crate::app::App;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
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
            .border_style(Style::default().fg(Color::LightMagenta))
            .title_alignment(tui::layout::Alignment::Center);

        let help_span = Spans::from("'h' Show help");

        let paragraph = Paragraph::new(help_span)
            .block(block)
            .alignment(tui::layout::Alignment::Center);

        frame.render_widget(paragraph, area);
    }
}
