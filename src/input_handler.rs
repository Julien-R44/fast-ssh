use crossterm::event::{self, Event, KeyCode};

use crate::app::{App, AppState, ConfigDisplayMode};

pub fn handle_inputs(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if let Event::Key(key) = event::read()? {
        match app.state {
            AppState::Normal => handle_input_normal_mode(app, key.code),
            AppState::Searching => handle_input_search_mode(app, key.code),
        };
    }
    Ok(())
}

fn handle_input_search_mode(app: &mut App, key: KeyCode) {}

fn handle_input_normal_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Tab => app.change_selected_group(),
        KeyCode::Down => app.change_selected_item(true),
        KeyCode::Up => app.change_selected_item(false),
        KeyCode::PageDown => app.scroll_config_paragraph(1),
        KeyCode::PageUp => app.scroll_config_paragraph(-1),

        KeyCode::Char('c') => app.toggle_config_display_mode(),
        KeyCode::Char('h') => app.show_help = !app.show_help,
        KeyCode::Char('s') => app.state = AppState::Searching,
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Enter => app.should_spawn_ssh = true,
        _ => {}
    }
}
