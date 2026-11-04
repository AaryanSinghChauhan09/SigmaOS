# Component Rules: Desktop

## Component owner
desktop-agent

## Upstream references
- Linux Mint Cinnamon / desktop onboarding flows
- Wayland compositor protocols and direct scanout
- SerenityOS LibGUI and window server design

## Purpose
Define required runtime stability, display recovery, event handling, and validation
for the Zenith desktop environment and GUI compositor.

## Mandatory rules

1. Desktop features must not precede a stable boot and core-runtime path.
2. Every desktop change must define behavior for input loss, application crash, display failure, and shutdown.
3. Graphical claims require a QEMU graphical smoke test.
4. The compositor must isolate crashing client windows without taking down the session.
5. IPC latency for window events and input dispatching must maintain defined frame deadlines.
6. Display output configuration changes must fail gracefully without leaving unusable video modes.
7. System resource exhaustion must prioritize core kernel processes over decorative UI effects.

## Prohibited

- Prioritizing desktop aesthetics over boot-to-shell runtime stability
- Unhandled surface destruction resulting in compositor aborts
- Unvalidated graphical feature claims without screen-capture/log evidence
- Status promotion without QEMU graphical smoke test artifacts

## Required evidence

- Unit test report
- Integration test report
- Graphical smoke test (QEMU screenshot / framebuffer log)
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
