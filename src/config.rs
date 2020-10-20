use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use serde::Deserialize;
use xdg::BaseDirectories;

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub mpd: Mpd,
    #[serde(default = "Notification::default")]
    pub notification: Notification,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mpd {
    pub host: String,
    pub port: u32,
    pub library: String,
    #[serde(
        rename = "cover-art-extensions",
        default = "Mpd::default_cover_art_extensions"
    )]
    pub cover_art_extensions: Vec<String>,
}

impl Mpd {
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
    #[serde(
        rename = "cover-art",
        default = "Notification::default_cover_art_enabled"
    )]
    pub cover_art_enabled: bool,
    #[serde(rename = "default-cover-art")]
    pub default_cover_art: Option<String>,
    #[serde(default = "NotificationText::default")]
    pub text: NotificationText,
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            timeout: Self::default_timeout(),
            cover_art_enabled: Self::default_cover_art_enabled(),
            default_cover_art: None,
            text: NotificationText::default(),
        }
    }
}

impl Notification {
    fn default_timeout() -> u32 {
        3000
    }

    fn default_cover_art_enabled() -> bool {
        true
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct NotificationText {
    #[serde(default = "NotificationText::default_appname")]
    pub appname: String,
    #[serde(default = "NotificationText::default_playing")]
    pub playing: String,
    #[serde(default = "NotificationText::default_paused")]
    pub paused: String,
    #[serde(default = "NotificationText::default_stopped")]
    pub stopped: String,
    #[serde(default = "NotificationText::default_repeat")]
    pub repeat: String,
    #[serde(default = "NotificationText::default_random")]
    pub random: String,
    #[serde(default = "NotificationText::default_consume")]
    pub consume: String,
    #[serde(
        rename = "status-group-left",
        default = "NotificationText::default_status_group_left"
    )]
    pub status_group_left: String,
    #[serde(
        rename = "status-group-right",
        default = "NotificationText::default_status_group_right"
    )]
    pub status_group_right: String,
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
            playing: Self::default_playing(),
            paused: Self::default_paused(),
            stopped: Self::default_stopped(),
            repeat: Self::default_repeat(),
            random: Self::default_random(),
            consume: Self::default_consume(),
            status_group_left: Self::default_status_group_left(),
            status_group_right: Self::default_status_group_right(),
            unknown_title: Self::default_unknown_title_text(),
            unknown_album: Self::default_unknown_album_text(),
        }
    }
}

impl NotificationText {
    fn default_appname() -> String {
        String::from("mpd")
    }

    fn default_playing() -> String {
        String::from("Playing")
    }

    fn default_paused() -> String {
        String::from("Paused")
    }

    fn default_stopped() -> String {
        String::from("Stopped")
    }

    fn default_repeat() -> String {
        String::from('r')
    }

    fn default_random() -> String {
        String::from('z')
    }

    fn default_consume() -> String {
        String::from('c')
    }

    fn default_status_group_left() -> String {
        String::from('(')
    }

    fn default_status_group_right() -> String {
        String::from(')')
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
