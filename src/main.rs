mod config;
mod mpdnd;

use config::Configuration;
use mpdnd::MpdND;

use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use clap::{crate_version, Clap};
use serde::Deserialize;

#[derive(Clap, Clone, Debug, Deserialize)]
#[clap(version = crate_version!(), author = env!("CARGO_PKG_AUTHORS"), about = "MPD notification daemon")]
pub struct Opts {
    #[clap(short, long, about = "Specify an alternate configuration file")]
    pub config: Option<String>,
    #[clap(short, long, about = "Display a notification for the current status and exit")]
    pub now: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let config_path = match &opts.config {
        Some(v) => PathBuf::from(v),
        None => config::default_file()?,
    };
    let config_str = fs::read_to_string(&config_path)?;
    let config: Configuration = toml::from_str(&config_str)?;

    let mut mpdnd = MpdND::connect(config).await?;
    if opts.now {
        mpdnd.notify().await?;
    } else {
        mpdnd.watch().await?;
    }

    Ok(())
}
