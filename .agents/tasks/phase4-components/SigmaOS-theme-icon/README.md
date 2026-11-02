# SigmaOS-theme-icon

Icon theme collection for SigmaOS. Provides complete icon sets for system, application, and status icons. Inspired by mint-l-icons and mint-y-icons.

## Upstream

- [mint-l-icons](https://github.com/linuxmint/mint-l-icons)
- [mint-y-icons](https://github.com/linuxmint/mint-y-icons)

## Build

```sh
make install
```

## Principles

- DRY: base icon layer shared; series-specific overlays applied on top
- Modularity: L-series and Y-series packaged independently
- FOSS: MIT licensed
- KISS: `make install` handles symlink hierarchy automatically
- Discoverability: COMPONENTS-entry.toml for registry lookup
