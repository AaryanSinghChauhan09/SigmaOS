# Component Rules: Desktop

## Component owner
desktop-agent

## Upstream references
- Linux Mint Cinnamon / XApp
- GNOME Shell / Mutter
- COSMIC desktop / libcosmic

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Zenith desktop environment, display manager, and user sessions.

## Mandatory rules

1. Desktop features must not precede a stable boot and core-runtime path.
2. Every desktop change must define behavior for input loss, application crash, display failure, and shutdown.
3. Graphical claims require a QEMU graphical smoke test.

## Prohibited

- Prioritizing desktop components over boot, memory, or core-runtime stability
- Crashing the whole session on individual application failure
- Unhandled compositor termination leaving input devices captured
- Promoting desktop status without QEMU graphical test logs

## Required evidence

- Compositor and widget logic unit tests
- Input device disconnect / crash recovery tests
- QEMU graphical smoke test recording or framebuffer log
- Regression test for display and session crashes

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests and headless compositor rendering |
| Integrated | QEMU graphical smoke test and session initialization |
| Hardened | Multi-display, input fuzzing, crash recovery verification |
| Supported | Reproducible desktop session launch and user interaction evidence |
