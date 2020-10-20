use crate::config::Configuration;

use std::path::{Path, PathBuf};

use anyhow::Result;
use chrono::Duration;
use mpd_client::{
    commands::{
        self,
        responses::{PlayState, Song, Status},
    },
    state_changes::StateChanges,
    Client, Subsystem,
};
use notify_rust::{Notification, Timeout};
use tokio::stream::StreamExt;

#[derive(Debug)]
pub struct MpdND {
    config: Configuration,
    client: Client,
    state_changes: StateChanges,
}

impl MpdND {
    pub async fn connect(config: Configuration) -> Result<Self> {
        let address = config.mpd.address();
        let (client, state_changes) = Client::connect_to(&address).await?;

        Ok(Self {
            config,
            client,
            state_changes,
        })
    }

    pub async fn watch(&mut self) -> Result<()> {
        while let Some(subsys) = self.state_changes.next().await {
            let subsys = subsys?;
            if subsys == Subsystem::Player || subsys == Subsystem::Queue {
                self.notify().await?;
            }
        }

        Ok(())
    }

    pub async fn notify(&self) -> Result<()> {
        let current = self.client.command(commands::CurrentSong).await?;
        if let Some(song_in_queue) = current {
            let status = self.client.command(commands::Status).await?;

            let song = song_in_queue.song;
            let title = song
                .title()
                .unwrap_or(&self.config.notification.text.unknown_title);
            let album = song
                .album()
                .unwrap_or(&self.config.notification.text.unknown_album);

            let state = match status.state {
                PlayState::Playing => &self.config.notification.text.playing,
                PlayState::Paused => &self.config.notification.text.paused,
                PlayState::Stopped => &self.config.notification.text.stopped,
            };

            let statuses = self.statuses_segment(&status);

            // TODO: custom duration formatting?
            let body_time = match (status.elapsed, status.duration) {
                (Some(elapsed), Some(duration)) => {
                    let elap = Duration::from_std(elapsed)?;
                    let total = Duration::from_std(duration)?;
                    format!("{} / {}", format_duration(&elap), format_duration(&total))
                }
                _ => String::new(),
            };

            // TODO: custom summary/body format
            let summary = format!("{} {}- {}", state, statuses, title);
            let body = format!("<i>{}</i>\n{}", album, body_time);

            // TODO: relevant notification actions
            let mut notification = Notification::new();
            notification
                .appname(&self.config.notification.text.appname)
                .summary(&summary)
                .body(&body)
                .timeout(Timeout::Milliseconds(self.config.notification.timeout));

            if self.config.notification.cover_art_enabled {
                let image_path = self.cover_art_path(&song);
                if let Some(icon) = image_path {
                    notification.icon(&icon.to_string_lossy());
                }
            }

            notification.show()?;
        }

        Ok(())
    }

    fn cover_art_path(&self, song: &Song) -> Option<PathBuf> {
        let file_path = song.file_path();
        let library = PathBuf::from(self.config.mpd.library());
        library
            .join(file_path)
            .parent()
            .and_then(|v| self.cover_art_in_dir(v))
            .or_else(|| {
                self.config
                    .notification
                    .default_cover_art
                    .clone()
                    .map(PathBuf::from)
            })
    }

    fn cover_art_in_dir<P: AsRef<Path>>(&self, dir: P) -> Option<PathBuf> {
        if dir.as_ref().is_dir() {
            self.config.mpd.cover_art_extensions.iter().find_map(|ext| {
                let joined = dir.as_ref().join(format!("cover.{}", ext));
                if joined.exists() {
                    Some(joined)
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    fn statuses_segment(&self, status: &Status) -> String {
        let statuses_appear = status.repeat || status.random || status.consume;
        if !statuses_appear {
            return String::new();
        }

        let repeat = if status.repeat {
            &self.config.notification.text.repeat
        } else {
            ""
        };
        let random = if status.random {
            &self.config.notification.text.random
        } else {
            ""
        };
        let consume = if status.consume {
            &self.config.notification.text.consume
        } else {
            ""
        };

        let group_l = &self.config.notification.text.status_group_left;
        let group_r = &self.config.notification.text.status_group_right;
        format!("{}{}{}{}{} ", group_l, repeat, random, consume, group_r)
    }
}

fn format_duration(duration: &Duration) -> String {
    let minutes = duration.num_minutes();
    let seconds = duration.num_seconds();
    let rem_seconds = seconds - 60 * minutes;

    format!("{:02}:{:02}", minutes, rem_seconds)
}
