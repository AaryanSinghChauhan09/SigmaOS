# Wayland Compositor & Display Server Component Agent

## Role
`SigmaCompositor` implements the core Wayland display server and window management infrastructure for SigmaOS — coordinating Wayland protocol multiplexing, hardware plane composition (DRM/KMS), GPU damage tracking, and dynamic window tiling inspired by wlroots, Sway, Hyprland, and Weston.

## Linux & BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Wayland Compositor | Sway / Hyprland / Weston | Weston | SigmaCompositor (pure Rust) |
| Protocol Engine | `wayland-server` | `wayland-server` | SigmaWayland |
| DRM/KMS Backend | wlroots DRM | wlroots | SigmaDRMBack |
| Buffer Sharing | DMA-BUF / GBM | libgbm | SigmaGBM (zero-copy GPU buffers) |
| Shell Protocols | `xdg-shell`, `layer-shell` | `xdg-shell` | SigmaShell |
| Window Placement | BSP tree / dynamic tiling | — | SigmaWM (dynamic tiling & float) |
| X11 Compatibility | XWayland | XWayland | SigmaXWayland |
| Input Subsystem | libinput | libinput | SigmaInput |

## Current SigmaOS Status
Basic framebuffer rendering, software mouse cursor, and core Wayland surface protocol dispatch implemented in `src/display/` and `src/desktop/`. Hardware DRM/KMS atomic modesetting and DMA-BUF scanout are active enhancement targets.

## Responsibilities & Architecture
- Wayland core protocol dispatch: `wl_display`, `wl_surface`, `wl_buffer`, `wl_seat`, `wl_output`.
- Shell protocol extensions: `xdg_wm_base`, `wlr-layer-shell-v1` (for bars, notifications, launchers).
- Direct DRM/KMS atomic modesetting with damage tracking for sub-5ms render latencies.
- Dynamic tiling layout engine with binary space partitioning (BSP), floating window support, and multi-workspace management.
- Multi-monitor hotplugging and per-monitor fractional scaling.
- Input event routing: keyboard keymaps (xkbcommon), pointer events, touch gestures.
- XWayland rootless bridge for legacy X11 application compatibility.

## Source Files
- `src/display/wayland.rs`
- `src/display/drm.rs`
- `src/display/compositor.rs`
- `src/display/damage.rs`
- `src/desktop/window_manager.rs`

## Testing Strategy
```bash
rustc --test src/display/wayland.rs --edition 2021 -o /tmp/test_wl && /tmp/test_wl --test-threads=1
```
- Wayland protocol conformance tests.
- 144Hz tear-free VSync rendering benchmarks.
- Offscreen headless surface composition validation.

## Future Development
1. **Direct scanout** — Fullscreen games bypass the compositor to render directly to KMS.
2. **Variable refresh rate (VRR)** — Adaptive-Sync / FreeSync frame presentation.
3. **Color management & HDR** — ICC color profiles and HDR10 wide-gamut tone mapping.
