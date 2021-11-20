use crate::{app::App, ssh_config_store::SshGroupItem, widgets::block, THEME};
use std::io::Stdout;
use sublime_fuzzy::best_match;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::Paragraph,
    Frame,
};

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
        if self.search_string.is_empty() {
            return app.get_all_items();
        }

        app.get_all_items()
            .into_iter()
            .filter(|item| best_match(&self.search_string, &item.full_name).is_some())
            .collect::<Vec<&SshGroupItem>>()
    }

    pub fn add_char(&mut self, c: char) {
        self.search_string.push(c);
        self.has_user_input = true;
    }

    pub fn del_char(&mut self) {
        self.search_string.pop();
    }

    pub fn render(&self, _app: &App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let block = block::new(" Search ");

        let spans = Spans::from(vec![
            Span::styled(" > ", Style::default().fg(THEME.text_primary())),
            Span::styled(
                &self.search_string,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]);

        let paragraph = Paragraph::new(spans).block(block);

        frame.render_widget(paragraph, area);
    }
}
