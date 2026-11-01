# SigmaOS-theme-mdm

Display manager theme collection for SigmaOS. Provides login screen themes for the MDM display manager. Inspired by mdm-themes.

## Upstream

- [mdm-themes](https://github.com/linuxmint/mdm-themes)

## Build

```sh
make install
```

## Principles

- DRY: shared login assets reused across theme variants
- Modularity: each login screen theme is a self-contained directory
- FOSS: MIT licensed
- KISS: installer checks for sudo once and applies all themes
- Discoverability: COMPONENTS-entry.toml for registry lookup
