# SigmaOS-terminal

GPU-accelerated terminal emulator for SigmaOS. Combines the best of Foot (GPU rendering), Ghostty (configuration model), Alacritty (raw speed), and Kitty (graphics protocol).

## Upstream Inspiration

- [foot](https://codeberg.org/dnkl/foot) — fast Wayland-native terminal
- [ghostty](https://mitchellh.com/ghostty) — config-first terminal philosophy
- [alacritty](https://alacritty.org/) — GPU-accelerated, no-frills speed
- [kitty](https://sw.kovidgoyal.net/kitty/) — graphics protocol, advanced features

## Purpose

- **PTY management**: fork/exec shell, read/write PTY, resize on window change
- **GPU rendering**: wgpu-backed cell renderer with glyph atlas
- **Configuration**: TOML config with hot-reload, color schemes, font selection
- **Graphics protocols**: sixel and kitty image protocol
- **Text features**: Unicode, hyperlinks, regex search, scrollback, selection

## Build Instructions

```bash
sudo apt install libfontconfig-dev

cargo build --release
cargo test
```

## API Overview

### Core Types

- `Terminal` — top-level terminal state
- `Pty` — PTY file descriptor pair with async read/write
- `Renderer` — wgpu-backed cell/glyph renderer
- `TermConfig` — parsed terminal configuration

### Example

```rust
use sigma_terminal::{Terminal, TermConfig};

let config = TermConfig::load_or_default()?;
let term = Terminal::new(config)?;
term.spawn_shell("/bin/bash")?;
```
