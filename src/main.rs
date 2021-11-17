use std::process::Command;

use tui::{
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
};

mod app;
mod group_tabs;
mod host_list;
mod input_handler;
mod ssh_config_store;
mod term;
use app::*;
use group_tabs::*;
use host_list::*;
use input_handler::*;
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

            render_group_tabs(&app, &chunks, frame);
            render_host_list(&mut app, &chunks, frame);
        })?;

        handle_inputs(&mut app)?;

        if app.should_quit || app.should_spawn_ssh {
            break;
        }
    }

    restore_terminal(&mut terminal)?;

    // TODO: store full host in app
    let host = &app.groups[app.selected_group].items[app.host_state.selected().unwrap()];
    let group = &app.groups[app.selected_group];
    let full_host = format!("{}/{}", group.name, host.name);

    Command::new("ssh").arg(full_host).spawn()?.wait()?;

    Ok(())
}
