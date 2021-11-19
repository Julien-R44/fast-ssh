use super::block;
use crate::App;
use std::io::Stdout;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::Tabs;
use tui::{backend::CrosstermBackend, Frame};

pub struct GroupsWidget {}

impl GroupsWidget {
    pub fn render(app: &App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let block = block::new(" Groups ");
        let titles = app
            .scs
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
}
