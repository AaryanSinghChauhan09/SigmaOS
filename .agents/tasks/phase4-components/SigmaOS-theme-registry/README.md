# SigmaOS-theme-registry

Central theme registry and metadata store for SigmaOS. Maintains a registry of available themes with metadata, version info, and compatibility data.

## Upstream

- [AppStream](https://www.freedesktop.org/wiki/Distributions/AppStream/)

## Build

```sh
cargo build --release
```

## Principles

- DRY: metadata schema defined once and shared by all consumers
- Modularity: `registry` and `metadata` modules have clear boundaries
- FOSS: MIT licensed
- KISS: single `init()` bootstraps the registry from disk
- Discoverability: COMPONENTS-entry.toml for registry lookup
