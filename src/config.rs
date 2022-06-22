use std::{fs::File, path::Path};

use anyhow::{Error, Result};
use derive_builder::Builder;
use indexmap::{indexmap, IndexMap};
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Config {
    pub default_server: Option<String>,
    pub servers: IndexMap<String, Server>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_server: None,
            servers: indexmap![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Builder, PartialEq, Eq, Clone)]
pub struct Server {
    pub url: String,
    pub token: Option<String>,
}

impl Config {
    pub fn from_file(filename: &Path) -> Result<Self, Error> {
        if filename.exists() {
            let file = File::open(filename)?;
            let config: Self = serde_yaml::from_reader(file)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.to_file(filename)?;
            Ok(config)
        }
    }

    pub fn to_file(&self, filename: &Path) -> Result<(), Error> {
        if let Some(parent) = filename.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(filename)?;
        serde_yaml::to_writer(file, self)?;

        Ok(())
    }

    pub fn get_default_server(&self) -> Option<(String, &Server)> {
        if let Some(name) = &self.default_server {
            if let Some(server) = self.servers.get(name) {
                return Some((name.clone(), server));
            }
        }
        None
    }
}
