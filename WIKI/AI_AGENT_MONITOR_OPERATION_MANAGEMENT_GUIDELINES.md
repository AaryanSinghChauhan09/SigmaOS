# SigmaOS AI Agent Monitor Operation Management Guidelines

## 1. Executive Summary & Overview

Display monitor operations in SigmaOS encompass Linux/BSD DRM/KMS kernel output drivers, Wayland compositor surface managers (Zenith Wayland, KDE Plasma 6 KWin, GNOME 46 Mutter, Sway/Regolith), EDID/DisplayID metadata parsing, multi-display geometry layouts, and display hardware metrics.

This document establishes the official guidelines and architectural standards for AI agents managing display monitor outputs, multi-monitor topographies, hotplug event processing, fractional scaling, refresh rates, and display hardware telemetry in SigmaOS.

---

## 2. Core Architectural Components for Display Monitor Operations

AI agents managing display monitors interface with key subsystem engines:

| Engine / Component | Module Path | Operational Responsibilities |
| :--- | :--- | :--- |
| `KdePlasma6Engine` | `src/desktop/ultimate_distro_desktop.rs` | KWin Wayland split-tiling grid geometry, multi-monitor output management |
| `Gnome46MutterEngine` | `src/desktop/ultimate_distro_desktop.rs` | Mutter fractional scaling surface management, display logical output layout |
| `MultiArchHalManager` | `src/hal/multi_arch.rs` | Hardware abstraction, display interrupt management, VBLANK vblank events |
| `BsdDevdHardwareEventDispatcher` | `src/distro/bsd_linux_innovations.rs` | Hotplug display connector detection (HDMI, DisplayPort, eDP, USB-C Alt-Mode) |

---

## 3. Display Monitor Management Protocol for AI Agents

### 3.1 DRM/KMS Mode Setting & EDID/DisplayID Resolution

1. **EDID / DisplayID Parsing**:
   - AI agents query display monitor capabilities (preferred resolution, physical dimensions, color space, color depth, supported refresh rates) via EDID or DisplayID blobs provided by DRM/KMS connectors.
2. **Atomic KMS Commit (`drmModeAtomicCommit`)**:
   - Display state modifications (CRTC mode, plane surface assign, gamma LUT) must be committed atomically to prevent visual tearing or flickering.

---

### 3.2 Multi-Monitor Topography & Coordinate System

1. **Global Logical Output Coordinates**:
   - Multi-monitor setups map outputs into a non-overlapping global 2D Cartesian coordinate space `(X, Y, Width, Height)`.
2. **Layout Modes**:
   - **Extended Desktop**: Displays are placed side-by-side or stacked (`X = X_prev + Width_prev`).
   - **Mirrored / Cloned Desktop**: Multiple physical outputs map to the same logical viewport origin `(0, 0)`.
   - **Display Rotation**: Support 0°, 90°, 180°, and 270° transform matrices for vertical/portrait monitor setups.

---

### 3.3 Fractional Scaling & Refresh Rate Management

1. **Wayland Fractional Scale Protocol (`wp_fractional_scale_v1`)**:
   - Surface buffers render at native physical pixel dimensions while applying fractional scale factors (1.0x, 1.25x, 1.5x, 1.75x, 2.0x, 2.25x) to eliminate blurriness.
2. **High Refresh Rate & Variable Refresh Rate (VRR / FreeSync / G-Sync)**:
   - Compositors negotiate preferred refresh rates (60Hz, 120Hz, 144Hz, 240Hz) and enable VRR dynamically during full-screen application execution.

---

### 3.4 Display Hotplug Event Processing

When a monitor connector is plugged or unplugged:

1. **Hotplug Detection**: `BsdDevdHardwareEventDispatcher` captures `HOTPLUG` kernel uevents or devd notifications.
2. **Fallback Topology Configuration**: If the primary monitor is unplugged, the compositor automatically migrates windows to the remaining active output without application crashes.
3. **Session State Persistence**: Desktop layout settings per EDID serial number are persisted across reboots and hotplug reconnects.

---

## 4. Verification & UI/UX Benchmark Protocol

AI agents modifying compositor or monitor layout logic must pass verification:

1. **UI/UX Benchmark Runner**: Run `./run_sigma_tests.sh` to execute the UI/UX compositor benchmark suite (validating frame times < 16ms for 60Hz or < 6.9ms for 144Hz).
2. **Subsystem Test Matrix**: Execute `cargo test --lib` to ensure no regressions in desktop environment engines.

---

*Approved by the SigmaOS Zenith Wayland & Desktop Architecture Committee.*
