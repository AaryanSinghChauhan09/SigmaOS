# Project Roadmap

**Status: Proposed.** This page summarizes release order and points to the canonical component pages. It is not a statement that planned capabilities exist. Shared status terms are defined in [Future Development](14-Future-Development.md#work-status-vocabulary).

## Current release position

SigmaOS is not ready for general installation. The main branch passes the hosted library check and the repository test runner, but the bare-metal target, boot-to-shell path, installer, recovery process, and desktop session are not verified. See the dated [main-branch test record](Testing.md) for exact results and unavailable checks.

## Ordered milestones

1. **Boot foundation:** separate hosted code from the kernel, define the boot contract, and prove reproducible QEMU boot with serial output. See [Kernel](04-Kernel.md) and [Installation](01-Installation.md).
2. **Minimal runtime:** establish memory, interrupts, input, storage, process execution, syscalls, init, and a shell on one virtual hardware profile.
3. **Persistent system:** validate a filesystem and block path, then safe installation and recovery on disposable virtual disks.
4. **Trust and updates:** add enforceable account boundaries, signed package metadata, transactional installation, update rollback, and offline recovery. See [Security](07-Security.md) and [Packaging](09-Packaging.md).
5. **Usable desktop:** connect display and input backends, then provide accessible, keyboard-first workflows and approachable onboarding inspired by Omarchy and Linux Mint. See [Desktop](08-Desktop.md).
6. **Supported releases:** publish a tested hardware matrix, reproducible artifacts, provenance, known limitations, recovery instructions, and a sustainable security/update policy.

## Release gate

General-use readiness requires a new user to obtain a verified image, install without data loss, create an account, boot to a usable session, use documented supported devices, install software, update and roll back, recover from failure, and receive security fixes under a published support policy. Feature names, source files, hosted unit tests, or a generated image alone do not satisfy this gate.

## Canonical component pages

Each component's status, reference comparisons, evidence, and future work belong on its [canonical component page](00-Home.md). This roadmap records only ordering and system-level release gates.
