use crate::{
    app::{App, ConfigDisplayMode as ConfigMode},
    ssh_config_store::SshGroupItem,
};
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

use super::block;

pub struct ConfigWidget {}

impl ConfigWidget {
    pub fn render(app: &mut App, area: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let block = block::new(" Configuration ");

        let paragraph = match app.config_display_mode {
            ConfigMode::Selected => ConfigWidget::get_paragraph_for_selected_mode(app, block),
            ConfigMode::Global => ConfigWidget::get_paragraph_for_global_mode(app, block),
        };

        frame.render_widget(paragraph.scroll((app.config_paragraph_offset, 0)), area);
    }

    fn get_paragraph_for_global_mode<'a>(app: &'a App, block: Block<'a>) -> Paragraph<'a> {
        let spans: Vec<Spans> = app.scs.groups.iter().fold(vec![], |mut acc, group| {
            let new_spans: Vec<Spans> = group
                .items
                .iter()
                .map(|item| ConfigWidget::ssh_group_item_to_spans(item))
                .flatten()
                .collect();

            acc.extend(new_spans);
            acc
        });

        Paragraph::new(spans)
            .block(block)
            .wrap(Wrap { trim: false })
    }

    fn get_paragraph_for_selected_mode<'a>(app: &'a App, block: Block<'a>) -> Paragraph<'a> {
        let mut spans = vec![Spans::from(Span::styled(
            "No item selected.\n",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))];

        let config = &app.get_selected_config();

        if let Some(config) = config {
            spans = ConfigWidget::ssh_group_item_to_spans(config);
        }

        Paragraph::new(spans)
            .block(block)
            .wrap(Wrap { trim: false })
    }

    fn ssh_group_item_to_spans(config: &SshGroupItem) -> Vec<Spans> {
        let mut spans = vec![Spans::from(vec![
            Span::styled("Host ", Style::default().fg(Color::Magenta)),
            Span::styled(&config.full_name, Style::default().fg(Color::White)),
        ])];

        config.host_config.iter().for_each(|(key, value)| {
            spans.push(Spans::from(vec![
                Span::styled("  ", Style::default().fg(Color::Magenta)),
                Span::styled(key.to_string(), Style::default().fg(Color::Magenta)),
                Span::styled(" ", Style::default().fg(Color::White)),
                Span::styled(value, Style::default().fg(Color::White)),
            ]));
        });

        spans.push(Spans::from(vec![Span::styled(
            "\n",
            Style::default().fg(Color::White),
        )]));

        spans
    }
}
