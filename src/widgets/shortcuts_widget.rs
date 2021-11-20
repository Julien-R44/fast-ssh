use crate::{App, THEME};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::Style,
    text::Spans,
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::block;

pub struct ShortcutsWidget {}

impl ShortcutsWidget {
    pub fn render(_app: &App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let block = block::new(" Help ");

        let text = vec![
            Spans::from("'Enter': Validate"),
            Spans::from("'Tab': Change Group"),
            Spans::from("'c': Configuration Display Mode"),
            Spans::from("'PageUp/Down': Scroll Configuration"),
            Spans::from("'s' Search Mode"),
            Spans::from("'Esc' Exit Search Mode"),
            Spans::from("'q': Exit"),
        ];

        let paragraph = Paragraph::new(text)
            .alignment(tui::layout::Alignment::Left)
            .block(block)
            .style(Style::default().fg(THEME.text_secondary()))
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}
