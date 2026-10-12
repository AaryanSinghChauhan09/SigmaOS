# PR Specification: Bootable Vertical Slices Missing Components Implementation Roadmap for SigmaOS

**PR Title:** `feat(roadmap): Implement Missing Components as Bootable Vertical Slices`
**PR Branch:** `feature/bootable-vertical-slices-roadmap`
**Target Branch:** `main`
**Status:** Proposal / Specification Ready
**Component Scope:** Kernel, Drivers, Userspace, Shell, Packaging, Desktop, Installer, Security, Agent Infrastructure, Documentation

---

## 1. Executive Summary & Problem Statement

SigmaOS must implement missing components as a sequence of **small, bootable vertical slices**, rather than parallel rewrites of every subsystem. While the repository already identifies the necessary OS layers—kernel, drivers, userspace, shell, packaging, desktop, installer, and agent infrastructure—its wiki and support matrix show that boot, runtime integration, hardware validation, and installer/update recovery remain unproven in real hardware and QEMU environments.

This PR specification establishes the non-negotiable methodology for implementing and validating missing OS components using small, bootable vertical slices backed by evidence-gated verification.

---

## 2. Implementation Philosophy

Use the discipline of mature Linux and BSD distributions: first make a minimal system boot and run, then make it installable and recoverable, then broaden hardware and application support.

| Distro/BSD Lesson | SigmaOS Implementation Rule |
|-------------------|-----------------------------|
| **Debian/Ubuntu** installer-first approach | Build a minimal bootable image before broad desktop features. |
| **Arch** explicit configuration | Every subsystem has a declared interface, owner, and test. |
| **Fedora** CI gating | CI blocks regressions rather than merely reporting them. |
| **FreeBSD** release engineering | Every release has known issues, rollback, and validation evidence. |
| **OpenBSD** least privilege | Security boundaries are designed before convenience features. |
| **NixOS** declarative model | System configuration, dependencies, and tests are explicit. |
| **Linux** hardware enablement | Hardware support is proven per device, not claimed generally. |

---

## 3. Component Implementation Order

Implement in strict sequence. Each stage must produce a runnable artifact and test evidence before the next milestone begins.

```text
M0: Reproducible build and test foundation
M1: Bootable kernel and shell
M2: Core userspace and filesystem
M3: Install, update, and recovery
M4: Security and isolation
M5: Desktop and applications
M6: Package ecosystem and compatibility
M7: Hardware enablement
M8: Agent-native automation
```

---

## 4. Detailed Milestone Specifications

### M0: Build and Test Foundation
**Missing capability:** A reliable, reproducible way to prove that the repository builds and passes its own checks.

- Deterministic build container with pinned Rust toolchain, QEMU version, dependencies, and build scripts.
- `make verify` target that runs formatting, linting, unit tests, integration tests, and artifact creation.
- CI pipeline that blocks merge on required failures.
- Build manifest containing commit hash, toolchain versions, container digest, commands, timestamps, and artifact hashes.
- Report directory for build, test, boot, installer, package, and security evidence.

*(Baseline checks: `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, `make format`, `make clean`).*

---

### M1: Bootable Kernel and Shell
**Missing capability:** A proven firmware-to-shell path.

| Component | Minimum Implementation |
|-----------|------------------------|
| **Bootloader integration** | UEFI first, then BIOS; load kernel and initrd. |
| **Kernel entry** | Early console, CPU init, memory map parsing, stack setup. |
| **Physical memory manager** | Detect usable regions, allocate/free frames. |
| **Virtual memory** | Kernel page tables, identity mapping, user/address-space foundation. |
| **Interrupts** | IDT, exception handlers, timer, keyboard input. |
| **Scheduler** | Cooperative or simple round-robin scheduler. |
| **Syscall interface** | Minimal stable syscall table and error conventions. |
| **Init process** | PID 1, service startup, graceful shutdown. |
| **Shell** | Execute built-ins and simple external commands. |
| **Serial logging** | Persistent boot diagnostics and failure-stage reporting. |

---

### M2: Core Userspace and Filesystem
**Missing capability:** A minimal but real userspace environment in which processes, files, and basic OS services work end to end.

- VFS abstraction with mount points, path resolution, file descriptors, and standard error codes.
- RAM-disk filesystem first; persistent filesystem after VFS semantics are stable.
- Process creation, exit, wait, and signal handling.
- Pipes, standard I/O streams, and basic IPC.
- Core userland utilities: `sh`, `ls`, `cat`, `echo`, `mkdir`, `rm`, `cp`, `ps`, `kill`, `reboot`.
- Service manager with start/stop/status/restart policies and dependency ordering.

---

### M3: Install, Update, and Recovery
**Missing capability:** A trustworthy install/update/recovery path.

- **Installer:** Disk detection, partitioning, filesystem creation, base installation, bootloader install, verification, install log.
- **Update System:** Atomic image/transaction updates, update manifest with hashes, pre-update health checks, staged activation, automatic rollback.
- **Recovery:** Recovery boot entry, shell access without desktop dependencies, filesystem repair hooks, generation rollback, backup/restore.

---

### M4: Security and Isolation
**Missing capability:** Enforceable security boundaries.

- Capability model for filesystem, network, process, IPC, device, package execution.
- Process isolation with syscall and resource limits.
- Sandboxed package script execution.
- Signed update manifest verification.
- QEMU measured-boot test hooks.
- Syscall, path, IPC, package, and network fuzzing.

---

### M5: Desktop and Applications
**Missing capability:** A minimal desktop path built on a stable runtime.

1. Framebuffer compositor & display server foundation.
2. Window management (create, focus, move, resize, close).
3. Graphical app launcher.
4. Session management (login, logout, lock, shutdown).
5. Diagnostics and crash reporting.
6. Zenith desktop workflow only after base desktop is stable.

---

### M6: Package Ecosystem and Compatibility
**Missing capability:** A safe package system and realistic Linux/BSD compatibility strategy.

- **Phase 1:** Package metadata schema and parser (Parse only).
- **Phase 2:** Dependency resolver (Dry-run only).
- **Phase 3:** Package installation into isolated image (No host execution).
- **Phase 4:** Sandboxed script execution (Explicit capabilities).
- **Phase 5:** Repository signing and verification.
- **Phase 6:** Linux/BSD ABI/metadata compatibility layer.

---

### M7: Hardware Enablement
**Missing capability:** Validated device support.

- **Tier 1:** Virtual devices required for CI (QEMU x86_64, virtio-blk, virtio-net, VGA).
- **Tier 2:** Common peripherals (USB keyboard, mouse, storage, basic audio).
- **Tier 3:** Laptop/desktop hardware (Wi-Fi, GPU, audio, suspend/resume).
- **Tier 4:** Advanced features (GPU acceleration, multi-monitor, power management).

---

### M8: Agent-Native Automation
**Missing capability:** A safe, repeatable agent layer.

- Agent registry, task queue, and runbooks.
- Agent CLI writing structured evidence to `reports/`.
- Capability auditor and persistent boot matrix validator.
- Security agent reviewing diffs.
- Mandatory human approval workflow for high-risk operations.

---

## 5. Industry-Standard Implementation Rules

1. **Vertical slices first**
2. **Interface contracts before implementation**
3. **Evidence-gated completion**
4. **CI is mandatory**
5. **Reproducibility**
6. **Fail safely**
7. **Least privilege**
8. **Security review is separate**
9. **Regression tests are mandatory**
10. **No borrowed-status claims**
11. **One owner per component**
12. **Human approval for destructive/high-risk operations**

---

## 6. Changed & Added Files

- `PR_PROPOSAL_BOOTABLE_VERTICAL_SLICES_MISSING_COMPONENTS.md`: Master PR proposal document.
- `docs/roadmap/PR_SPEC_BOOTABLE_VERTICAL_SLICES_MISSING_COMPONENTS.md`: This PR specification document.
- `src/distro/bootable_vertical_slices_missing_components_pr.rs`: Rust PR proposal gateway engine implementation.

---

## 7. Verification and Compliance

- Validated with `cargo check --lib`, `./run_sigma_tests.sh`, and `make verify`.
- Full alignment with repository roadmap guidelines.
