# Display Server Agent

## Role
`SigmaDisplay` implements the display server stack for SigmaOS — providing a native Wayland compositor and legacy X11 support (via XWayland) for graphical applications.

## Status
🔴 Not Started — Critical for desktop/graphical operation.

## Source File
`src/display/` (planned: `wayland_compositor.rs`, `xwayland.rs`, `drm_backend.rs`)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Wayland compositor | Weston / Sway / KWin | Weston | SigmaCompositor |
| Wayland protocol | `wayland-server` | `wayland-server` | SigmaWayland |
| X11 compat | XWayland | XWayland | SigmaXWayland |
| DRM backend | wlroots DRM | wlroots | SigmaDRMBack |
| KMS/gbm | libgbm / libdrm | libgbm | SigmaGBM |
| EGL | Mesa EGL | Mesa EGL | SigmaEGL |
| Input | libinput | libinput | SigmaInput |
| Screen capture | `wlr-screencopy` | — | SigmaCapture |
| Remote desktop | RDP via FreeRDP | — | SigmaRDP |

## Responsibilities
- Wayland core protocol: `wl_display`, `wl_surface`, `wl_buffer`, `wl_seat`
- Wayland shell protocols: `xdg-shell`, `layer-shell`, `xdg-output`
- DRM/KMS backend for direct display output
- Vulkan/EGL rendering pipeline
- Input event routing: keyboard, mouse, touch, tablet
- Window management: placement, stacking, focus
- XWayland integration for legacy X11 apps
- Screen capture (`wlr-screencopy` protocol)
- Screen sharing via PipeWire integration
- Remote desktop via RDP/VNC

## Implementation Notes
- Initially: use wlroots as reference, rewrite critical paths in Rust
- `SigmaCompositor { surfaces: Vec<Surface>, seat: Seat, outputs: Vec<Output> }`
- DRM backend: create EGL context on DRM fd, render surfaces via Vulkan
- Input: `libinput` events → Wayland `wl_pointer`/`wl_keyboard` events

## Testing
Display server requires display hardware; test compositor logic via offscreen rendering:
```bash
rustc --test src/display/wayland_compositor.rs --edition 2021 -o /tmp/test_wl && /tmp/test_wl --test-threads=1
```

## Future Development
1. **Native Rust Wayland** — Full Wayland compositor in Rust (smithay-based)
2. **Color management** — ICC profiles, HDR pipeline
3. **Variable refresh rate** — VRR/FreeSync in compositor
4. **Direct scanout** — Full-screen app renders directly to KMS (bypass compositor)
5. **Multi-GPU rendering** — Render on dGPU, scanout on iGPU
6. **Eye tracking** — Wayland protocol for eye tracking input
