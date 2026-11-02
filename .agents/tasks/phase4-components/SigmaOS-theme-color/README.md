# SigmaOS-theme-color

Color theme definitions and palettes for SigmaOS. Provides the canonical SigmaOS color palette used across all themes and UI components. Inspired by mint-y-colors.

## Overview

SigmaOS-theme-color defines the authoritative color palette for SigmaOS. All themes, UI toolkits, and components reference these definitions to ensure visual consistency across the desktop.

## Upstream Inspiration

- [mint-y-colors](https://github.com/linuxmint/mint-y-colors)

## Build / Install

```sh
make install
```

This copies color definition files from `./colors/` to `$XDG_DATA_HOME/sigma/colors` (defaults to `~/.local/share/sigma/colors`).

## Principles

- **DRY**: Single source of truth for all SigmaOS color values
- **Modularity**: Color definitions are plain data files, consumable by any toolkit
- **FOSS**: MIT licensed, upstream is open source
- **KISS**: Plain files, no build step required to use the palette
- **Discoverability**: COMPONENTS-entry.toml for registry integration

## License

MIT 2025 SigmaOS Contributors
