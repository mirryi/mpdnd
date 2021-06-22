[![Build status](https://github.com/Dophin2009/mpdnd/workflows/ci/badge.svg)](https://github.com/Dophin2009/mpdnd/actions)
[![Crates.io](https://img.shields.io/crates/v/mpdnd.svg)](https://crates.io/crates/mpdnd)

# mpdnd

mpdnd is a notification daemon for MPD.

![An example notification](assets/example.png)

## Installation

Run `cargo install mpdnd`.

A configuration file at `$XDG_CONFIG_HOME/mpdnd/config.toml` must be created to
look like this:

``` toml
[mpd]
host = "localhost"
port = 6600
library = "~/music"
```

See [`etc/default.toml`](etc/default.toml) for options and default values.

## Usage

See `mpdnd --help`.
