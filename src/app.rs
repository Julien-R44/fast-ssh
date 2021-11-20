use anyhow::{format_err, Context, Result};
use std::fs;
use tui::widgets::TableState;

use crate::{
    config::{resolve_config, Config},
    database::FileDatabase,
    searcher::Searcher,
    ssh_config_store::{SshConfigStore, SshGroup, SshGroupItem},
};

pub enum ConfigDisplayMode {
    Global,
    Selected,
}

pub enum AppState {
    Searching,
    Normal,
}

pub struct App {
    pub state: AppState,
    pub searcher: Searcher,
    pub selected_group: usize,
    pub host_state: TableState,
    pub scs: SshConfigStore,
    pub config_display_mode: ConfigDisplayMode,
    pub should_quit: bool,
    pub should_spawn_ssh: bool,
    pub config_paragraph_offset: u16,
    pub db: FileDatabase,
    pub show_help: bool,
    pub config: Config,
}

impl App {
    pub async fn new() -> Result<App> {
        let db = App::create_or_get_db_file()?;
        let scs = SshConfigStore::new(&db).await?;
        let config = resolve_config();

        Ok(App {
            state: AppState::Normal,
            selected_group: 0,
            config_paragraph_offset: 0,
            scs,
            host_state: TableState::default(),
            should_quit: false,
            should_spawn_ssh: false,
            config_display_mode: ConfigDisplayMode::Selected,
            db,
            searcher: Searcher::new(),
            show_help: false,
            config,
        })
    }

    pub fn create_or_get_db_file() -> Result<FileDatabase> {
        let config_dir =
            dirs::config_dir().ok_or_else(|| format_err!("Could not get config directory"))?;

        let conf_path = config_dir.join("FastSSH");
        let db_path = conf_path.join("db.ron");

        fs::create_dir_all(&conf_path)
            .with_context(|| format_err!("Could not create the config directory"))?;

        FileDatabase::new(db_path.to_str().unwrap())
    }

    pub fn get_selected_group(&self) -> &SshGroup {
        &self.scs.groups[self.selected_group]
    }

    pub fn get_selected_item(&self) -> Option<&SshGroupItem> {
        if let Some(host_state) = self.host_state.selected() {
            let items_len = self.get_items_based_on_mode().len();
            if host_state < items_len {
                Some(self.get_items_based_on_mode()[host_state])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_all_items(&self) -> Vec<&SshGroupItem> {
        self.scs
            .groups
            .iter()
            .map(|group| &group.items)
            .flatten()
            .collect::<Vec<&SshGroupItem>>()
    }

    pub fn get_items_based_on_mode(&self) -> Vec<&SshGroupItem> {
        let items: Vec<&SshGroupItem> = match self.state {
            AppState::Normal => self
                .get_selected_group()
                .items
                .iter()
                .collect::<Vec<&SshGroupItem>>(),
            AppState::Searching => self.searcher.get_filtered_items(self),
        };

        items
    }

    pub fn change_selected_group(&mut self, rot_right: bool) {
        let actual_idx = self.selected_group;
        let items_len = self.scs.groups.len();

        self.selected_group = match rot_right {
            true => (actual_idx + 1) % items_len,
            false => (actual_idx + items_len - 1) % items_len,
        };
    }

    pub fn change_selected_item(&mut self, rot_right: bool) {
        let items_len = self.get_items_based_on_mode().len();

        if items_len == 0 {
            return;
        }

        let i = match self.host_state.selected() {
            Some(i) => {
                if rot_right {
                    (i + 1) % items_len
                } else {
                    (i + items_len - 1) % items_len
                }
            }
            None => 0,
        };
        self.host_state.select(Some(i));
    }

    pub fn scroll_config_paragraph(&mut self, offset: i64) {
        self.config_paragraph_offset = (self.config_paragraph_offset as i64 + offset).max(0) as u16;
    }

    pub fn toggle_config_display_mode(&mut self) {
        self.config_display_mode = match self.config_display_mode {
            ConfigDisplayMode::Global => ConfigDisplayMode::Selected,
            ConfigDisplayMode::Selected => ConfigDisplayMode::Global,
        };
    }
}
