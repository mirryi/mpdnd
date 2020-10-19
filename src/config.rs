use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use serde::Deserialize;
use xdg::BaseDirectories;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub mpd: MPDConfiguration,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MPDConfiguration {
    pub host: String,
    pub port: u32,
    pub library: String,
}

impl MPDConfiguration {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn library(&self) -> &Path {
        Path::new(&self.library)
    }
}

pub fn default_file() -> Result<PathBuf> {
    let xdg_dirs = BaseDirectories::with_prefix("mpdnd")?;
    xdg_dirs
        .find_config_file("config.toml")
        .ok_or_else(|| anyhow!("could not find configuration file"))
}
