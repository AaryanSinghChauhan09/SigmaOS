# SigmaOS-terminal-text-effects

Terminal text animation effects library for SigmaOS. Render rich text animations and effects in the terminal. Inspired by ttfx (terminal text effects).

## Overview

SigmaOS-terminal-text-effects is a library and CLI tool for producing rich, animated text effects directly in the terminal. It provides a collection of built-in effects and a renderer abstraction that integrates with SigmaOS boot sequences, notifications, and UI transitions.

## Upstream Inspiration

- [ttfx (terminal text effects)](https://github.com/ChrisBuilds/terminaltexteffects)

## Build

```sh
cargo build --release
```

## Usage

```sh
echo "SigmaOS" | ./target/release/sigma-tte --effect matrix
```

## Principles

- **DRY**: Single renderer abstraction shared by all effect implementations
- **Modularity**: `effects` and `renderer` modules are independently usable
- **FOSS**: MIT licensed, upstream is open source
- **KISS**: Effects are declarative data structures, not imperative scripts
- **Discoverability**: COMPONENTS-entry.toml for registry integration

## License

MIT 2025 SigmaOS Contributors
