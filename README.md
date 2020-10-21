# mpdnd

mpdnd is a notification daemon for MPD.

![An example notification](assets/example.png)

## Installation

Run `cargo install --git https://github.com/Dophin2009/mpdnd`.

A configuration file at `$XDG_CONFIG_HOME/mpdnd/config.toml` must be
created to look like this:

``` toml
[mpd]
host = "localhost"
port = 6600
library = "~/music"
```

See [etc/default.toml](etc/default.toml) for options and default values.

## Usage

See `mpdnd --help`.
