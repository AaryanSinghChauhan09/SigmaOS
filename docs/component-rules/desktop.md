# Component Rules: Desktop

## Component owner
desktop-agent

## Upstream references
- Linux Mint Cinnamon / GTK ecosystem
- elementary OS Pantheon & Granite HIG
- Hyprland / Wayland compositor architecture

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS Zenith Desktop components.

## Mandatory rules

1. Desktop features must not precede a stable boot and core-runtime path.
2. Every desktop change must define behavior for input loss, application crash, display failure, and shutdown.
3. Graphical claims require a QEMU graphical smoke test.
4. User interface components must fail gracefully without crashing core graphics services.

## Prohibited

- Desktop development blocking kernel/VFS/boot validation work
- Silent crashes or unhandled display server disconnects
- Unsupported desktop parity claims without graphical evidence
- Status promotion without QEMU GUI verification

## Required evidence

- Desktop unit test report
- Input/display failure recovery test log
- QEMU graphical boot & display smoke test output
- Interface regression test suite log

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
