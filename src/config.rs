use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use serde::Deserialize;
use xdg::BaseDirectories;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub mpd: MPDConfiguration,
    #[serde(default = "default_notification_configuration")]
    pub notification: NotificationConfiguration,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MPDConfiguration {
    pub host: String,
    pub port: u32,
    pub library: String,
    #[serde(
        rename = "cover-art-extensions",
        default = "MPDConfiguration::default_cover_art_extensions"
    )]
    pub cover_art_extensions: Vec<String>,
}

impl MPDConfiguration {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn library(&self) -> &Path {
        Path::new(&self.library)
    }

    fn default_cover_art_extensions() -> Vec<String> {
        vec!["png", "jpg", "tiff", "bmp"]
            .into_iter()
            .map(String::from)
            .collect()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct NotificationConfiguration {
    #[serde(default = "default_appname")]
    pub appname: String,
    #[serde(rename = "unknown-title-text", default = "default_unknown_title_text")]
    pub unknown_title_text: String,
    #[serde(rename = "unknown-album-text", default = "default_unknown_album_text")]
    pub unknown_album_text: String,
}

fn default_notification_configuration() -> NotificationConfiguration {
    NotificationConfiguration {
        appname: default_appname(),
        unknown_title_text: default_unknown_title_text(),
        unknown_album_text: default_unknown_album_text(),
    }
}

fn default_appname() -> String {
    String::from("mpd")
}

fn default_unknown_title_text() -> String {
    String::from("Unknown title")
}

fn default_unknown_album_text() -> String {
    String::from("Unknown album")
}

pub fn default_file() -> Result<PathBuf> {
    let xdg_dirs = BaseDirectories::with_prefix("mpdnd")?;
    xdg_dirs
        .find_config_file("config.toml")
        .ok_or_else(|| anyhow!("could not find configuration file"))
}
