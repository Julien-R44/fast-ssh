use tui::widgets::ListState;

use crate::ssh_config_store::{SshGroup, SshGroupItem};

pub struct App<'a> {
    pub selected_group: usize,
    pub selected_host: usize,
    pub host_state: ListState,
    pub groups: &'a [SshGroup],
    pub should_quit: bool,
    pub should_spawn_ssh: bool,
}

impl App<'_> {
    pub fn get_selected_group(&self) -> &SshGroup {
        &self.groups[self.selected_group]
    }

    pub fn get_selected_config(&self) -> Option<&SshGroupItem> {
        if let Some(host_state) = self.host_state.selected() {
            Some(&self.get_selected_group().items[host_state])
        } else {
            None
        }
    }
}
