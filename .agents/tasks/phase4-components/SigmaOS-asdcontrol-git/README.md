# SigmaOS-asdcontrol-git

## Overview

SigmaOS-asdcontrol-git is a low-level display control utility for SigmaOS that manages hardware-specific display settings exposed through the ASD (Advanced System Display) interface. It provides a Rust library and CLI for reading and writing display parameters such as brightness, contrast, and color temperature at the hardware register level, bypassing the higher-level compositor layer when necessary.

## Features

- Direct ASD register read/write via `/dev/i2c-*` and DDC/CI protocol
- Brightness and contrast control independent of compositor
- Color temperature adjustment (warm/cool presets and manual Kelvin values)
- Display enumeration across multi-monitor configurations
- Safe register access with bounds checking and rollback on error
- JSON-serialisable output for integration with system dashboards

## Upstream References

- [asdcontrol](https://github.com/groeck/asdcontrol) — Low-level ASD display control utility this component extends

## Building

This component is written in Rust.

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Use the CLI
./target/release/asdcontrol --list-displays
./target/release/asdcontrol --set brightness=80 --display 0
```

Requirements: Rust 1.70+, `libi2c-dev` for DDC/CI support.

## Contributing

1. Fork this repository and create a feature branch.
2. Run `cargo fmt` and `cargo clippy -- -D warnings` before submitting.
3. All unsafe blocks must include a safety comment explaining the invariants upheld.
4. Open a pull request against `main` with a clear description of the change.
5. All contributions must be licensed under the MIT License.
