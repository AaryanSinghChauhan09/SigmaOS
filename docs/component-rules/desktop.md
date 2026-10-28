# Component Rules: Desktop

## Component owner
desktop-agent

## Upstream references
- Linux Mint Cinnamon / XApp
- elementary OS Pantheon
- Omarchy / Wayland desktop stack

## Purpose
Define graphical desktop stability, input failure resilience, and UI session lifecycle
for SigmaOS Desktop components.

## Mandatory rules

1. Desktop features must not precede a stable boot and core-runtime path.
2. Every desktop change must define behavior for input loss, application crash, display failure, and shutdown.
3. Graphical claims require a QEMU graphical smoke test.

## Prohibited

- Adding UI features while core boot or system paths are unstable
- Silent crashes on display or compositor failure
- Unsupported GUI parity claims without visual or smoke test evidence
- Status promotion without QEMU graphical smoke test logs

## Required evidence

- Unit test report
- Input loss & app crash recovery log
- QEMU graphical smoke test log
- Regression test for desktop session crashes

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
