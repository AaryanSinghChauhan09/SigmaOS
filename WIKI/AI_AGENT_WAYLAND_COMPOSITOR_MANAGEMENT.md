# AI Agent Directive: Wayland Protocol & Compositor Subsystem Management

## Overview

The `WaylandProtocolEngine` (`src/desktop/wayland_protocol.rs`, re-exported in `src/desktop/mod.rs`) implements a zero-dependency, `#![no_std]` native Rust Wayland display server protocol engine for SigmaOS. It provides Linux desktop compositor protocol parity without external C library dependencies (e.g. `libwayland-server`).

## Key Architectural Structures

1. **`WaylandMessageWireHeader` & `WaylandWireMessageEncoder`**:
   - Parses 8-byte Wayland wire protocol headers (`object_id: u32`, `opcode_and_size: u32`).
   - Encodes events and requests into big-endian/native byte slices without heap reallocation.

2. **`WlSurfaceState`**:
   - Manages surface lifecycle, buffer attachments, frame callbacks, and regional damage tracking (`damage_x`, `damage_y`, `width`, `height`).

3. **`XdgShellState`**:
   - Manages `xdg_wm_base` ping/pong serials, `xdg_surface` roles, and `xdg_toplevel` window configure states (`ack_configure`).

4. **`WlSeatInputState`**:
   - Routes pointer motion/button events and keyboard key/modifiers events across active surfaces.

5. **`WlDataDeviceClipboardState`**:
   - Coordinates `wl_data_source` MIME offer selections, selection offers, and IPC data transfers for Wayland clipboard sharing.

## Directives for AI Agents

- **Zero-Dependency Rule**: Do NOT import `libwayland`, `ffi`, or external C crates. All protocol serialization must remain native Rust.
- **`#![no_std]` Compatibility**: Ensure core protocol logic remains `no_std` compatible, utilizing `alloc` types (`Vec`, `String`) where required.
- **Verification**: Run standalone unit tests using:
  ```bash
  rustc --test --edition 2021 src/desktop/wayland_protocol.rs -o build/wayland_test && ./build/wayland_test
  ```
