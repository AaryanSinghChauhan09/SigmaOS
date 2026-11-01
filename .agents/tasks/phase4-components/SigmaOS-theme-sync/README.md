# SigmaOS-theme-sync

Cross-application theme synchronization tool for SigmaOS. Keep all application themes in sync with a single unified configuration.

## Upstream

- [Oomox](https://github.com/themix-project/oomox)

## Build

```sh
cargo build --release
```

## Principles

- DRY: single config file drives all application theme outputs
- Modularity: `sync` engine and `config` parser are separate concerns
- FOSS: MIT licensed
- KISS: `init()` reads config and triggers a full sync in one call
- Discoverability: COMPONENTS-entry.toml for registry lookup
