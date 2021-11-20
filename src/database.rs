use anyhow::{Context, Result};
use rustbreak::{deser::Ron, FileDatabase as _FileDatabase, RustbreakError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct FileDatabase {
    db: _FileDatabase<HashMap<String, HostDatabaseEntry>, Ron>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct HostDatabaseEntry {
    pub connection_count: i64,
    pub last_used_date: i64,
}

impl FileDatabase {
    pub fn new(filename: &str) -> Result<FileDatabase> {
        let db =
            _FileDatabase::<HashMap<String, HostDatabaseEntry>, Ron>::load_from_path_or_default(
                filename,
            )
            .with_context(|| format!("Error while loading database from {}", filename))?;

        db.load()?;
        Ok(FileDatabase { db })
    }

    pub fn get_host_values(&self, host_key: &str) -> Result<HostDatabaseEntry, RustbreakError> {
        self.db.read(|db| {
            let key_value = db.get_key_value(host_key);

            if let Some(value) = key_value {
                *value.1
            } else {
                HostDatabaseEntry {
                    connection_count: 0,
                    last_used_date: 0,
                }
            }
        })
    }

    pub fn save_host_values(
        &self,
        host_key: &str,
        connection_count: i64,
        last_used_date: i64,
    ) -> Result<(), RustbreakError> {
        self.db.write(|db| {
            db.insert(
                host_key.to_owned(),
                HostDatabaseEntry {
                    connection_count,
                    last_used_date,
                },
            );
        })?;

        self.db.save()?;
        Ok(())
    }
}
