# SigmaOS-systemd

Systemd integration and management tools for SigmaOS. Inspect, enable, disable, and monitor systemd units and the journal.

## Overview

SigmaOS-systemd provides a clean interface for interacting with the systemd service manager: listing and filtering units, enabling/disabling services, starting/stopping units, and reading the systemd journal.

## Upstream Inspiration

- [systemd](https://github.com/systemd/systemd)

## Build

```sh
cargo build --release
```

## Usage

```sh
./target/release/sigma-systemd
```

## Principles

- **DRY**: Shared D-Bus connection and unit-state types reused across modules
- **Modularity**: `units` and `journal` modules are independently usable
- **FOSS**: MIT licensed
- **KISS**: Communicates via systemd's standard D-Bus and journal APIs
- **Discoverability**: COMPONENTS-entry.toml for registry integration

## License

MIT 2025 SigmaOS Contributors
