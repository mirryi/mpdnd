mod config;
mod mpdnd;

use config::Configuration;
use mpdnd::MpdND;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{crate_version, Clap};
use serde::Deserialize;

#[derive(Clap, Clone, Debug, Deserialize)]
#[clap(version = crate_version!(), author = env!("CARGO_PKG_AUTHORS"), about = "MPD notification daemon")]
pub struct Opts {
    #[clap(short, long, about = "Specify an alternate configuration file")]
    pub config: Option<String>,
    #[clap(
        short,
        long,
        about = "Display a notification for the current status and exit"
    )]
    pub now: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let config_path = match &opts.config {
        Some(v) => PathBuf::from(v),
        None => config::default_file()?,
    };

    let config = load_config(&config_path)
        .with_context(|| format!("Couldn't load configuration from {}", config_path.display()))?;
    let mut mpdnd = MpdND::connect(config).await?;
    if opts.now {
        mpdnd.notify().await?;
    } else {
        mpdnd.watch().await?;
    }

    Ok(())
}

fn load_config(path: impl AsRef<Path>) -> Result<Configuration> {
    let config_str = fs::read_to_string(path)?;
    let config = toml::from_str(&config_str)?;
    Ok(config)
}
