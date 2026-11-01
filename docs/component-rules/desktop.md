# Component Rules: Desktop

## Component owner
desktop-agent

## Upstream references
- Linux Mint Cinnamon / Xfce
- Elementary OS Pantheon
- Wayland compositor architecture

## Purpose
Define required reliability, input resilience, crash handling, and validation
for the Zenith desktop environment and display subsystem.

## Mandatory rules

1. Desktop features must not precede a stable boot and core-runtime path.
2. Every desktop change must define behavior for input loss, application crash, display failure, and shutdown.
3. Graphical claims require a QEMU graphical smoke test.

## Prohibited

- Prioritizing desktop eye-candy ahead of core kernel, memory, or VFS stability
- Freezing entire session when a single desktop app or panel crashes
- Making unverified claims about graphical environments without test footage or QEMU logs
- Ungraceful exit that leaves framebuffer or terminal in unusable state

## Required evidence

- Compositor frame-render unit tests
- Input event routing and keyboard/mouse focus recovery tests
- QEMU graphical boot smoke test log / screenshot
- App crash isolation test ensuring compositor survival

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | Desktop UX specification and compositor architecture ADR |
| Prototype | Framebuffer rendering and event loop unit tests |
| Integrated | QEMU graphical boot smoke test |
| Hardened | Client crash isolation and multi-display failover tests |
| Supported | Multi-workstation daily-driver verified desktop environment |
