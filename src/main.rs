use std::process::Command;

use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Paragraph},
};

mod app;
mod database;
mod input_handler;
mod render_config;
mod render_group_tabs;
mod render_host_table;
mod render_shortcuts;
mod ssh_config_store;
mod term;

use app::*;
use input_handler::*;
use render_config::*;
use render_group_tabs::*;
use render_host_table::*;
use render_shortcuts::*;
use term::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = match App::new().await {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut terminal = init_terminal()?;

    app.host_state.select(Some(0));

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .horizontal_margin(4)
                .constraints([Constraint::Length(3), Constraint::Percentage(90)].as_ref())
                .split(frame.size());

            let chunk_t = Layout::default()
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
                .split(chunks[0]);

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

            let chunk_b = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .horizontal_margin(0)
                .constraints(constraints.as_ref())
                .split(chunks[1]);

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(tui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(Color::LightMagenta))
                .title_alignment(tui::layout::Alignment::Center);

            let help_span = Spans::from("'h' Show help");
            frame.render_widget(
                Paragraph::new(help_span)
                    .block(block)
                    .alignment(tui::layout::Alignment::Center),
                chunk_t[2],
            );

            render_group_tabs(&app, chunk_t[0], frame);
            render_host_table(&mut app, chunk_b[0], frame);
            render_config(&mut app, chunk_b[2], frame);

            if app.show_help {
                render_shortcuts(&app, chunk_b[4], frame);
            }
        })?;

        handle_inputs(&mut app)?;

        if app.should_quit || app.should_spawn_ssh {
            break;
        }
    }

    restore_terminal(&mut terminal)?;

    if app.should_spawn_ssh {
        let selected_config = app.get_selected_config().unwrap();
        let host_name = &selected_config.full_name;

        app.db.save_host_values(
            host_name,
            selected_config.connection_count + 1,
            chrono::offset::Local::now().timestamp(),
        )?;

        Command::new("ssh")
            .arg(host_name.split(' ').take(1).collect::<Vec<&str>>().join(""))
            .spawn()?
            .wait()?;
    }

    Ok(())
}
