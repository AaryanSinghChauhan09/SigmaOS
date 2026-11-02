# SigmaOS-term

Terminal emulator with multiple backend support for SigmaOS. Supports Foot, Ghostty, Alacritty, and Kitty as interchangeable backends. Inspired by ptyxis-minted and omaterm.

## Overview

SigmaOS-term provides a unified terminal emulator experience on SigmaOS. A common configuration layer sits above a pluggable backend system, allowing users to choose their preferred terminal implementation — Foot, Ghostty, Alacritty, or Kitty — without changing how SigmaOS integrates with it.

## Upstream Inspiration

- [ptyxis-minted](https://github.com/linuxmint/ptyxis-minted)
- [omaterm](https://github.com/basecamp/omarchy)
- [Foot](https://codeberg.org/dnkl/foot)
- [Ghostty](https://ghostty.org)
- [Alacritty](https://github.com/alacritty/alacritty)
- [Kitty](https://github.com/kovidgoyal/kitty)

## Build

```sh
cargo build --release
```

## Usage

```sh
./target/release/sigma-term
```

## Principles

- **DRY**: Shared config model applied uniformly across all backends
- **Modularity**: `backend` and `config` modules are independently usable
- **FOSS**: MIT licensed, all upstream terminals are open source
- **KISS**: Backend selection via a single config key, no patching of upstreams
- **Discoverability**: COMPONENTS-entry.toml for registry integration

## License

MIT 2025 SigmaOS Contributors
