# SigmaOS-compositor

A Wayland compositor server extracted from SigmaOS-desktop. Handles surface management, output layout, and buffer sharing for the SigmaOS desktop environment.

## Upstream Inspiration

- [wlroots](https://gitlab.freedesktop.org/wlroots/wlroots) — modular Wayland compositor library
- [smithay](https://github.com/Smithay/smithay) — Rust Wayland compositor framework
- [sway](https://github.com/swaywm/sway) — i3-compatible Wayland compositor

## Purpose

SigmaOS-compositor provides the core Wayland compositor logic:

- **Surface management**: creation, destruction, subsurface ordering, damage tracking
- **Output layout**: multi-monitor arrangement, HiDPI scaling, vsync/frame scheduling
- **Buffer sharing**: DMABUF import, SHM buffers, buffer lifecycle
- **Input routing**: keyboard/pointer focus dispatch, seat management
- **Protocol extensions**: wlr-layer-shell, xdg-output, presentation-time, Xwayland bridge

## Build Instructions

```bash
# Prerequisites: Wayland dev libraries, DRM/KMS headers, libxkbcommon
sudo apt install libwayland-dev libdrm-dev libxkbcommon-dev

cargo build --release
cargo test
```

## API Overview

### Core Types

- `Compositor` — top-level compositor state machine
- `Surface` — represents a wl_surface with damage tracking
- `Output` — physical or virtual display output
- `Buffer` — GPU-backed or SHM buffer abstraction
- `InputSeat` — keyboard + pointer seat with focus routing

### Key Traits

- `SurfaceHandler` — lifecycle callbacks for surface events
- `OutputHandler` — frame callbacks, mode changes
- `BufferHandler` — buffer import/release notifications

### Example

```rust
use sigma_compositor::{Compositor, CompositorConfig};

let config = CompositorConfig::default();
let compositor = Compositor::new(config)?;
compositor.run()?;
```
