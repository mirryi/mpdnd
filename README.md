[![Build status](https://github.com/mirryi/mpdnd/workflows/ci/badge.svg)](https://github.com/mirryi/mpdnd/actions)
[![Crates.io](https://img.shields.io/crates/v/mpdnd.svg)](https://crates.io/crates/mpdnd)

# mpdnd

mpdnd is a notification daemon for MPD.

![An example notification](assets/example.png)

mpdnd does not display notifications itself, it simply emits [XDG Desktop Notifications
Specification](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html)
notifications.

## Installation

Install via Cargo:

```bash
$ cargo install mpdnd
```

## Configuration

A configuration file at `${XDG_CONFIG_HOME}/mpdnd/config.toml` must be created to
look like this:

``` toml
[mpd]
host = "localhost"
port = 6600
library = "~/music"
```

See [`etc/default.toml`](etc/default.toml) for options and default values.

## Usage

See `mpdnd --help` for full details.

To listen for MPD song changes and emit notificiations:

```bash
$ mpdnd
```

To emit a notification for the current song:

```bash
$ mpdnd -n
```
