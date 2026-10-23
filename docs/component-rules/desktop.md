# Component Rules: Desktop

## Component owner
desktop-agent

## Upstream references
- elementary OS Pantheon / Granite / Gala
- Hyprland / Wayland Compositor Ecosystem
- Mint Cinnamon / XApps Suite

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Zenith Desktop, Wayland Compositor, Window Manager, and HIG components.

## Mandatory rules

1. Desktop features must not precede a stable kernel boot and core-runtime path.
2. Every desktop change must define explicit error handling for input loss, application crash, display failure, and clean shutdown.
3. Graphical claims require a QEMU graphical smoke test or framebuffer render test.
4. UI controls and HIG themes must respect system accent colors and dark/light modes cleanly.
5. Compositor damage regions and double buffering must be managed to prevent screen tearing or memory leaks.

## Prohibited

- Claiming desktop usability without kernel boot-to-shell stability
- Unhandled compositor crashes on missing display outputs
- Bypassing Wayland protocol capability checks
- Memory leaks in frame buffer or texture allocation loops
- Status promotion without QEMU graphical smoke test evidence

## Required evidence

- Desktop unit tests (`tests/desktop/` or `elementary_pantheon_innovations.rs`)
- QEMU graphical framebuffer smoke test output
- Zenith display bridge test logs (`src/desktop/zenith_display_bridge.rs`)
- App display failure and input loss fallback test reports

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | HIG specification and desktop component ADR |
| Prototype | Component unit tests & headless rendering |
| Integrated | QEMU graphical framebuffer smoke test + compositor bridge |
| Hardened | Input event stress tests, display disconnect handling |
| Supported | Interactive session boot log and userland UI evidence |
