# SigmaOS-theme-marketplace

Theme discovery and download marketplace for SigmaOS. Browse, preview, and install community themes directly from the desktop.

## Upstream

- [GNOME-Look.org](https://www.gnome-look.org/)

## Build

```sh
cargo build --release
```

## Principles

- DRY: catalog and download logic separated into reusable modules
- Modularity: `catalog` and `download` crates are independently testable
- FOSS: MIT licensed
- KISS: single `init()` entry point for host integration
- Discoverability: COMPONENTS-entry.toml for registry lookup
