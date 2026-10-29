# Component Rules: Desktop

## Component owner
desktop-agent

## Upstream references
- Wayland / Zenith Compositor
- Linux Mint Cinnamon / XApp ecosystem
- Omarchy / Elementary OS desktop UX

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS desktop components.

## Mandatory rules

1. Desktop features must not precede a stable boot and core-runtime path.
2. Every desktop change must define behavior for input loss, application crash, display failure, and shutdown.
3. Graphical claims require a QEMU graphical smoke test.

## Prohibited

- Prioritizing desktop UI features over kernel boot stability
- Unhandled compositor crashes on display disconnect
- Unsupported graphical parity claims

## Required evidence

- Desktop unit test report
- Input/crash recovery test report
- QEMU graphical smoke test log

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | QEMU graphical session boot |
| Hardened | Display crash recovery & fault tolerance |
| Supported | End-to-end desktop session validation evidence |
