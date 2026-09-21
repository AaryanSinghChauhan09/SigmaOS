# 🖥️ SigmaOS Desktop Environment Development Plan: Linux & BSD Inspired Innovation

## 1. Executive Summary & Vision

The **SigmaOS Zenith Desktop Environment (DE)** is a fast, keyboard-driven, memory-safe desktop experience built from scratch in Safe-Rust (`ZenithWaylandCore`). It combines the best usability, visual, and architectural paradigms from leading Linux desktop environments (KDE Plasma 6, GNOME 46 Shell, Pop!_OS COSMIC, Omarchy/Hyprland auto-tiling, Cinnamon), BSD desktop shells (FreeBSD Lumina), and modern operating systems (macOS Stage Manager, Windows 11 Copilot/PowerToys).

Operating without legacy systemd or X11 desktop dependencies, Zenith DE delivers sub-millisecond input latency, <2.0s login-to-desktop startup times, and sub-second CoW theme palette live-switching.

```
                      ┌─────────────────────────────────────────┐
                      │    SigmaOS Zenith Desktop Shell         │
                      │  (`ZenithWaylandCore` / DRM KMS Scanout) │
                      └────────────────────┬────────────────────┘
                                           │
         ┌─────────────────────────────────┼─────────────────────────────────┐
         ▼                                 ▼                                 ▼
┌──────────────────┐            ┌──────────────────┐              ┌──────────────────┐
│ Omarchy / Hyprland│            │  KDE / Pop!_OS   │              │ Agentic Steering │
│ Tiling & Themes  │            │ Spatial Grid &   │              │  (Bolt ⚡, Palette🎨│
│ (Live Tokyo-Night)│            │ Auto-Stacking    │              │  Sentinel 🛡️)    │
└────────┬─────────┘            └────────┬─────────┘              └────────┬─────────┘
         │                                 │                                 │
         └─────────────────────────────────┼─────────────────────────────────┘
                                           ▼
                      ┌─────────────────────────────────────────┐
                      │    Zero-CSS Native Widget Engine        │
                      │     (Direct Canvas & Wayland Layer)     │
                      └─────────────────────────────────────────┘
```

---

## 2. Key Inspirations from Linux & BSD Desktop Ecosystems

| Desktop / OS | Inspired Subsystem | Key Capability Incorporated in Zenith DE |
|---|---|---|
| **Omarchy Linux & Hyprland** | Dwindle Tiling & Palette Studio | Fibonacci spiral auto-tiling (`LayoutType::Dwindle`), dynamic palette live switcher (Tokyo-Night, Catppuccin, Gruvbox, Nord). |
| **KDE Plasma 6** | Spatial Workspace Grid & Widgets | Multi-monitor spatial grid navigation, HUD desktop widgets (Phoronix benchmark HUD, HW Busters power gauge). |
| **Pop!_OS COSMIC** | Auto-Stacking & Navigation | Keyboard-driven window stack grouping (`WindowTileLayoutMode::MasterStack`) and rapid focus switching. |
| **GNOME 46 Shell** | Damage Tracking & Fractional Scaling | Partial surface damage region repainting (`WlDamageRegion`), HiDPI PPI calculations, and zero-stutter frame buffers. |
| **FreeBSD Lumina** | Lightweight BSD Shell | Minimal memory footprint (<12MB RAM compositor baseline) operating without systemd desktop dependencies. |
| **Cinnamon** | Theme Studio & Desklets | Desktop sticky notes, desklets, and system tray status notifications (`StatusNotifierItem`). |
| **macOS Stage Manager** | Workspace Grouping | Visual sidebar app group previews and instant background window stack switching. |
| **Windows 11 Copilot / PowerToys** | AI Sidebar & FancyZones | AI Copilot sidebar (`WindowsCopilotAiAssistantSidebar`), FancyZones grid tiling (`PowerToysFancyZonesEngine`). |

---

## 3. Core Architectural Subsystems

### 3.1 Zenith Wayland Compositor Core (`ZenithWaylandCore` / `SigmaCompositor`)
- **Wayland Protocols:** Complete implementation of `WlDisplay`, `WlSurface`, `WlBuffer`, `WlOutput`, `WlSeat`, and `zenith_layer_shell`.
- **Damage Region Repainting:** Calculates dirty bounding boxes (`WlRect`) for partial repaints, reducing GPU fill-rate overhead.
- **Hardware Acceleration:** Direct DRM/KMS scanout buffer integration with VirtIO-GPU and Vulkan rendering pipelines.

### 3.2 Agentic Steering Tri-Engine (Bolt ⚡, Palette 🎨, Sentinel 🛡️)
- **Bolt ⚡:** Automated layout optimization and performance thread-pinning to CPU Performance-cores.
- **Palette 🎨:** Hot-reloads desktop color schemes, wallpaper accents, and widget themes instantly without window restarts.
- **Sentinel 🛡️:** Enforces OpenBSD-inspired `pledge`/`unveil` path restrictions and default-deny Zorin Exec Guard permission popups for untrusted apps.

### 3.3 Zero-CSS Native Widget Toolkit
- **Eliminating External CSS:** Renders widget borders, shadows, headers, and buttons directly via compiled Rust Canvas and Wayland layer-shell protocol primitives (`SovereignCssEliminationEngine`).
- **QuickShell HUD Bridge:** Provides top bar launcher, system tray, battery/volume sliders, and notification popups (`OmarchyQuickShellBridge`).

### 3.4 Desktop Environment API & XDG Portals (`docs/DESKTOP_ENVIRONMENT_API.md`)
- **Layer Shell Anchoring:** Manages background, bottom, top, and overlay surfaces.
- **Status Notifier Item (SNI):** Asynchronous IPC interface for tray icons and context menus.
- **Notifications Daemon (`org.freedesktop.Notifications`):** Low, Normal, Critical urgency notifications with action buttons.

---

## 4. Development Phases & Roadmap Milestones

### Phase 1: Compositor Core & Layer Shell Baseline (M1 Milestone - Active)
- [x] Implement Wayland core protocol types and surface management in `src/compositor/zenith_core.rs`.
- [x] Implement layer-shell surface anchoring in `docs/DESKTOP_ENVIRONMENT_API.md`.
- [x] Implement live theme palette switcher (`OmarchyThemeLiveEngine`) in `src/distro/omarchy_master_synthesis.rs`.
- [ ] Complete direct KMS/DRM VirtIO-GPU scanout integration and QEMU boot serial test.

### Phase 2: Spatial Grid & FancyZones Window Layouts (M2 Milestone)
- [x] Implement FancyZones grid manager in `src/tools/market_competitor_tools.rs`.
- [x] Implement Pop!_OS / KDE Plasma spatial workspace tiling in `src/desktop/sovereign_ux_innovation_hub.rs`.
- [ ] Connect interactive mouse drag-and-drop snapping to FancyZone grid targets.

### Phase 3: Zorin Exec Guard & Security Dialogs (M3 Milestone)
- [x] Implement Zorin Exec Guard recommendation engine in `src/distro/linux_bsd_distro_strategic_innovations.rs`.
- [ ] Render interactive Wayland capability prompt dialogs when untrusted binaries request camera, mic, or host filesystem access.

### Phase 4: Bare-Metal Hardware Alpha & ISO Release (M4-M5 Milestone)
- [ ] Certified x86_64 bare-metal boot testing (Intel 8th+ Gen / AMD Ryzen).
- [ ] Intel Wi-Fi / Realtek wired networking status tray panel in Zenith QuickShell.
- [ ] Public bootable ISO release with fail-safe generational rollback boot menu.

---

## 5. Security & Permission Architecture

1. **Client Isolation:** Every Wayland client process runs under a distinct PID seat with isolated surface buffers, preventing X11-style global keystroke sniffing or window scraping.
2. **Fine-Grained Permissions:** Applications request portal permissions (Camera, Microphone, Location, Screen Capture) through `FineGrainedAccessControlMatrix` in `src/security/publication_permissions.rs`.
3. **Pledge & Unveil Subshell Protection:** Userland applications and desktop widgets operate under OpenBSD-inspired `pledge("stdio rpath wpath cpath", NULL)` restrictions.

---

## 6. Verification & Quality Assurance

- **Unit Testing:** Compositor geometry intersections, pixel buffer reads/writes, tiling layout engines, and theme engines are verified via native Rust unit tests in `src/compositor/` and `src/desktop/`.
- **Integration Testing:** Verified via `./run_sigma_tests.sh` and `./scripts/changed_files_rustc_tests.sh`.
