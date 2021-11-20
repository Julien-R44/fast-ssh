use crossterm::event::{self, Event, KeyCode};

use crate::app::{App, AppState};

pub fn handle_inputs(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if let Event::Key(key) = event::read()? {
        match app.state {
            AppState::Normal => handle_input_normal_mode(app, key.code),
            AppState::Searching => handle_input_search_mode(app, key.code),
        };

        match key.code {
            KeyCode::Tab => app.change_selected_group(true),
            KeyCode::Left => app.change_selected_group(false),
            KeyCode::Right => app.change_selected_group(true),
            KeyCode::Down => app.change_selected_item(true),
            KeyCode::Up => app.change_selected_item(false),
            KeyCode::PageDown => app.scroll_config_paragraph(1),
            KeyCode::PageUp => app.scroll_config_paragraph(-1),
            KeyCode::Enter => app.should_spawn_ssh = true,
            _ => {}
        };
    }
    Ok(())
}

fn handle_input_search_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => app.state = AppState::Normal,
        KeyCode::Char(c) => app.searcher.add_char(c),
        KeyCode::Backspace => app.searcher.del_char(),
        _ => {}
    }
}

fn handle_input_normal_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('c') => app.toggle_config_display_mode(),
        KeyCode::Char('h') => app.show_help = !app.show_help,
        KeyCode::Char('s') => app.state = AppState::Searching,
        KeyCode::Char('q') => app.should_quit = true,
        _ => {}
    }
}
