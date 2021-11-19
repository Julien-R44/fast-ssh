use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{app::App, ssh_config_store::SshGroupItem};

pub struct Searcher {
    search_string: String,
    has_user_input: bool,
}

impl Searcher {
    pub fn new() -> Searcher {
        Searcher {
            search_string: String::new(),
            has_user_input: false,
        }
    }

    pub fn get_filtered_items<'a>(&self, app: &'a App) -> Vec<&'a SshGroupItem> {
        app.get_all_items()
            .into_iter()
            .filter(|item| (**item).full_name.contains(&self.search_string))
            .collect::<Vec<&SshGroupItem>>()
    }

    pub fn add_char(&mut self, c: char) {
        self.search_string.push(c);
        self.has_user_input = true;
        self.search();
    }

    pub fn del_char(&mut self) {
        self.search_string.pop();
        self.search();
    }

    fn search(&self) {}

    pub fn render(&self, _app: &App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightMagenta))
            .title_alignment(tui::layout::Alignment::Center)
            .border_type(tui::widgets::BorderType::Rounded)
            .title(Span::styled(" Search ", Style::default().fg(Color::White)));

        let spans = Spans::from(vec![
            Span::styled(" > ", Style::default().fg(Color::Magenta)),
            Span::styled(
                &self.search_string,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]);

        let paragraph = Paragraph::new(spans).block(block);

        frame.render_widget(paragraph, area);
    }
}
