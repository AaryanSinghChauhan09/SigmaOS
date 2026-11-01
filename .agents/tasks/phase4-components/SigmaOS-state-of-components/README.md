# SigmaOS-state-of-components

Component health dashboard for SigmaOS. Tracks build status, test coverage, and dependency health across all SigmaOS components. Inspired by state-of-omarchy.

## Overview

SigmaOS-state-of-components aggregates CI build results, test coverage reports, and dependency health signals for every component in the SigmaOS ecosystem into a single dashboard.

## Upstream Inspiration

- [state-of-omarchy](https://github.com/basecamp/omarchy)

## Build

```sh
cargo build --release
```

## Usage

```sh
./target/release/sigma-state-of-components
```

## Principles

- **DRY**: Shared health-check logic reused across all component types
- **Modularity**: `dashboard` and `health` modules are independently usable
- **FOSS**: MIT licensed
- **KISS**: Data fetched from standard CI APIs, rendered simply
- **Discoverability**: COMPONENTS-entry.toml for registry integration

## License

MIT 2025 SigmaOS Contributors
