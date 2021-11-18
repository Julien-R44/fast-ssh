use std::process::Command;

use layout::create_layout;
use tui::layout::{Constraint, Direction, Layout};

mod app;
mod database;
mod input_handler;
mod layout;
mod render_group_tabs;
mod render_shortcuts;
mod searcher;
mod ssh_config_store;
mod term;
mod widgets;

use app::*;
use input_handler::*;
use render_group_tabs::*;
use render_shortcuts::*;
use searcher::*;
use term::*;
use widgets::{config_widget::ConfigWidget, help_widget::HelpWidget, hosts_widget::HostsWidget};

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
            let layout = create_layout(&app, frame);

            match app.state {
                AppState::Normal => render_group_tabs(&app, layout.chunks_top[0], frame),
                AppState::Searching => app.searcher.render(&app, layout.chunks_top[0], frame),
            };

            HelpWidget::render(&mut app, layout.chunks_top[2], frame);
            HostsWidget::render(&mut app, layout.chunks_bot[0], frame);
            ConfigWidget::render(&mut app, layout.chunks_bot[2], frame);

            if app.show_help {
                render_shortcuts(&app, layout.chunks_bot[4], frame);
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
