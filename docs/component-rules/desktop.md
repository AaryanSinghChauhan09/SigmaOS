# Component Rules: Desktop

## Component owner
desktop-agent

## Upstream references
- Elementary OS Pantheon & Granite HIG
- Linux Mint Cinnamon
- Wayland compositor specs

## Purpose
Define required desktop user session behavior, window management, and display server safety rules.

## Mandatory rules

1. Desktop features must not precede a stable boot and core-runtime path.
2. Every desktop change must define behavior for input loss, application crash, display failure, and shutdown.
3. Graphical claims require a QEMU graphical smoke test.
4. User session components must communicate via authenticated IPC channels.
5. High DPI and theme settings must fail gracefully to default system values.

## Prohibited

- Premature desktop feature status claims without lower-level runtime support
- Unhandled graphical compositor crashes that hang the system
- Direct display memory access bypassing the display server abstraction
- Unresponsive UI loops during background operations

## Required evidence

- Desktop component unit tests
- QEMU graphical display smoke test screenshot or log
- Session recovery on application crash report
- HIG theme compliance test report

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and desktop session architecture design |
| Prototype | Unit tests for window manager and UI controls |
| Integrated | QEMU graphical session boot-to-desktop smoke test |
| Hardened | Input loss and crash recovery testing |
| Supported | Multi-monitor, session lock, and multi-user verification |
