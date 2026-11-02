# SigmaOS-system-admin

System administration tools for SigmaOS. Manage users, groups, services, and system configuration with a unified GUI. Inspired by mintsysadm.

## Overview

SigmaOS-system-admin provides a unified interface for common system administration tasks: user and group management, service control, and system configuration — all from a single, consistent GUI.

## Upstream Inspiration

- [mintsysadm](https://github.com/linuxmint/mintsysadm)

## Build

```sh
cargo build --release
```

## Usage

```sh
./target/release/sigma-system-admin
```

## Principles

- **DRY**: Shared privilege-escalation and D-Bus plumbing across all admin modules
- **Modularity**: `users` and `services` modules are independently usable
- **FOSS**: MIT licensed, upstream is open source
- **KISS**: Thin wrapper over existing system APIs (passwd, systemd)
- **Discoverability**: COMPONENTS-entry.toml for registry integration

## License

MIT 2025 SigmaOS Contributors
