# OSS Absorption: Wayland & X11

## Overview

Wayland and X11 are the dominant display server protocols in Linux. X11 is a legacy, network-transparent protocol with a massive footprint, while Wayland is a modern, streamlined protocol where the compositor is the display server.

## Key Principles Absorbed

### Zenith Compositor (Compositor as Display Server)

- Like Wayland, SigmaOS eliminates the middleman display server. `sigma_compositor::ZenithCompositor` directly manages surfaces and renders them.
- All window management logic (`WindowManager`), tiling (`TilingNode`), and rendering (`RenderBackend`) are tightly integrated into a single unified binary, avoiding round-trips.

### Direct Input Routing

- Inputs from physical hardware (like `libinput`) are routed directly by `sigma_compositor::input::InputRouter` to the currently focused surface.
- This prevents the X11 problem where any app can sniff global keyboard events (keyloggers).

### Damage Tracking

- Rendering is highly optimized. `sigma_compositor::render::DamageTracker` keeps track of exactly which surfaces have changed, updating only the affected regions of the screen.

## Displaced Technologies

| Technology | SigmaOS Replacement |
| --- | --- |
| X11 / XServer | `ZenithCompositor` |
| Wayland / wlroots | `ZenithCompositor` native API |
| libinput | `InputRouter` |
| Window Managers (i3, sway) | `WindowManager` + `TilingNode` |

## Status

**Core Absorbed** — The display server foundations, layout engine, input routing, and damage tracking primitives are fully integrated in `userland/sigma_compositor`.
