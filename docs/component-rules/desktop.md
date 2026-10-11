# Component Rules: Desktop & Graphical Interface

## Component owner
desktop-agent

## Upstream references
- Linux Mint Cinnamon / XApps
- Elementary OS Pantheon & Granite HIG
- Wayland / DRM-KMS Compositors

## Purpose
Define integration standards for the Zenith desktop environment, display bridges, and window management.

## Mandatory rules

1. Desktop features must not precede a stable boot and core-runtime path.
2. Every desktop change must define behavior for input loss, application crash, display failure, and shutdown.
3. Graphical claims require a QEMU graphical smoke test or display log evidence.
4. Zenith desktop components must adhere to high-contrast and accessibility fallbacks.
5. Display bridge operations must validate framebuffers and damage regions to avoid buffer overruns.

## Prohibited

- Hardcoding display resolutions without fallback modes
- Silent failure during display server crash or input device disconnect
- Claiming graphical desktop readiness without QEMU frame render evidence
- Prioritizing desktop eye-candy over kernel or installer stability

## Required evidence

- Framebuffer and compositor unit tests (`cargo test --lib`)
- Zenith display bridge integration tests (`./run_sigma_tests.sh`)
- QEMU graphical smoke test log or frame buffer dump

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and GUI widget/compositor spec |
| Prototype | Framebuffer rendering unit tests |
| Integrated | Zenith display bridge initialization in QEMU |
| Hardened | Crash recovery and input device loss handling |
| Supported | Interactive session validation with evidence |
