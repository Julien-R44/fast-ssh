use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::App;

pub struct AppLayout {
    pub chunks_top: Vec<Rect>,
    pub chunks_bot: Vec<Rect>,
}

pub fn create_layout(app: &App, frame: &mut Frame<CrosstermBackend<Stdout>>) -> AppLayout {
    let base_chunk = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .horizontal_margin(4)
        .constraints([Constraint::Length(3), Constraint::Percentage(90)].as_ref())
        .split(frame.size());

    let chunks_top = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Length(2),
                Constraint::Length(10),
            ]
            .as_ref(),
        )
        .split(base_chunk[0]);

    let constraints = match app.show_help {
        false => {
            vec![
                Constraint::Percentage(50),
                Constraint::Length(2),
                Constraint::Percentage(50),
            ]
        }
        true => {
            vec![
                Constraint::Percentage(40),
                Constraint::Length(2),
                Constraint::Percentage(30),
                Constraint::Length(2),
                Constraint::Percentage(30),
            ]
        }
    };

    let chunks_bot = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .horizontal_margin(0)
        .constraints(constraints.as_slice())
        .split(base_chunk[1]);

    AppLayout {
        chunks_bot,
        chunks_top,
    }
}
