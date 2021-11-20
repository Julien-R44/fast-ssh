use crate::app::App;
use crate::THEME;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend, layout::Rect, style::Style, text::Spans, widgets::Paragraph, Frame,
};

use super::block;

pub struct HelpWidget {}

impl HelpWidget {
    pub fn render(_app: &mut App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let block = block::new("");

        let help_span = Spans::from("'h' Show help");

        let paragraph = Paragraph::new(help_span)
            .block(block)
            .style(Style::default().fg(THEME.text_secondary()))
            .alignment(tui::layout::Alignment::Center);

        frame.render_widget(paragraph, area);
    }
}
