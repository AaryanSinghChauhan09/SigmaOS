# SigmaOS-stream

IPTV and media streaming client for SigmaOS. Watch live TV channels and VOD content via M3U playlists. Inspired by Hypnotix.

## Overview

SigmaOS-stream is a lightweight IPTV and media streaming client that parses M3U playlists, organizes channels by group and category, and plays live or on-demand streams using an underlying media backend.

## Upstream Inspiration

- [Hypnotix](https://github.com/linuxmint/hypnotix)

## Build

```sh
cargo build --release
```

## Usage

```sh
./target/release/sigma-stream --playlist ~/my-channels.m3u
```

## Principles

- **DRY**: Playlist parsing and player control shared across stream types
- **Modularity**: `playlist` and `player` modules are independently usable
- **FOSS**: MIT licensed, upstream is open source
- **KISS**: M3U format, no proprietary formats required
- **Discoverability**: COMPONENTS-entry.toml for registry integration

## License

MIT 2025 SigmaOS Contributors
