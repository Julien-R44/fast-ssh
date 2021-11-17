use tui::widgets::ListState;

use crate::ssh_config_store::{SshGroup, SshGroupItem};

pub enum ConfigDisplayMode {
    Global,
    Selected,
}

pub struct App<'a> {
    pub selected_group: usize,
    pub host_state: ListState,
    pub groups: &'a [SshGroup],
    pub config_display_mode: ConfigDisplayMode,
    pub should_quit: bool,
    pub should_spawn_ssh: bool,
    pub config_paragraph_offset: u16,
}

impl App<'_> {
    pub fn get_selected_group(&self) -> &SshGroup {
        &self.groups[self.selected_group]
    }

    pub fn get_selected_config(&self) -> Option<&SshGroupItem> {
        if let Some(host_state) = self.host_state.selected() {
            let items_len = self.get_selected_group().items.len();
            if host_state < items_len {
                Some(&self.get_selected_group().items[host_state])
            } else {
                None
            }
        } else {
            None
        }
    }
}
