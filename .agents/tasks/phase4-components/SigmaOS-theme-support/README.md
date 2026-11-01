# SigmaOS-theme-support

Theme compatibility library for SigmaOS. Provides shared theming APIs and compatibility shims for applications. Inspired by libadwaita.

## Upstream

- [libadwaita](https://gitlab.gnome.org/GNOME/libadwaita)

## Build

```sh
cargo build --release
```

## Principles

- DRY: compatibility shims centralised; no per-app duplication
- Modularity: `compat` and `api` modules can be used independently
- FOSS: MIT licensed
- KISS: one `init()` call wires up the full compatibility layer
- Discoverability: COMPONENTS-entry.toml for registry lookup
