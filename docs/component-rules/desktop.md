# Component Rules: Desktop

## Component owner
desktop-agent

## Upstream references
- elementary OS Pantheon & Granite
- Linux Mint Cinnamon / XApp ecosystem
- Omarchy Desktop Architecture

## Purpose
Define graphical stability, input loss resilience, and component layering rules for Zenith and desktop environment components.

## Mandatory rules

1. Desktop features must not precede a stable boot and core-runtime path.
2. Every desktop change must define explicit behavior for input loss, application crash, display driver failure, and shutdown.
3. Graphical claims require a QEMU graphical smoke test or framebuffer render evidence.
4. User interface themes and settings must fail gracefully to accessible fallback defaults.
5. Window management and compositor operations must run with constrained user-level capabilities.

## Prohibited

- Desktop developments that bypass kernel boot-to-shell or storage requirements
- Unhandled compositor crashes causing kernel or system-wide panics
- Direct hardware GPU/display access bypassing DRM/KMS abstractions
- Status promotion without QEMU graphical or framebuffer evidence

## Required evidence

- Framebuffer and display bridge unit tests (`src/desktop/zenith_display_bridge.rs`)
- QEMU graphical smoke test recording or screenshot log
- Input failure and window crash recovery test report

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Desktop architecture ADR and HIG specification |
| Prototype | Standalone UI layout and widget unit tests |
| Integrated | QEMU framebuffer render and input handling integration tests |
| Hardened | Crash recovery, display fallback, accessibility audit |
| Supported | Multi-display and hardware display driver evidence |
