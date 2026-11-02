# SigmaOS-theme-cursor

Cursor theme collection for SigmaOS. Includes multiple cursor themes with consistent visual styling. Inspired by mint-cursor-themes and Bibata.

## Overview

SigmaOS-theme-cursor ships a curated set of cursor themes with a consistent visual language for the SigmaOS desktop. Themes are installed to the standard XDG icons directory for automatic discovery by GTK, Qt, and Wayland compositors.

## Upstream Inspiration

- [mint-cursor-themes](https://github.com/linuxmint/mint-cursor-themes)
- [Bibata](https://github.com/ful1e5/Bibata_Cursor)

## Build / Install

```sh
make install
```

This copies cursor themes from `./cursors/` to `$HOME/.local/share/icons/`.

## Principles

- **DRY**: Shared install logic in `src/main.sh` used for all cursor variants
- **Modularity**: Each cursor theme is a self-contained directory
- **FOSS**: MIT licensed, upstreams are open source
- **KISS**: Standard XCursor format, no custom tooling required
- **Discoverability**: COMPONENTS-entry.toml for registry integration

## License

MIT 2025 SigmaOS Contributors
