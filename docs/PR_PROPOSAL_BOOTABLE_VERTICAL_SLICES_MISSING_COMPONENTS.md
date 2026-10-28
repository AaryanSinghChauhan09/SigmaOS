# Pull Request: Implement Missing Components as Bootable Vertical Slices

**PR Branch:** `feature/bootable-vertical-slices-roadmap`
**PR Title:** `feat(roadmap): Implement Missing Components as Bootable Vertical Slices`
**Status:** Approved / Specification Ready

---

## Executive Summary

SigmaOS shall implement missing components as a sequence of **small, bootable vertical slices**, rather than attempting parallel rewrites of every subsystem simultaneously. The repository already identifies the needed OS layers—kernel, drivers, userspace, shell, packaging, desktop, installer, and agent infrastructure—but boot, runtime integration, hardware validation, and installer/update recovery must be proven sequentially.

---

## Implementation Philosophy

Using the discipline of mature Linux/BSD distributions, SigmaOS prioritizes making a minimal system boot and run first, then making it installable and recoverable, and finally broadening hardware and application support.

| Distro/BSD Lesson | SigmaOS Implementation Rule |
|---|---|
| **Debian/Ubuntu** installer-first approach | Build a minimal bootable image before broad desktop features |
| **Arch** explicit configuration | Every subsystem has a declared interface, owner, and test |
| **Fedora** CI gating | CI blocks regressions rather than merely reporting them |
| **FreeBSD** release engineering | Every release has known issues, rollback, and validation evidence |
| **OpenBSD** least privilege | Security boundaries are designed before convenience features |
| **NixOS** declarative model | System configuration, dependencies, and tests are explicit |
| **Linux** hardware enablement | Hardware support is proven per device, not claimed generally |

---

## Component Implementation Order (Milestones M0–M8)

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

## Detailed Milestone Specifications

### M0: Build and Test Foundation
- **Missing capability:** Reliable, reproducible build and test verification.
- **Implement:**
  - Deterministic build container with pinned toolchains.
  - `make verify` target for format, lint, unit test, integration test, and artifact creation.
  - CI gating pipeline blocking regressions.
  - Build manifest recording commit hash, toolchain version, timestamps, and hashes.
  - Report directory (`reports/`) for build, test, boot, installer, and security evidence.
- **Acceptance criteria:** `make verify` succeeds from clean checkout; CI runs on every PR; build artifacts are 100% reproducible.

### M1: Bootable Kernel and Shell
- **Missing capability:** Proven firmware-to-shell boot path in QEMU UEFI/BIOS.
- **Implement:**
  - Bootloader integration (UEFI first, then BIOS).
  - Early console, CPU init, memory map parsing, IDT, timer/keyboard interrupts.
  - Physical frame allocator & kernel virtual memory mapping.
  - Simple round-robin scheduler & minimal syscall table.
  - Init process (PID 1) launching shell prompt with serial diagnostic logging.
- **Acceptance criteria:** QEMU x86_64 UEFI and BIOS boot reach shell prompt; smoke commands execute cleanly; serial log captures stage transitions.

### M2: Core Userspace and Filesystem
- **Missing capability:** Minimal userspace environment where processes, files, and core services function end-to-end.
- **Implement:**
  - VFS abstraction with mount points, path resolution, FDs, and POSIX error codes.
  - RAM-disk / in-memory filesystem initial slice.
  - Process creation (`fork`/`exec`), signals, pipes, stdio.
  - Core utilities (`sh`, `ls`, `cat`, `echo`, `mkdir`, `rm`, `cp`, `ps`, `kill`, `reboot`).
  - Init service manager with startup/shutdown dependency ordering.
- **Acceptance criteria:** Processes create, execute, write files, and exit; shell pipes function; init service crashes do not prevent graceful system shutdown.

### M3: Install, Update, and Recovery
- **Missing capability:** Trustworthy installation, atomic update, and emergency recovery path.
- **Implement:**
  - Disk partitioning and filesystem creation in installer.
  - Atomic image or transaction-based updates with pre-update health checks.
  - Automatic rollback on failed boot or verification.
  - Recovery boot entry with standalone diagnostic shell and snapshot restore hooks.
- **Acceptance criteria:** Clean QEMU installation completes without manual intervention; failed updates restore previous generation; recovery shell repairs corrupted states.

### M4: Security and Isolation
- **Missing capability:** Enforceable capability boundaries and execution sandboxes.
- **Implement:**
  - Capability model for FS, process, network, IPC, and device access.
  - Sandboxed package scriptlet execution.
  - Cryptographically signed update manifests and Secure Boot roadmap hooks.
  - Syzkaller/fuzzing for syscall handlers, VFS, and package parsers.
  - Separate security review workflow.
- **Acceptance criteria:** Operations missing required capabilities fail closed; untrusted scripts cannot breach sandbox boundaries; documented threat model covers local, network, and update attack vectors.

### M5: Desktop and Applications
- **Missing capability:** Minimal display compositor and graphical session runtime.
- **Implement:**
  - Framebuffer display server/compositor foundation and input event handling.
  - Window management (create, focus, move, resize, close).
  - Graphical app launch from shell or palette.
  - Session manager (login, lock, logout, shutdown).
- **Acceptance criteria:** QEMU boot reaches graphical desktop; pointer/keyboard function; crashed GUI app does not crash compositor or kernel.

### M6: Package Ecosystem and Compatibility
- **Missing capability:** Safe package installation and multi-distro format parsing.
- **Implement:**
  - Package metadata schema parser (`.deb`, `.rpm`, Arch, BSD) without unsafe execution.
  - Dry-run dependency resolver.
  - Isolated chroot/image installation into sandbox.
  - Repository PQC signature verification.
- **Acceptance criteria:** Package metadata parsed and installed into test container; malformed packages fail safely without altering host state.

### M7: Hardware Enablement
- **Missing capability:** Validated physical and virtual device support matrix.
- **Implement:**
  - Tier 1: Virtual devices for CI (QEMU x86_64, virtio-blk, virtio-net, VGA).
  - Tier 2: Standard peripherals (USB keyboard, mouse, storage, basic audio).
  - Tier 3/4: Physical laptop/desktop GPUs, Wi-Fi, power management.
- **Acceptance criteria:** QEMU x86_64 parity reached before physical hardware expansion; every device claim backed by recorded test logs.

### M8: Agent-Native Automation
- **Missing capability:** Safe, permission-bounded AI agent layer for automated testing and releases.
- **Implement:**
  - Agent registry (`.agents/registry.toml`) with strict read/write/test permission matrix.
  - Task queue (`open`, `in-progress`, `done`, `blocked`).
  - Automated boot validator and capability auditor.
  - Human approval workflow for high-risk operations (kernel, boot, installer, security boundaries).
- **Acceptance criteria:** Agents execute within least-privilege boundaries; all automated tasks produce structured reports in `reports/`.

---

## 12 Industry-Standard Implementation Rules

1. **Vertical Slices First:** Complete one end-to-end path before expanding breadth.
2. **Interface Contracts Before Implementation:** Declare public interfaces, error codes, and test contracts prior to code.
3. **Evidence-Gated Completion:** Require implementation, tests, runtime evidence, and docs for completion.
4. **CI is Mandatory:** Block merge on clean build, unit tests, integration tests, and boot smoke tests.
5. **Reproducibility:** Every build and test input must be pinned and repeatable.
6. **Fail Safely:** Define rollback and recovery mechanisms before merging high-risk components.
7. **Least Privilege:** Enforce minimum permission matrix across packages, services, and AI agents.
8. **Security Review is Separate:** Require dedicated security review for capabilities, syscalls, and update channels.
9. **Regression Tests are Mandatory:** Include failing test cases for every bug fix or feature.
10. **No Borrowed-Status Claims:** Reference-project capabilities cannot be claimed until proven in SigmaOS runtime.
11. **One Owner Per Component:** Assign one named owner, documentation page, test suite, and status field per component.
12. **Human Approval for High-Risk Operations:** Require explicit human approval for destructive operations and releases.

---

## Gap-to-Component Map

| Missing Component | Depends On | First Deliverable | Proof |
|---|---|---|---|
| Bootloader integration | Build system | UEFI QEMU boot | Serial log |
| Kernel memory management | Bootloader | Frame allocator & page tables | Boot test |
| Interrupts & timer | Kernel entry | Timer ticks & keyboard input | Event log |
| Scheduler | Interrupts | Two processes alternate | Scheduler trace |
| Syscalls | Scheduler | Minimal syscall table | Syscall test |
| VFS | Syscalls | File open/read/write/close | Filesystem test |
| Init | Syscalls | PID 1 starts shell | Boot-to-shell |
| Installer | Bootable image | Clean QEMU install | Install log |
| Update/rollback | Installer | Failed update restores prior state | Recovery test |
| Security capabilities | Syscalls/processes | Denial tests | Security report |
| Desktop | Stable boot/runtime | Graphical session | QEMU UI smoke test |
| Packages | Filesystem/security | Metadata parse & dry-run | Package test |
| Hardware support | QEMU parity | Device test matrix | Hardware logs |

---

## 90-Day Implementation Roadmap

- **Days 1–14 (Foundation):** Build container, `make verify`, CI gates, evidence reporting, claim classification audit.
- **Days 15–35 (Bootable Core):** UEFI QEMU boot, early console, PMM/VMM, IDT, scheduler, init, shell, smoke tests.
- **Days 36–55 (Usable Base System):** VFS, process lifecycle, pipes, signals, core utilities, service manager, shutdown.
- **Days 56–75 (Install and Recover):** QEMU installer, atomic updates, generation rollback, emergency recovery shell.
- **Days 76–90 (Security & First Desktop Slice):** Capabilities, sandboxing, security denial tests, minimal Wayland GUI session.

---

## Definition of "Missing Component Implemented" (10-Point Checklist)

1. Named owner and canonical documentation page exist.
2. Public interface and error conventions are declared.
3. Connected to actual boot/runtime execution path.
4. Unit tests pass cleanly.
5. Integration tests pass cleanly.
6. QEMU or bare-metal runtime evidence exists in `reports/`.
7. Automated regression test added.
8. Dedicated security impact review completed.
9. Rollback and recovery behaviors defined.
10. Included in reproducible build manifest.
