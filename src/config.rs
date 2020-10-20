use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use serde::Deserialize;
use xdg::BaseDirectories;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub mpd: MPD,
    #[serde(default = "Notification::default")]
    pub notification: Notification,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MPD {
    pub host: String,
    pub port: u32,
    pub library: String,
    #[serde(
        rename = "cover-art-extensions",
        default = "MPD::default_cover_art_extensions"
    )]
    pub cover_art_extensions: Vec<String>,
}

impl MPD {
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
pub struct Notification {
    #[serde(default = "Notification::default_timeout")]
    pub timeout: u32,
    #[serde(default = "NotificationText::default")]
    pub text: NotificationText,
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            timeout: Self::default_timeout(),
            text: NotificationText::default(),
        }
    }
}

impl Notification {
    fn default_timeout() -> u32 {
        3000
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct NotificationText {
    #[serde(default = "NotificationText::default_appname")]
    pub appname: String,
    #[serde(
        rename = "unknown-title",
        default = "NotificationText::default_unknown_title_text"
    )]
    pub unknown_title: String,
    #[serde(
        rename = "unknown-album",
        default = "NotificationText::default_unknown_album_text"
    )]
    pub unknown_album: String,
}

impl Default for NotificationText {
    fn default() -> Self {
        Self {
            appname: Self::default_appname(),
            unknown_title: Self::default_unknown_title_text(),
            unknown_album: Self::default_unknown_album_text(),
        }
    }
}

impl NotificationText {
    fn default_appname() -> String {
        String::from("mpd")
    }

    fn default_unknown_title_text() -> String {
        String::from("Unknown title")
    }

    fn default_unknown_album_text() -> String {
        String::from("Unknown album")
    }
}

pub fn default_file() -> Result<PathBuf> {
    let xdg_dirs = BaseDirectories::with_prefix("mpdnd")?;
    xdg_dirs
        .find_config_file("config.toml")
        .ok_or_else(|| anyhow!("could not find configuration file"))
}
