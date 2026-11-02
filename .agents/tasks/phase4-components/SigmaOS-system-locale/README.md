# SigmaOS-system-locale

System locale and language settings manager for SigmaOS. Configure system language, regional formats, and input methods. Inspired by mintlocale.

## Overview

SigmaOS-system-locale manages system-wide locale settings: language selection, regional formats (dates, numbers, currency), and input method configuration.

## Upstream Inspiration

- [mintlocale](https://github.com/linuxmint/mintlocale)

## Build

```sh
cargo build --release
```

## Usage

```sh
./target/release/sigma-system-locale
```

## Principles

- **DRY**: Shared locale-data accessors reused by both locale and language modules
- **Modularity**: `locale` and `language` modules are independently usable
- **FOSS**: MIT licensed, upstream is open source
- **KISS**: Thin wrapper over glibc locale and ibus/fcitx input methods
- **Discoverability**: COMPONENTS-entry.toml for registry integration

## License

MIT 2025 SigmaOS Contributors
