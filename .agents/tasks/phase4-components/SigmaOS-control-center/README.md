# SigmaOS-control-center

## Overview

SigmaOS-control-center is the unified system settings application for SigmaOS. It aggregates all system settings panels — display, sound, network, users, privacy, and more — into a single, consistently designed interface, giving users a central place to configure every aspect of their desktop.

## Features

- Single-window host for all system settings panels
- Panel discovery and dynamic loading at runtime
- Search across all settings panels and options
- Keyboard navigation and accessibility support
- Per-panel permission model using PolicyKit
- Deep-link support (`sigmaos-control-center <panel-name>`) from other components
- Integration with SigmaOS-daemon-settings for live preference updates

## Upstream References

- [cinnamon-control-center](https://github.com/linuxmint/cinnamon-control-center) — Cinnamon desktop control center, primary structural reference
- [gnome-control-center](https://gitlab.gnome.org/GNOME/gnome-control-center) — GNOME settings application, referenced for panel APIs and UX patterns

## Building

Requires Rust 1.75+ and Cargo.

```bash
# Build in release mode
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Contributing

1. Fork the repository and create a feature branch.
2. Run `cargo clippy` and `cargo fmt` before submitting.
3. Add tests for any new panel registration or search logic.
4. Open a pull request against `main` with a clear description of the change.

All contributions are subject to the [MIT License](LICENSE).
