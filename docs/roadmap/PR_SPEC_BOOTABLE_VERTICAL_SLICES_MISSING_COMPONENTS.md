# PR Proposal: Implement Missing Components as Bootable Vertical Slices

**PR Title:** `feat(roadmap): Implement Missing Components as Bootable Vertical Slices`
**PR Branch:** `feature/bootable-vertical-slices-roadmap`
**Status:** Approved / Specification Ready
**Patch Digest:** `0xcbf29ce484222325`

---

## Executive Summary

SigmaOS implements missing OS components as a sequence of **small, bootable vertical slices**, rather than attempting parallel rewrites across every subsystem simultaneously. While the repository identifies the target OS layers—kernel, drivers, userspace, shell, packaging, desktop, installer, and agent infrastructure—this specification enforces an evidence-gated milestone sequence (M0 through M8) with non-negotiable acceptance criteria at every tier.

---

## Implementation Philosophy

Adopting the discipline of mature Linux/BSD distributions, SigmaOS adheres to the principle: **first make a minimal system boot and run, then make it installable and recoverable, then broaden hardware and application support.**

| Distro/BSD Lesson | SigmaOS Implementation Rule |
|---|---|
| **Debian/Ubuntu** installer-first approach | Build a minimal bootable image before broad desktop features |
| **Arch Linux** explicit configuration | Every subsystem has a declared interface, owner, and test |
| **Fedora** CI gating | CI blocks regressions rather than merely reporting them |
| **FreeBSD** release engineering | Every release has known issues, rollback, and validation evidence |
| **OpenBSD** least privilege | Security boundaries are designed before convenience features |
| **NixOS** declarative model | System configuration, dependencies, and tests are explicit |
| **Linux** hardware enablement | Hardware support is proven per device, not claimed generally |

---

## Component Implementation Order

Milestones must be completed sequentially. Each stage must produce a runnable artifact and test evidence before the next begins.

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
- **Missing Capability:** A reliable, reproducible way to prove that the repository builds and passes its own checks.
- **Implement:**
  - Deterministic build container with pinned Rust toolchain, QEMU version, dependencies, and build scripts.
  - A `make verify` target executing formatting, linting, unit tests, integration tests, and artifact creation.
  - CI pipeline blocking merges on failure.
  - Build manifest detailing commit hash, toolchain versions, container digest, commands, timestamps, and artifact hashes.
  - Structured report directory (`reports/`) for build, test, boot, installer, package, and security evidence.
- **Acceptance Criteria:**
  - `make verify` succeeds from a clean checkout.
  - CI runs on every PR.
  - Every artifact can be reproduced from its manifest.
  - Test failures generate actionable diagnostic reports.

### M1: Bootable Kernel and Shell
- **Missing Capability:** A proven firmware-to-shell path in QEMU UEFI/BIOS.
- **Minimum Boot Contract:** `UEFI/BIOS` -> `bootloader` -> `kernel entry` -> `memory management` -> `interrupts/timer` -> `scheduler` -> `init` -> `shell prompt` -> `command execution` -> `clean shutdown`.
- **Implement:**
  - Bootloader integration (UEFI first, then BIOS) loading kernel and initrd.
  - Kernel entry: early console, CPU init, memory map parsing, stack setup.
  - Physical memory manager (detect usable regions, frame allocator).
  - Virtual memory (kernel page tables, identity mapping, user address space foundation).
  - Interrupt handling (IDT, exception handlers, timer ticks, keyboard input).
  - Simple round-robin or cooperative scheduler.
  - Minimal syscall interface with standard error codes.
  - Init process (PID 1, service startup, graceful shutdown).
  - Shell executing built-ins and external commands.
  - Persistent serial diagnostic logging.
- **Acceptance Criteria:**
  - QEMU x86_64 UEFI boot reaches shell prompt.
  - BIOS boot reaches shell prompt.
  - Serial log captures each boot stage.
  - Smoke command executes successfully.
  - Panics or failed init generate classified diagnostic reports.

### M2: Core Userspace and Filesystem
- **Missing Capability:** Minimal userspace environment where processes, VFS, and basic OS services work end-to-end.
- **Implement:**
  - VFS abstraction with mount points, path resolution, file descriptors, and standard error codes.
  - RAM-disk/in-memory filesystem before persistent storage.
  - Process creation, exit, wait, and basic signal handling.
  - Pipes, stdio redirection, and basic IPC.
  - Core utilities: `sh`, `ls`, `cat`, `echo`, `mkdir`, `rm`, `cp`, `ps`, `kill`, `reboot`.
  - Service manager with dependency ordering and restart policies.
- **Acceptance Criteria:**
  - Process creation, program execution, file I/O, and graceful exit verified.
  - Redirection and pipes functional.
  - Filesystem conformance tests pass.
  - Init starts services in dependency order.

### M3: Install, Update, and Recovery
- **Missing Capability:** Trustworthy install/update/recovery path with atomic rollback.
- **Implement:**
  - **Installer:** Disk detection, GPT/partitioning, FS creation, base install, bootloader config, post-install verification, install logs.
  - **Update System:** Transactional image updates, manifest verification, health checks, staged activation, auto-rollback on failure, update journal.
  - **Recovery Mode:** Minimal recovery boot entry, rescue shell, FS check/repair, generation rollback, backup/restore.
- **Acceptance Criteria:**
  - QEMU clean installation completes without manual intervention.
  - Corrupted or failed updates restore prior generation automatically.
  - Recovery mode successfully repairs or rolls back system.

### M4: Security and Isolation
- **Missing Capability:** Enforceable security capabilities and sandboxing boundaries.
- **Implement:**
  - Capability model for FS, process, IPC, network, device, and package execution.
  - Process isolation with restricted syscall access.
  - Sandboxed package script execution.
  - Cryptographically signed update manifests.
  - Fuzzing for syscall handlers, paths, IPC, and package parsers.
  - Independent security review workflow.
- **Acceptance Criteria:**
  - Protected operations fail without explicit capabilities.
  - Untrusted scripts cannot escape sandbox.
  - Threat model documented for local users, compromised processes, malicious packages, and network vectors.

### M5: Desktop and Applications
- **Missing Capability:** Minimal display compositor and GUI app session.
- **Implement (in order):**
  1. Display server / compositor foundation (framebuffer output, surfaces, input handling).
  2. Window management (create, focus, move, resize, close).
  3. App launcher & session management (login, lock, logout, shutdown).
  4. Zenith workflow (palette, workspaces) once base desktop is stable.
- **Acceptance Criteria:**
  - QEMU boots to graphical session.
  - Pointer and keyboard input functional.
  - App crash does not crash compositor or kernel.

### M6: Package Ecosystem and Compatibility
- **Missing Capability:** Safe sandboxed package installation and multi-distro metadata compatibility.
- **Phases:** Metadata parsing -> Dry-run dependency resolution -> Isolated image install -> Sandboxed script execution -> Signed repository verification -> ABI compatibility layer.
- **Acceptance Criteria:**
  - Package parsed, resolved, installed into test image, and rolled back safely.
  - Malformed packages fail gracefully without system corruption.

### M7: Hardware Enablement
- **Missing Capability:** Validated device support across QEMU tier 1 to physical hardware.
- **Tiers:** Tier 1 (QEMU virtio/VGA) -> Tier 2 (USB input/storage, audio) -> Tier 3 (Wi-Fi, GPU, power management) -> Tier 4 (Acceleration, external displays).
- **Acceptance Criteria:**
  - Tier 1 QEMU parity achieved before ARM64/bare-metal expansion.
  - Device test matrix logged per supported device.

### M8: Agent-Native Automation
- **Missing Capability:** Safe, repeatable agent layer with restricted permission matrix.
- **Implement:** Agent registry, task queue (`open`, `in-progress`, `done`, `blocked`), task runbooks, agent CLI with structured evidence output, boot matrix validator, capability auditor, and mandatory human approval workflow for high-risk operations.

---

## 12 Industry-Standard Implementation Rules

1. **Vertical Slices First:** Complete end-to-end slice before broadening breadth.
2. **Interface Contracts First:** Declare public interfaces, error behavior, and tests before implementation.
3. **Evidence-Gated Completion:** Component is complete only with code, tests, runtime evidence, and docs.
4. **CI is Mandatory:** Automated checks block regressions.
5. **Reproducibility:** All builds/tests repeatable from recorded inputs.
6. **Fail Safely:** Rollback and recovery defined before merge.
7. **Least Privilege:** Strict permission boundaries for processes, packages, and agents.
8. **Separate Security Review:** Dedicated review for syscalls, capabilities, sandboxing, and updates.
9. **Mandatory Regression Tests:** Every fix includes a failing-test safeguard.
10. **No Borrowed-Status Claims:** Reference projects inspire design but cannot count as SigmaOS capabilities without evidence.
11. **Single Component Owner:** One owner/agent, canonical documentation, and test suite per component.
12. **Human Approval for Destructive Operations:** Mandatory human authorization for bootloader, installer, security, and release modifications.

---

## Gap-to-Component Map

| Missing Component | Depends On | First Deliverable | Proof Standard |
|---|---|---|---|
| Bootloader integration | Build system | UEFI QEMU boot | Serial log |
| Kernel memory management | Bootloader | Frame allocator and page tables | Boot test |
| Interrupts and timer | Kernel entry | Timer ticks and keyboard input | Event log |
| Scheduler | Interrupts | Two processes alternate | Scheduler trace |
| Syscalls | Scheduler | Minimal syscall table | Syscall test |
| VFS | Syscalls | File open/read/write/close | Filesystem test |
| Init | Syscalls | PID 1 starts shell | Boot-to-shell |
| Installer | Bootable image | Clean QEMU install | Install log |
| Update/rollback | Installer | Failed update restores prior state | Recovery test |
| Security capabilities | Syscalls/processes | Denial tests | Security report |
| Desktop | Stable boot/runtime | Graphical session | QEMU UI smoke test |
| Packages | Filesystem/security | Metadata parse and dry-run | Package test |
| Hardware support | QEMU parity | Device-specific test matrix | Hardware logs |

---

## 90-Day Implementation Roadmap

- **Days 1–14 (Foundation):** Build container, `make verify`, CI gates, component ownership, audit claims.
- **Days 15–35 (Bootable Core):** UEFI QEMU boot, console, memory, interrupts, scheduler, init, shell, boot-to-shell smoke tests.
- **Days 36–55 (Usable Base System):** VFS, processes, pipes, signals, core userland utilities, service manager, graceful shutdown.
- **Days 56–75 (Install & Recover):** QEMU clean install, atomic updates, auto-rollback, recovery mode.
- **Days 76–90 (Security & First Desktop Slice):** Security capabilities, sandboxing, minimal GUI compositor session, nightly evidence-gated release.

---

## Definition of "Missing Component Implemented" (10-Point Checklist)

1. Named owner and canonical documentation page exist.
2. Public interface and error contracts declared.
3. Connected directly to real boot/runtime path.
4. Unit and integration tests pass.
5. QEMU or hardware runtime evidence recorded.
6. Automated regression test included.
7. Security impact reviewed.
8. Rollback and recovery defined.
9. Status and evidence links updated.
10. Included in reproducible build/release artifact.
