# SigmaOS-theming-native

Native theming engine integration for SigmaOS. Bridges the SigmaOS theme system with native toolkit rendering for consistent visual results. Inspired by aether.

## Upstream

- [aether](https://github.com/aether-theme/aether)

## Build

```sh
cargo build --release
```

## Principles

- DRY: engine core shared; toolkit-specific bridges are thin adapters
- Modularity: `engine` and `bridge` modules replaceable independently
- FOSS: MIT licensed
- KISS: `init()` auto-detects the active toolkit and loads the right bridge
- Discoverability: COMPONENTS-entry.toml for registry lookup
