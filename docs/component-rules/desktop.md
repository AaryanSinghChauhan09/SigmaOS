# Component Rules: Desktop

## Component owner
desktop-agent

## Upstream references
- Wayland compositor protocols & wlroots
- Linux Mint Cinnamon desktop & XApps
- Omarchy Hyprland & Zenith UI guidelines

## Purpose
Ensure desktop extensions follow a stable boot path, define crash handling, and mandate graphical smoke tests.

## Mandatory rules
1. Desktop features must not precede a stable boot and core-runtime path.
2. Every desktop change must define behavior for input loss, application crash, display failure, and shutdown.
3. Graphical claims require a QEMU graphical smoke test.
4. Compositor rendering must support damage region tracking and vsync synchronization.
5. Accessibility standards (WCAG AAA contrast, keyboard navigation) must be enforced across desktop widgets.

## Prohibited
- Desktop features blocking core kernel boot
- Unhandled compositor crashes or display server panics
- Untested graphical parity claims
- Hardcoded screen resolutions without dynamic adapter queries

## Required evidence
- Unit tests for compositor damage tracking and layout logic
- QEMU display smoke test log or screenshot
- Input loss and process crash recovery test log
- Accessibility compliance test output

## Promotion criteria
| Level | Requirement |
|---|---|
| Proposed | Desktop compositor & UI architecture ADR |
| Prototype | Framebuffer / Wayland unit tests passing |
| Integrated | QEMU graphical smoke test and session restoration |
| Hardened | Crash recovery, fuzzing, accessibility validation |
| Supported | Production desktop session stability evidence |
