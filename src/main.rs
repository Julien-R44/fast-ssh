use std::process::Command;

use tui::{
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
};

mod app;
mod input_handler;
mod render_config;
mod render_group_tabs;
mod render_host_list;
mod ssh_config_store;
mod term;
use app::*;
use input_handler::*;
use render_config::*;
use render_group_tabs::*;
use render_host_list::*;
use ssh_config_store::*;
use term::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scs = SshConfigStore::new().await?;

    let mut terminal = init_terminal()?;

    let mut app = App {
        selected_group: 1,
        selected_host: 0,
        groups: &scs.groups,
        host_state: ListState::default(),
        should_quit: false,
        should_spawn_ssh: false,
    };

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .horizontal_margin(4)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                .split(frame.size());

            let chunk_b = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .horizontal_margin(0)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);

            render_group_tabs(&app, chunks[0], frame);
            render_host_list(&mut app, chunk_b[0], frame);
            render_config(&mut app, chunk_b[1], frame);
        })?;

        handle_inputs(&mut app)?;

        if app.should_quit || app.should_spawn_ssh {
            break;
        }
    }

    restore_terminal(&mut terminal)?;

    if app.should_spawn_ssh {
        // TODO: store full host in app
        let host = &app.groups[app.selected_group].items[app.host_state.selected().unwrap()];
        let group = &app.groups[app.selected_group];
        let full_host = format!("{}/{}", group.name, host.name);
        Command::new("ssh").arg(full_host).spawn()?.wait()?;
    }

    Ok(())
}
