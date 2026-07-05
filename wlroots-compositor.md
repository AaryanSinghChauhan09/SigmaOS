# Zenith Compositor: Smithay-Based Wayland Compositor

## Overview

The Zenith desktop compositor is SigmaOS's Wayland compositor. It is implemented **cleanroom** in Rust using the [Smithay](https://github.com/Smithay/smithay) library (MIT), **not** wlroots (MIT, C). Smithay provides high-level Wayland protocol handling, KMS/DRM output, libinput input, and EGL/GBM GPU integration entirely in Rust.

The wlroots project (MIT, C) was studied for interface design only. No wlroots code is included.

---

## Architecture

```
Wayland clients (weston-terminal, sigma-edit, etc.)
        │  Wayland socket: /run/user/1000/wayland-0
        ▼
  Zenith Compositor (desktop/compositor/)
        │  Smithay WaylandServer + CompositorHandler
        │  ShmHandler (software rendering)
        │  SeatHandler (keyboard, pointer, touch)
        │  OutputHandler (DRM/KMS or virtio-gpu)
        ▼
  KMS/DRM → kernel DRM atomic modesetting
  (or: virtio-gpu in QEMU CI)
```

---

## File Layout

```
desktop/compositor/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs          # entry point, event loop

    ├── compositor.rs    # CompositorHandler impl

    ├── output.rs        # DRM/KMS output setup

    ├── seat.rs          # keyboard/pointer/touch

    ├── shell.rs         # xdg-shell surface management

    └── render.rs        # EGL + GBM renderer

```

---

## Cargo.toml

```toml
[package]
name    = "zenith-compositor"
version = "0.1.0"
edition = "2021"

[dependencies]
smithay = { version = "=0.4.0", features = [
    "backend_drm",
    "backend_egl",
    "backend_gbm",
    "backend_libinput",
    "backend_winit",   # for QEMU/headless CI

    "wayland_frontend",
    "xwayland",
] }
tracing          = "0.1"
tracing-subscriber = "0.3"
calloop          = "0.13"
```

---

## Compositor Skeleton (src/main.rs)

```rust
use smithay::{
    delegate_compositor, delegate_shm, delegate_seat, delegate_output,
    delegate_xdg_shell,
    reexports::wayland_server::{Display, ListeningSocket},
    wayland::{
        compositor::CompositorState,
        shell::xdg::XdgShellState,
        shm::ShmState,
        seat::SeatState,
        output::OutputManagerState,
    },
    backend::winit,
};

struct ZenithState {
    compositor_state:   CompositorState,
    xdg_shell_state:    XdgShellState,
    shm_state:          ShmState,
    seat_state:         SeatState<ZenithState>,
    output_manager:     OutputManagerState,
}

impl ZenithState {
    fn new(display: &mut Display<ZenithState>) -> Self {
        let dh = display.handle();
        Self {
            compositor_state:   CompositorState::new::<Self>(&dh),
            xdg_shell_state:    XdgShellState::new::<Self>(&dh),
            shm_state:          ShmState::new::<Self>(&dh, vec![]),
            seat_state:         SeatState::new(),
            output_manager:     OutputManagerState::new_with_xdg_output::<Self>(&dh),
        }
    }
}

delegate_compositor!(ZenithState);
delegate_shm!(ZenithState);
delegate_xdg_shell!(ZenithState);

fn main() {
    tracing_subscriber::fmt::init();
    let mut display: Display<ZenithState> = Display::new().unwrap();
    let mut state = ZenithState::new(&mut display);

    let socket = ListeningSocket::bind("wayland-0").unwrap();
    tracing::info!("Zenith compositor listening on {:?}", socket.socket_name());

    // Use winit backend for QEMU/CI; swap for DRM in production
    let (mut winit_backend, mut winit_evt) =
        winit::init::<ZenithState>().unwrap();

    loop {
        winit_evt.dispatch_new_events(|event| {
            // handle input events
            let _ = event;
        }).unwrap();
        winit_backend.render_frame(|_renderer, _output, _state| {
            // render surfaces
        }).unwrap();
        display.dispatch_clients(&mut state).unwrap();
        display.flush_clients().unwrap();
    }
}
```

---

## KMS/DRM Integration

In production (real hardware), the winit backend is replaced with the Smithay DRM backend:

```rust
use smithay::backend::drm::{DrmDevice, DrmDeviceFd};
use smithay::backend::gbm::GbmDevice;

let drm_fd = DrmDeviceFd::open("/dev/dri/card0").unwrap();
let drm_device = DrmDevice::new(drm_fd.clone(), false).unwrap();
let gbm_device = GbmDevice::new(drm_fd).unwrap();
// Configure atomic modesetting: CRTC, plane, connector
```

---

## Exit Criteria

- `weston-terminal` runs on Zenith compositor in QEMU with virtio-gpu.

- Keyboard input and window focus work correctly.

- `cargo test -p zenith-compositor` passes Smithay unit tests.
