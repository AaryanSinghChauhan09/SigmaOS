# PR Specification: SigmaOS Study-to-Implementation Pipeline Governance

## Executive Summary

SigmaOS enforces a strict engineering discipline: **study → extract pattern → implement minimal slice → validate at runtime**. This specification documents the governance standards, study curriculum, implementation rules, ADR format, research directory layout, and 12-week execution schedule for borrowed upstream OS concepts.

---

## Repeatable 10-Step Study-to-Implementation Cycle

For every mechanism borrowed from an upstream project:

1. **SELECT a narrow OS concept**: Choose a single mechanism (e.g. VFS path resolution error handling, scheduler virtual deadline calculation, pledge promise masks) rather than an entire OS domain.
2. **STUDY upstream implementation and documentation**: Examine data structures, concurrency, security assumptions, and error recovery paths.
3. **WRITE a design note for SigmaOS**: Record answers to the 7 canonical study questions in `research/upstream/<project>/`.
4. **DEFINE the smallest testable slice**: Establish the minimal functional API surface required for runtime execution.
5. **IMPLEMENT behind a clear module boundary**: Write safe Rust (`#![no_std] + alloc`) code cleanly decoupled from adjacent systems.
6. **ADD unit, integration, and runtime tests**: Cover happy path, edge cases, and failure modes.
7. **VALIDATE in QEMU or on named hardware**: Execute through actual boot or runtime path; capture serial/kernel logs.
8. **DOCUMENT what was adopted, changed, and rejected**: Author an Architecture Decision Record (ADR) in `research/decisions/`.
9. **ADD regression protection**: Ensure automated tests fail if the behavior regresses.
10. **UPDATE capability status with evidence**: Log runtime evidence and promote maturity rating in `CAPABILITY_MATRIX.toml`.

---

## Reference Systems & Study Deliverables

| Upstream Project | Domain Focus | SigmaOS Application | Required Deliverable |
|---|---|---|---|
| **xv6** | Minimal kernel boot, paging, processes, syscalls, FS | Clean bootloader-to-shell reference path | Annotated boot-to-shell map |
| **Linux Kernel** | VFS, EEVDF scheduler, driver model, namespaces | Production kernel abstractions | Interface comparison matrix |
| **Redox OS** | Safe Rust microkernel, scheme IPC, userspace drivers | Rust architecture and unsafe-code boundaries | Rust architecture note |
| **Arch Linux** | PKGINFO format, SAT solver, pacman-key, transparent config | SigmaPkg format and system configuration | Package/config design note |
| **Linux Mint** | Onboarding UX, pre-flight update checks, Timeshift recovery | Installer recovery and rollback workflows | Recovery workflow |
| **FreeBSD** | Release engineering, jails, ZFS boot environments, network | Tiered platform support, jail isolation | Release checklist |
| **OpenBSD** | Secure defaults, pledge/unveil, W^X, audit culture | Least-privilege process & path sandboxing | Threat model |
| **Debian** | DEB822 stanzas, multi-channel repos, stable discipline | Package metadata parser and repo channels | Release/packaging policy |
| **Fedora** | Greenwave CI gating, atomic rpm-ostree deployments | Test-gated releases and staged updates | CI/release gate design |
| **NixOS** | Pure functional stanzas, generation profiles | Declarative TOML configuration schema | Configuration schema |
| **OSDev** | Bare-metal x86_64, GDT/IDT, 16550 UART, VGA | Bootloader entry and early console trace | QEMU boot trace |

---

## 5 Curriculum Phases & Exit Gates

1. **Phase 1: OS Fundamentals (2 weeks)**
   - *Scope*: Boot path, CPU mode switch, physical/virtual memory allocators, interrupts, timer, scheduler, syscall dispatch, init, shell.
   - *Exit Gate*: QEMU x86_64 boots cleanly to a shell, runs a basic command, and shuts down cleanly with zero panic.

2. **Phase 2: Kernel Services (3 weeks)**
   - *Scope*: EEVDF fair scheduler, trait VFS, process fork/exec/wait/signals, pipes, IPC, stable syscall table.
   - *Exit Gate*: Integration tests confirm process creation, VFS open/read/write, pipe IPC, signal reception, and clean process termination.

3. **Phase 3: Userspace & Distribution (3 weeks)**
   - *Scope*: Minimal shell/coreutils, RAM-disk VFS, SigmaPkg metadata parsing, SAT solver, staged atomic update, Timeshift rollback.
   - *Exit Gate*: QEMU clean install completes, boots, and a deliberately failed update restores the previous pre-update snapshot cleanly.

4. **Phase 4: Security & Isolation (3 weeks)**
   - *Scope*: Capability checks, pledge promise masks, unveil path trees, sandboxed package scripts, signed update manifests, fuzzing.
   - *Exit Gate*: Every unprivileged protected operation is denied, and malicious package scripts fail to escape their declared sandbox.

5. **Phase 5: Desktop & Hardware (4 weeks)**
   - *Scope*: DRM/KMS framebuffer, input handling, Zenith Compositor, VirtIO drivers (virtio-blk, virtio-net, VGA), hardware matrix.
   - *Exit Gate*: QEMU boots to a graphical session, processes input events, launches a windowed app, and records zero kernel panics.

---

## 12 Mandatory Implementation Rules

1. **Rule 1: Study one concept at a time** — Focus strictly on a single mechanism per cycle.
2. **Rule 2: Extract the invariant, not the code** — Record underlying invariants rather than copying source code.
3. **Rule 3: License and attribution first** — Verify dual MIT/GPL-2.0 compatibility before adopting code structures.
4. **Rule 4: Implement the smallest vertical slice** — Connect end-to-end paths (boot → memory → scheduler → syscall → VFS → shell).
5. **Rule 5: Runtime validation is mandatory** — Unit test passes alone do not establish implementation status.
6. **Rule 6: CI gates merges** — Require `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, formatting, and QEMU smoke tests.
7. **Rule 7: Security changes get separate review** — Capabilities, sandboxes, syscalls, and update paths require independent security sign-off.
8. **Rule 8: Reproducibility** — Log commit hashes, toolchain versions, execution timestamps, exit codes, and artifact digests.
9. **Rule 9: Least privilege** — Restrict agent, CI runner, process, and package-script privileges.
10. **Rule 10: Document adoption boundaries** — State adopted, modified, simplified, and rejected upstream aspects.
11. **Rule 11: Regression tests are mandatory** — Add tests that fail whenever behavior regresses.
12. **Rule 12: Human approval for high-risk work** — Require human maintainer approval for boot, storage, security, update, and installer changes.
