use crossterm::event::{self, Event, KeyCode};

use crate::app::{App, ConfigDisplayMode};

pub fn handle_inputs(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Tab => {
                app.selected_group = (app.selected_group + 1) % app.groups.len();
                app.selected_host = 0;
            }
            KeyCode::Down => {
                let items = &app.groups[app.selected_group].items;
                let i = match app.host_state.selected() {
                    Some(i) => {
                        if i >= items.len() - 1 {
                            0
                        } else {
                            i + 1
                        }
                    }
                    None => 0,
                };
                app.host_state.select(Some(i));
            }
            KeyCode::Up => {
                let items = &app.groups[app.selected_group].items;
                let i = match app.host_state.selected() {
                    Some(i) => {
                        if i == 0 {
                            items.len() - 1
                        } else {
                            i - 1
                        }
                    }
                    None => 0,
                };
                app.host_state.select(Some(i));
            }
            KeyCode::Char('c') => {
                app.config_display_mode = match app.config_display_mode {
                    ConfigDisplayMode::Global => ConfigDisplayMode::Selected,
                    ConfigDisplayMode::Selected => ConfigDisplayMode::Global,
                };
            }
            KeyCode::Char('q') => app.should_quit = true,
            KeyCode::Enter => app.should_spawn_ssh = true,
            _ => {}
        }
    }
    Ok(())
}
