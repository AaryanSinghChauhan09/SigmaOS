# PR Proposal: Implement Missing Components as Bootable Vertical Slices

## Summary

SigmaOS implements missing components as a sequence of **small, bootable vertical slices**, not as parallel rewrites of every subsystem. The repository identifies all required OS layers—kernel, drivers, userspace, shell, packaging, desktop, installer, and agent infrastructure—and enforces boot, runtime integration, hardware validation, and installer/update recovery via evidence-gated vertical slice milestones.

## Implementation Philosophy

Use the discipline of mature Linux/BSD distributions: first make a minimal system boot and run, then make it installable and recoverable, then broaden hardware and application support.

| Distro/BSD lesson | SigmaOS implementation rule |
|---|---|
| Debian/Ubuntu installer-first approach | Build a minimal bootable image before broad desktop features |
| Arch’s explicit configuration | Every subsystem has a declared interface, owner, and test |
| Fedora CI gating | CI blocks regressions rather than merely reporting them |
| FreeBSD release engineering | Every release has known issues, rollback, and validation evidence |
| OpenBSD least privilege | Security boundaries are designed before convenience features |
| NixOS declarative model | System configuration, dependencies, and tests are explicit |
| Linux hardware enablement | Hardware support is proven per device, not claimed generally |

## Component Implementation Order

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

## Milestone Specifications

### M0: Build and Test Foundation
- **Missing capability:** A reliable, reproducible way to prove that the repository builds and passes its own checks.
- **Implementation:**
  - Deterministic build container with pinned Rust toolchain, QEMU version, dependencies, and build scripts.
  - A `make verify` target running formatting, check, and test suites.
  - CI pipeline blocking merges on failures.
  - Build manifest containing commit hash, toolchain versions, container digest, commands, timestamps, and artifact hashes.
  - Report directory for build, test, boot, installer, package, and security evidence.

### M1: Bootable Kernel and Shell
- **Minimum boot contract:**
  `UEFI/BIOS -> bootloader -> kernel entry -> memory management -> interrupts/timer -> scheduler -> init -> shell prompt -> command execution -> clean shutdown`
- **Acceptance criteria:**
  - QEMU x86_64 UEFI and BIOS boot reach a shell prompt.
  - Serial log captures each boot stage.
  - Kernel panic or failed init creates classified failure reports.

### M2: Core Userspace and Filesystem
- **Implementation:** VFS abstraction with mount points, RAM-disk filesystem, process lifecycle (`fork`/`exec`/`exit`/`wait`), IPC pipes, core userland utilities (`sh`, `ls`, `cat`, `echo`, `mkdir`, `rm`, `cp`, `ps`, `kill`, `reboot`), and dependency-ordered service manager.

### M3: Install, Update, and Recovery
- **Installer:** Disk detection, partitioning, filesystem creation, base system installation, bootloader install, post-install verification.
- **Update System:** Atomic image updates, package hash manifests, pre-update health checks, automatic rollback on failed boot.
- **Recovery Mode:** Minimal shell access without desktop dependencies, FS repair hooks, rollback to prior generation.

### M4: Security and Isolation
- Capability model for FS, network, process, IPC, device, and package execution.
- Sandboxed package script execution and secure update verification with signed manifests.
- Syzkaller/fuzzing hooks for syscall handlers and IPC parsers.

### M5: Desktop and Applications
- Framebuffer compositor foundation -> window management -> graphical app launch -> session management -> diagnostics -> Zenith workflow.

### M6: Package Ecosystem and Compatibility
- Isolated dry-run package parsing, dependency resolution, sandbox execution, repo signing, and multi-distro metadata translation (`.deb`, `.rpm`, Arch, BSD).

### M7: Hardware Enablement
- Tiered device enablement starting from Tier 1 virtual devices (QEMU virtio-blk/net) to physical USB, audio, Wi-Fi, and GPU devices.

### M8: Agent-Native Automation
- Agent registry, machine-readable TOML task queues, runbooks, capability auditor, boot matrix validator, and human approval workflow for destructive/high-risk operations.

## 12 Industry-Standard Implementation Rules

1. **Vertical slices first:** Complete one end-to-end path before adding breadth.
2. **Interface contracts before implementation:** Subsystems publish public interface, error behavior, dependencies, and test contract before code.
3. **Evidence-gated completion:** Code + tests + runtime evidence + docs + regression tests.
4. **CI is mandatory:** Required CI checks block merge.
5. **Reproducibility:** Every build/test is repeatable from recorded inputs.
6. **Fail safely:** Define rollback or recovery behavior before merge.
7. **Least privilege:** Least privilege permission matrix across agents, packages, and services.
8. **Security review is separate:** Dedicated security review for security boundaries and cryptography.
9. **Regression tests are mandatory:** Bug fixes and features require failing-then-passing test coverage.
10. **No borrowed-status claims:** Reference features do not establish SigmaOS claims without runtime proof.
11. **One owner per component:** One named owner, canonical doc, test suite, and status field.
12. **Human approval for destructive or high-risk operations:** Human sign-off required for installer, update, bootloader, security boundary, and subsystem deletion changes.

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

## 90-Day Implementation Roadmap

- **Days 1–14 (Foundation):** Build container, `make verify`, CI gates, evidence reporting, ownership matrix.
- **Days 15–35 (Bootable Core):** UEFI QEMU boot, early console, memory manager, interrupts, scheduler, init, shell, boot-to-shell smoke test.
- **Days 36–55 (Usable Base System):** VFS, processes, pipes, signals, core userland utilities, service manager.
- **Days 56–75 (Install and Recover):** Clean QEMU install, atomic updates, recovery mode, failure rollback tests.
- **Days 76–90 (Security and First Desktop Slice):** Capabilities, sandboxing, fuzzing, minimal compositor session, nightlies.

## Definition of "Missing Component Implemented" Checklist (10 Points)

1. Named owner and canonical documentation page exists.
2. Public interface contract published.
3. Connected to real boot/runtime path.
4. Unit and integration tests pass.
5. QEMU or hardware runtime evidence recorded.
6. Regression test suite prevents reversal.
7. Security impact reviewed.
8. Rollback and failure behavior defined.
9. Status and evidence links updated in capability matrix.
10. Included in a reproducible build or release artifact.
