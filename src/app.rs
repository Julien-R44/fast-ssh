use tui::widgets::ListState;

use crate::ssh_config_store::SshGroup;

pub struct App<'a> {
    pub selected_group: usize,
    pub selected_host: usize,
    pub host_state: ListState,
    pub groups: &'a [SshGroup],
    pub should_quit: bool,
    pub should_spawn_ssh: bool,
}
