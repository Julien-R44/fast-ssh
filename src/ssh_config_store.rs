use crate::database::FileDatabase;
use anyhow::{format_err, Result};
use ssh_cfg::{SshConfig, SshConfigParser, SshHostConfig};
use std::fmt::Debug;

#[derive(Debug)]
pub struct SshGroupItem {
    pub name: String,
    pub full_name: String,
    pub connection_count: i64,
    pub last_used: i64,
    pub host_config: SshHostConfig,
}

#[derive(Debug)]
pub struct SshGroup {
    pub name: String,
    pub items: Vec<SshGroupItem>,
}

#[derive(Debug)]
pub struct SshConfigStore {
    pub config: SshConfig,
    pub groups: Vec<SshGroup>,
}

impl SshConfigStore {
    pub async fn new(db: &FileDatabase) -> Result<SshConfigStore> {
        let ssh_config = SshConfigParser::parse_home().await?;

        let mut scs = SshConfigStore {
            config: ssh_config,
            groups: Vec::new(),
        };

        scs.create_ssh_groups(db);

        if scs.groups.is_empty() {
            return Err(format_err!("Your configuration file contains no entries (or only wildcards) ! Please add at least one."));
        }

        Ok(scs)
    }

    fn create_ssh_groups(&mut self, db: &FileDatabase) {
        let mut groups: Vec<SshGroup> = vec![SshGroup {
            name: "Others".to_string(),
            items: Vec::new(),
        }];

        self.config.iter().for_each(|(key, value)| {
            let host_entry = db.get_host_values(key).unwrap();

            if key.contains('*') {
                return;
            }

            if key.contains('/') {
                let group_name = key.split('/').next().unwrap();
                let group_key = key.split('/').skip(1).collect::<Vec<&str>>().join("");

                let group = groups.iter_mut().find(|g| g.name == group_name);

                let group_item = SshGroupItem {
                    name: group_key,
                    connection_count: host_entry.connection_count,
                    last_used: host_entry.last_used_date,
                    full_name: key.to_string(),
                    host_config: value.clone(),
                };

                if group.is_none() {
                    groups.push(SshGroup {
                        name: group_name.to_string(),
                        items: vec![group_item],
                    });

                    return;
                }

                let group = &mut group.unwrap().items;
                group.push(group_item);

                return;
            }

            groups[0].items.push(SshGroupItem {
                full_name: key.to_string(),
                connection_count: host_entry.connection_count,
                last_used: host_entry.last_used_date,
                name: key.to_string(),
                host_config: value.clone(),
            });
        });

        groups.reverse();
        self.groups = groups.into_iter().filter(|g| !g.items.is_empty()).collect();
    }
}
