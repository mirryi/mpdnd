use crate::config::Configuration;

use std::path::PathBuf;

use anyhow::Result;
use chrono::Duration;
use mpd_client::{
    commands::{self, responses::PlayState},
    state_changes::StateChanges,
    Client, Subsystem,
};
use notify_rust::Notification;
use tokio::stream::StreamExt;

const APPNAME: &str = "mpd";

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
                self.notify_song().await?;
            }
        }

        Ok(())
    }

    async fn notify_song(&self) -> Result<()> {
        let current = self.client.command(commands::CurrentSong).await?;
        if let Some(song_in_queue) = current {
            let status = self.client.command(commands::Status).await?;

            let song = song_in_queue.song;
            let title = song.title().unwrap_or("Unknown title");
            let album = song.album().unwrap_or("Unknown album");

            let file_path = song.file_path();
            let library = PathBuf::from(self.config.mpd.library());
            let image_path = library.join(file_path).parent().and_then(|v| {
                if v.is_dir() {
                    ["png", "jpg", "tiff", "bmp"].iter().find_map(|ext| {
                        let joined = v.join(format!("cover.{}", ext));
                        if joined.exists() {
                            Some(joined)
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            });

            let state = match status.state {
                PlayState::Playing => "Playing",
                PlayState::Stopped => "Stopped",
                PlayState::Paused => "Paused",
            };

            let repeat = if status.repeat { "r" } else { "" };
            let random = if status.random { "z" } else { "" };
            let consume = if status.consume { "c" } else { "" };
            let statuses = format!("[{}{}{}]", repeat, random, consume);

            let body_time = match (status.elapsed, status.duration) {
                (Some(elapsed), Some(duration)) => {
                    let elap = Duration::from_std(elapsed)?;
                    let total = Duration::from_std(duration)?;
                    format!("{} / {}", format_duration(&elap), format_duration(&total))
                }
                _ => String::new(),
            };

            let summary = format!("{} {} - {}", state, statuses, title);
            let body = format!("<i>{}</i>\n{}", album, body_time);

            let mut notification = Notification::new();
            notification
                .appname(APPNAME)
                .summary(&summary)
                .body(&body)
                .timeout(3000);
            if let Some(icon) = image_path {
                notification.icon(&icon.to_string_lossy());
            }

            notification.show()?;
        }

        Ok(())
    }
}

fn format_duration(duration: &Duration) -> String {
    let minutes = duration.num_minutes();
    let seconds = duration.num_seconds();
    let rem_seconds = seconds - 60 * minutes;

    format!("{:02}:{:02}", minutes, rem_seconds)
}
