use std::fs;
use tui::widgets::TableState;

use crate::{
    database::FileDatabase,
    ssh_config_store::{SshConfigStore, SshGroup, SshGroupItem},
};

pub enum ConfigDisplayMode {
    Global,
    Selected,
}

pub struct App {
    pub selected_group: usize,
    pub host_state: TableState,
    pub scs: SshConfigStore,
    pub config_display_mode: ConfigDisplayMode,
    pub should_quit: bool,
    pub should_spawn_ssh: bool,
    pub config_paragraph_offset: u16,
    pub db: FileDatabase,
}

impl App {
    pub async fn new() -> Result<App, Box<dyn std::error::Error>> {
        let db = App::create_or_get_db_file()?;
        let scs = SshConfigStore::new(&db).await?;

        Ok(App {
            selected_group: 0,
            config_paragraph_offset: 0,
            scs,
            host_state: TableState::default(),
            should_quit: false,
            should_spawn_ssh: false,
            config_display_mode: ConfigDisplayMode::Selected,
            db,
        })
    }

    pub fn create_or_get_db_file() -> Result<FileDatabase, Box<dyn std::error::Error>> {
        if let Some(home) = dirs::home_dir() {
            let conf_path = home.join(".fast-ssh");
            let db_path = conf_path.join("db.ron");

            fs::create_dir_all(&conf_path)?;
            return FileDatabase::new(db_path.to_str().unwrap());
        }

        Err("Could not find home directory".into())
    }

    pub fn get_selected_group(&self) -> &SshGroup {
        &self.scs.groups[self.selected_group]
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
