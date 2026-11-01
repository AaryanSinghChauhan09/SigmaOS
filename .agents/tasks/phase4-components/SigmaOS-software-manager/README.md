# SigmaOS-software-manager

Software center and app store for SigmaOS. Browse, install, and remove applications with ratings, screenshots, and categories. Inspired by mintinstall.

## Overview

SigmaOS-software-manager provides a graphical and CLI interface for discovering, installing, and removing software on SigmaOS. It supports application ratings, screenshots, and category-based browsing.

## Upstream Inspiration

- [mintinstall](https://github.com/linuxmint/mintinstall)
- [GNOME Software](https://gitlab.gnome.org/GNOME/gnome-software)

## Build

```sh
cargo build --release
```

## Usage

```sh
./target/release/sigma-software-manager
```

## Principles

- **DRY**: Shared catalog and install logic in reusable modules
- **Modularity**: `catalog` and `install` modules are independently usable
- **FOSS**: MIT licensed, upstreams are all open source
- **KISS**: Minimal dependencies, straightforward data flow
- **Discoverability**: COMPONENTS-entry.toml for registry integration

## License

MIT 2025 SigmaOS Contributors
