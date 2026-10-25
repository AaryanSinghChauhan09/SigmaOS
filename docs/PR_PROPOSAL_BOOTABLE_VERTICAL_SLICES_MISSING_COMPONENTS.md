# Pull Request Proposal: Implement Missing Components as Bootable Vertical Slices

**PR Title:** `feat(roadmap): Implement Missing Components as Bootable Vertical Slices`
**PR Branch:** `feature/bootable-vertical-slices-roadmap`
**Author:** Sovereign Maintainer Core Team
**Category:** Architecture / Distro Roadmap / Engineering Strategy
**Milestones Covered:** M0 – M8 (9 Vertical Slice Milestones)
**Implementation Rules:** 12 Industry-Standard Rules
**Status:** Approved / Specification Ready

---

## Executive Summary & Implementation Philosophy

SigmaOS must implement missing components as a sequence of **small, bootable vertical slices**, rather than attempting parallel rewrites of every subsystem simultaneously. The repository already identifies the required OS layers—kernel, drivers, userspace, shell, packaging, desktop, installer, and agent infrastructure—but its support matrix shows that boot, runtime integration, hardware validation, and installer/update recovery require evidence-gated verification.

Adopting the proven engineering discipline of mature Linux and BSD distributions:

| Distro/BSD lesson | SigmaOS implementation rule |
|---|---|
| **Debian/Ubuntu installer-first approach** | Build a minimal bootable image before broad desktop features |
| **Arch’s explicit configuration** | Every subsystem has a declared interface, owner, and test |
| **Fedora CI gating** | CI blocks regressions rather than merely reporting them |
| **FreeBSD release engineering** | Every release has known issues, rollback, and validation evidence |
| **OpenBSD least privilege** | Security boundaries are designed before convenience features |
| **NixOS declarative model** | System configuration, dependencies, and tests are explicit |
| **Linux hardware enablement** | Hardware support is proven per device, not claimed generally |

---

## Component Implementation Order

Implement in this order. Each stage must produce a runnable artifact and test evidence before the next begins.

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

## Milestone Specifications

### M0: Build and Test Foundation

**Missing capability:** A reliable, reproducible way to prove that the repository builds and passes its own checks.

**Implement:**
- A deterministic build container with pinned Rust toolchain, QEMU version, dependencies, and build scripts.
- A `make verify` target that runs formatting, linting, unit tests, integration tests, and artifact creation.
- A CI pipeline that blocks merge on required failures.
- A build manifest containing commit hash, toolchain versions, container digest, commands, timestamps, and artifact hashes.
- A report directory for build, test, boot, installer, package, and security evidence.

**Acceptance criteria:**
- `make verify` succeeds from a clean checkout.
- CI runs on every PR.
- Every artifact can be reproduced from its manifest.
- Test failures create actionable reports, not silent logs.

---

### M1: Bootable Kernel and Shell

**Missing capability:** A proven firmware-to-shell path in QEMU x86_64 UEFI/BIOS.

**Implement:**
- Bootloader integration: UEFI first, then BIOS; load kernel and initrd.
- Kernel entry: Early console, CPU init, memory map parsing, stack setup.
- Physical memory manager: Detect usable regions, allocate/free frames.
- Virtual memory: Kernel page tables, identity mapping, user address space foundation.
- Interrupts: IDT, exception handlers, timer, keyboard input.
- Scheduler: Cooperative or simple round-robin scheduler.
- Syscall interface: Minimal stable syscall table and error conventions.
- Init process: PID 1, service startup, graceful shutdown.
- Shell: Execute built-ins and simple external commands.
- Serial logging: Persistent boot diagnostics and failure-stage reporting.

**Minimum boot contract:**
```text
UEFI/BIOS
  → bootloader
  → kernel entry
  → memory management
  → interrupts/timer
  → scheduler
  → init
  → shell prompt
  → command execution
  → clean shutdown
```

**Acceptance criteria:**
- QEMU x86_64 UEFI boot reaches a shell prompt.
- BIOS boot reaches a shell prompt.
- Serial log captures each boot stage.
- A smoke command executes successfully.
- Kernel panic, timeout, or failed init produces a classified failure report.

---

### M2: Core Userspace and Filesystem

**Missing capability:** A minimal but real userspace environment in which processes, files, and basic OS services work end to end.

**Implement:**
- VFS abstraction with mount points, path resolution, file descriptors, and standard error codes.
- A simple in-memory or RAM-disk filesystem first; add a persistent filesystem only after VFS semantics are stable.
- Process creation, exit, wait, and basic signal handling.
- Pipes, standard input/output/error, and basic IPC.
- Core userland utilities: `sh`, `ls`, `cat`, `echo`, `mkdir`, `rm`, `cp`, `ps`, `kill`, and `reboot`.
- A minimal libc or Rust-based userspace runtime.
- A service manager with start, stop, status, dependency ordering, and restart policy.

**Acceptance criteria:**
- A process can be created, execute a program, read/write files, and exit.
- Shell redirection and pipes work.
- Filesystem operations survive basic conformance tests.
- Init starts required services in dependency order.
- A service crash does not prevent shutdown.

---

### M3: Install, Update, and Recovery

**Missing capability:** A trustworthy install/update/recovery path with atomic rollback.

**Implement:**
- **Installer:** Disk detection, partitioning, filesystem creation, base system installation, bootloader installation, initial system configuration, post-install verification, and install log/recovery.
- **Update system:** Atomic image or transaction-based updates, update manifest with hashes, pre-update health checks, staged activation, automatic rollback on failed boot/verification, and update journal.
- **Recovery:** Recovery boot entry, shell access without desktop dependencies, filesystem check/repair hooks, rollback to previous system generation, and backup/restore of configuration.

**Acceptance criteria:**
- A clean QEMU installation completes without manual intervention.
- A failed update either completes atomically or restores the previous generation.
- A deliberately corrupted update can be rolled back.
- Recovery mode can repair or restore the system.

---

### M4: Security and Isolation

**Missing capability:** Enforceable security boundaries, not only security-related code.

**Implement:**
- Capability model for filesystem, network, process, IPC, device, and package execution.
- Process isolation with restricted syscall and resource access.
- Mandatory access checks for privileged operations.
- Sandboxed package-script execution.
- Secure update verification using signed manifests.
- Secure boot and measured-boot roadmap (QEMU test hooks).
- Fuzzing for syscall handlers, filesystem paths, IPC, package parsers, and network input.
- Security review workflow separate from ordinary code review.

**Acceptance criteria:**
- Every protected operation fails without the required capability.
- Untrusted package scripts cannot access host resources beyond declared capabilities.
- Security tests include positive authorization and negative denial cases.
- Threat model covers local user, compromised process, malicious package, network input, and update-channel attacks.

---

### M5: Desktop and Applications

**Missing capability:** A minimal desktop path built on a stable runtime.

**Implement in stages:**
1. Display server/compositor foundation: framebuffer output, window surfaces, input events, basic rendering.
2. Window management: create, focus, move, resize, close windows.
3. Application launch: launch a graphical application from shell or launcher.
4. Session management: login, logout, lock, shutdown.
5. Diagnostics: logs, crash reporting, system information.
6. Zenith workflow: command palette, workspace organization, application integration.

**Acceptance criteria:**
- QEMU boot reaches a graphical session.
- Keyboard and pointer input work.
- A basic application opens, receives input, and closes.
- A crashed application does not crash the compositor or kernel.
- Logs identify the failing component.

---

### M6: Package Ecosystem and Compatibility

**Missing capability:** A safe package system and realistic Linux/BSD compatibility strategy.

**Implement in phases:**
1. Package metadata schema and parser (Parse only; no execution)
2. Dependency resolver (Dry-run only)
3. Package installation into isolated image (No host execution)
4. Sandboxed script execution (Explicit capabilities)
5. Repository signing and verification (Reject unsigned or tampered packages)
6. Linux/BSD compatibility layer (ABI/metadata compatibility first)

**Acceptance criteria:**
- A package can be parsed, dependency-resolved, installed into a test image, and rolled back.
- Package scripts run only inside a sandbox.
- A malicious or malformed package fails safely.
- Compatibility claims distinguish "metadata parsed", "installable", and "runtime compatible".

---

### M7: Hardware Enablement

**Missing capability:** Validated device support.

**Implement in tiers:**
- Tier 1: Virtual devices required for CI (QEMU x86_64, virtio-blk, virtio-net, VGA/framebuffer)
- Tier 2: Common virtual/real peripherals (USB keyboard, mouse, storage, basic audio)
- Tier 3: Laptop/desktop hardware (Specific Wi-Fi, GPU, audio, sleep/resume)
- Tier 4: Advanced features (GPU acceleration, external displays, power management)

---

### M8: Agent-Native Automation

**Missing capability:** A safe, repeatable agent layer that accelerates development without compromising security.

**Implement:**
- Agent registry defining each agent's scope, permissions, tools, and report location.
- Task queue with `open`, `in-progress`, `done`, and `blocked` states.
- Runbooks for build, test, boot, audit, packaging, documentation, and release.
- Agent CLI executing tasks and writing structured evidence.
- Capability auditor comparing claims with tests and runtime logs.
- Boot validator maintaining a persistent boot matrix.
- Security agent reviewing security-relevant diffs.
- Human approval workflow for high-risk and destructive changes.

**Agent Permission Matrix:**
| Agent | Repository read | Reports | Tests | Network | Package execution | Direct merge |
|---|---|---|---|---|---|---|
| Documentation | Yes | Yes | No | No | No | No |
| Test runner | Yes | Yes | Yes | No | No | No |
| Boot validator | Yes | Yes | Yes | No | No | No |
| Capability auditor | Yes | Yes | No | No | No | No |
| Package agent | Yes | Yes | Yes | No | Sandbox only | No |
| Security agent | Yes | Yes | Yes | No | No | No |

---

## 12 Industry-Standard Implementation Rules

1. **Vertical slices first:** Complete one end-to-end path before adding breadth.
2. **Interface contracts before implementation:** Publish public interface, error behavior, dependencies, and test contract first.
3. **Evidence-gated completion:** Implementation, tests, runtime evidence, documentation, and regression coverage required.
4. **CI is mandatory:** Required CI checks block merge.
5. **Reproducibility:** Every build and test must be repeatable from recorded inputs.
6. **Fail safely:** Boot, installer, update, VFS, and storage changes must define rollback or recovery behavior before merge.
7. **Least privilege:** Minimum permissions required for agents, CI jobs, packages, and services.
8. **Security review is separate:** Dedicated security review for privileged operations and boundaries.
9. **Regression tests are mandatory:** Bug fixes and features must include tests preventing regression.
10. **No borrowed-status claims:** External reference capabilities cannot be claimed without local proof.
11. **One owner per component:** Named owner/agent, canonical documentation page, test suite, and status field.
12. **Human approval for destructive or high-risk operations:** Human signoff required for installer, update, bootloader, security, and release changes.

---

## Gap-to-Component Map

| Missing component | Depends on | First deliverable | Proof |
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

- **Days 1–14 (Foundation):** Create build container, `make verify`, CI gates, evidence reporting; define ownership; audit claims.
- **Days 15–35 (Bootable core):** Implement UEFI QEMU boot, early console, memory management, interrupts, scheduler, init, shell, boot smoke tests.
- **Days 36–55 (Usable base system):** Implement VFS, processes, pipes, signals, core userland utilities, service manager, shutdown.
- **Days 56–75 (Install and recover):** Implement clean QEMU installation, atomic update, rollback, recovery mode.
- **Days 76–90 (Security and first desktop slice):** Implement capability checks, sandboxed package dry-runs, security denial tests, minimal graphical session.

---

## Definition of "Missing Component Implemented" (10-Point Checklist)

1. Named owner and canonical documentation page exist.
2. Public interface is defined.
3. Implementation is connected to the real boot/runtime path.
4. Unit and integration tests pass.
5. QEMU or hardware runtime evidence exists.
6. A regression test prevents reversal.
7. Security impact has been reviewed.
8. Rollback or failure behavior is defined where relevant.
9. Status and evidence links are current.
10. Component appears in a reproducible build or release artifact.
