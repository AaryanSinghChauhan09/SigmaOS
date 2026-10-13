# Component Rules: Desktop

## Component owner
desktop-agent

## Upstream references
- elementary OS Pantheon & Granite HIG
- Linux Mint Cinnamon & XApps
- Hyprland / Wayland compositors

## Purpose
Define stability boundaries, window management contracts, input safety, and graphical display verification for Zenith Desktop.

## Mandatory rules

1. Desktop features must not precede a stable kernel boot and core-runtime path.
2. Every desktop component must define clear behavior for input loss, application crash, display failure, and clean shutdown.
3. Graphical claims require a QEMU graphical smoke test or frame buffer verification log.
4. Window compositor and display bridge must handle damage region bounds safely without memory overflow.

## Prohibited

- Prioritizing desktop eye-candy over core boot stability
- Unhandled compositor panics on input driver disconnect
- Claiming graphical desktop readiness without QEMU display smoke test evidence

## Required evidence

- Framebuffer bridge unit test report
- Compositor layout and damage calculation test logs
- QEMU graphical display smoke test screenshot or frame buffer log
- Desktop session crash recovery test log

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and HIG specification |
| Prototype | Component unit tests and off-screen rendering validation |
| Integrated | QEMU graphical display boot and interaction smoke test |
| Hardened | Input fault injection and multi-display crash recovery test |
| Supported | Production display driver hardware evidence and user session metrics |
