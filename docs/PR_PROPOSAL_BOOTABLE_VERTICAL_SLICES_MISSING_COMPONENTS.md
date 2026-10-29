# PR Proposal: Implement Missing Components as Bootable Vertical Slices

- **PR Title**: `feat(roadmap): Implement Missing Components as Bootable Vertical Slices`
- **PR Branch**: `feature/bootable-vertical-slices-roadmap`
- **Status**: Approved / Specification Ready

## Executive Summary

SigmaOS should implement missing components as a sequence of **small, bootable vertical slices**, not as parallel rewrites of every subsystem. The repository already identifies the needed OS layers—kernel, drivers, userspace, shell, packaging, desktop, installer, and agent infrastructure—but its own wiki and support matrix show that boot, runtime integration, hardware validation, and installer/update recovery remain unproven.

## Implementation Philosophy

Use the discipline of mature Linux/BSD distributions: first make a minimal system boot and run, then make it installable and recoverable, then broaden hardware and application support.

| Distro/BSD lesson | SigmaOS implementation rule |
|---|---|
| **Debian/Ubuntu installer-first approach** | Build a minimal bootable image before broad desktop features |
| **Arch’s explicit configuration** | Every subsystem has a declared interface, owner, and test |
| **Fedora CI gating** | CI blocks regressions rather than merely reporting them |
| **FreeBSD release engineering** | Every release has known issues, rollback, and validation evidence |
| **OpenBSD least privilege** | Security boundaries are designed before convenience features |
| **NixOS declarative model** | System configuration, dependencies, and tests are explicit |
| **Linux hardware enablement** | Hardware support is proven per device, not claimed generally |

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

## M0: Build and Test Foundation

**Missing capability:** A reliable, reproducible way to prove that the repository builds and passes its own checks.

### Implement

- A deterministic build container with pinned Rust toolchain, QEMU version, dependencies, and build scripts.
- A `make verify` target that runs formatting, linting, unit tests, integration tests, and artifact creation.
- A CI pipeline that blocks merge on required failures.
- A build manifest containing commit hash, toolchain versions, container digest, commands, timestamps, and artifact hashes.
- A report directory for build, test, boot, installer, package, and security evidence.

### Acceptance Criteria

- `make verify` succeeds from a clean checkout.
- CI runs on every PR.
- Every artifact can be reproduced from its manifest.
- Test failures create actionable reports, not silent logs.

SigmaOS already exposes `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and Make targets for check, test, format, and clean; these should become the non-negotiable baseline.

## M1: Bootable Kernel and Shell

**Missing capability:** A proven firmware-to-shell path. The support matrix records 0% QEMU x86_64 boot success and marks UEFI/BIOS support as not started.

### Implement

| Component | Minimum implementation |
|---|---|
| **Bootloader integration** | UEFI first, then BIOS; load kernel and initrd |
| **Kernel entry** | Early console, CPU init, memory map parsing, stack setup |
| **Physical memory manager** | Detect usable regions, allocate/free frames |
| **Virtual memory** | Kernel page tables, identity mapping, user/address-space foundation |
| **Interrupts** | IDT, exception handlers, timer, keyboard input |
| **Scheduler** | Cooperative or simple round-robin scheduler |
| **Syscall interface** | Minimal stable syscall table and error conventions |
| **Init process** | PID 1, service startup, graceful shutdown |
| **Shell** | Execute built-ins and simple external commands |
| **Serial logging** | Persistent boot diagnostics and failure-stage reporting |

### Minimum Boot Contract

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

### Acceptance Criteria

- QEMU x86_64 UEFI boot reaches a shell prompt.
- BIOS boot reaches a shell prompt.
- Serial log captures each boot stage.
- A smoke command executes successfully.
- Kernel panic, timeout, or failed init produces a classified failure report.

## M2: Core Userspace and Filesystem

**Missing capability:** A minimal but real userspace environment in which processes, files, and basic OS services work end to end.

### Implement

- VFS abstraction with mount points, path resolution, file descriptors, and standard error codes.
- A simple in-memory or RAM-disk filesystem first; add a persistent filesystem only after VFS semantics are stable.
- Process creation, exit, wait, and basic signal handling.
- Pipes, standard input/output/error, and basic IPC.
- Core userland utilities: `sh`, `ls`, `cat`, `echo`, `mkdir`, `rm`, `cp`, `ps`, `kill`, and `reboot`.
- A minimal libc or Rust-based userspace runtime.
- A service manager with start, stop, status, dependency ordering, and restart policy.

### Acceptance Criteria

- A process can be created, execute a program, read/write files, and exit.
- Shell redirection and pipes work.
- Filesystem operations survive basic conformance tests.
- Init starts required services in dependency order.
- A service crash does not prevent shutdown.

## M3: Install, Update, and Recovery

**Missing capability:** A trustworthy install/update/recovery path. SigmaOS’s README identifies installer and update-path recovery as a near-term priority.

### Implement

#### Installer

- Disk detection and partitioning.
- Filesystem creation.
- Base system installation.
- Bootloader installation.
- Initial system configuration.
- Post-install verification.
- Install log and failure recovery.

#### Update System

- Atomic image or transaction-based updates.
- Update manifest with package/component hashes.
- Pre-update health checks.
- Staged activation.
- Automatic rollback on failed boot or verification.
- Update journal and known-good-version marker.

#### Recovery

- Recovery boot entry.
- Shell access without normal desktop dependencies.
- Filesystem check and repair hooks.
- Rollback to previous system generation.
- Backup and restore of configuration.

### Acceptance Criteria

- A clean QEMU installation completes without manual intervention.
- A failed update either completes atomically or restores the previous generation.
- A deliberately corrupted update can be rolled back.
- Recovery mode can repair or restore the system.

This is the point at which SigmaOS becomes distribution-like rather than merely a kernel project.

## M4: Security and Isolation

**Missing capability:** Enforceable security boundaries, not only security-related code.

### Implement

- Capability model for filesystem, network, process, IPC, device, and package execution.
- Process isolation with restricted syscall and resource access.
- Mandatory access checks for privileged operations.
- Sandboxed package-script execution.
- Secure update verification using signed manifests.
- Secure boot and measured-boot roadmap, beginning with documentation and QEMU test hooks.
- Fuzzing for syscall handlers, filesystem paths, IPC, package parsers, and network input.
- Security review workflow separate from ordinary code review.

### Acceptance Criteria

- Every protected operation fails without the required capability.
- Untrusted package scripts cannot access host resources beyond declared capabilities.
- Security tests include positive authorization and negative denial cases.
- A documented threat model covers local user, compromised process, malicious package, network input, and update-channel attacks.

## M5: Desktop and Applications

**Missing capability:** A minimal desktop path built on a stable runtime.

### Implement in Stages

1. **Display server/compositor foundation:** framebuffer output, window surfaces, input events, and basic rendering.
2. **Window management:** create, focus, move, resize, and close windows.
3. **Application launch:** launch a graphical application from shell or launcher.
4. **Session management:** login, logout, lock, and shutdown.
5. **Diagnostics:** logs, crash reporting, and system information.
6. **Zenith workflow:** command palette, workspace organization, and application integration only after the base desktop is stable.

### Acceptance Criteria

- QEMU boot reaches a graphical session.
- Keyboard and pointer input work.
- A basic application opens, receives input, and closes.
- A crashed application does not crash the compositor or kernel.
- Logs identify the failing component.

Do not begin advanced desktop features until the minimal desktop passes repeated QEMU boot and interaction tests.

## M6: Package Ecosystem and Compatibility

**Missing capability:** A safe package system and realistic Linux/BSD compatibility strategy.

### Implement in Phases

| Phase | Deliverable | Safety rule |
|---|---|---|
| **1** | Package metadata schema and parser | Parse only; no execution |
| **2** | Dependency resolver | Dry-run only |
| **3** | Package installation into isolated image | No host execution |
| **4** | Sandboxed script execution | Explicit filesystem, process, and network capabilities |
| **5** | Repository signing and verification | Reject unsigned or tampered packages |
| **6** | Linux/BSD compatibility layer | Start with ABI/metadata compatibility, not blanket binary execution |

### Compatibility Matrix

Track each format and capability separately:
- `.deb` metadata parse
- `.rpm` metadata parse
- Arch package metadata parse
- BSD package metadata parse
- Dependency resolution
- File layout translation
- Scriptlet policy
- Sandbox execution
- Install success
- Update/rollback behavior

### Acceptance Criteria

- A package can be parsed, dependency-resolved, installed into a test image, and rolled back.
- Package scripts run only inside a sandbox.
- A malicious or malformed package fails safely.
- Compatibility claims distinguish “metadata parsed,” “installable,” and “runtime compatible.”

## M7: Hardware Enablement

**Missing capability:** Validated device support. The support matrix currently reports no real hardware testing and no graphics, Wi-Fi, audio, USB, or suspend/resume support.

### Implement in Tiers

| Tier | Scope | Examples |
|---|---|---|
| **Tier 1** | Virtual devices required for CI | QEMU x86_64, virtio-blk, virtio-net, VGA/framebuffer |
| **Tier 2** | Common virtual/real peripherals | USB keyboard, mouse, storage, basic audio |
| **Tier 3** | Laptop/desktop hardware | Specific Wi-Fi, GPU, audio, sleep/resume devices |
| **Tier 4** | Advanced features | GPU acceleration, external displays, power management |

### Rules

- QEMU x86_64 must reach its boot target before ARM64 or physical hardware work expands.
- Every supported device needs a test log, device model, driver version, date, and known limitations.
- Unsupported features must remain explicitly listed as unsupported.
- A hardware claim without a reproducible test is prohibited.

## M8: Agent-Native Automation

**Missing capability:** A safe, repeatable agent layer that accelerates development without creating an unreliable or insecure system.

### Implement

- Agent registry defining each agent’s scope, permissions, tools, and report location.
- Task queue with `open`, `in-progress`, `done`, and `blocked` states.
- Runbooks for build, test, boot, audit, packaging, documentation, and release.
- Agent CLI that executes a task and writes structured evidence.
- Capability auditor that compares claims with tests and runtime logs.
- Boot validator that maintains a persistent boot matrix.
- Security agent that reviews only security-relevant diffs.
- Human approval workflow for kernel, boot, installer, update, package execution, and release changes.

### Agent Permission Model

| Agent | Repository read | Reports | Tests | Network | Package execution | Direct merge |
|---|---|---|---|---|---|---|
| **Documentation** | Yes | Yes | No | No | No | No |
| **Test runner** | Yes | Yes | Yes | No | No | No |
| **Boot validator** | Yes | Yes | Yes | No | No | No |
| **Capability auditor** | Yes | Yes | No | No | No | No |
| **Package agent** | Yes | Yes | Yes | No | Sandbox only | No |
| **Security agent** | Yes | Yes | Yes | No | No | No |

## Industry-Standard Implementation Rules

1. **Vertical slices first**: Complete one end-to-end path before adding breadth. For example: UEFI boot → kernel init → shell → command execution → shutdown.
2. **Interface contracts before implementation**: Every subsystem must publish its public interface, error behavior, dependencies, and test contract before large implementation work begins.
3. **Evidence-gated completion**: A component is complete only when implementation, tests, runtime evidence, documentation, and regression coverage all exist.
4. **CI is mandatory**: Required CI checks must block merge. At minimum: clean build, unit tests, integration tests, formatting, boot smoke test, and security checks for affected paths.
5. **Reproducibility**: Every build and test must be repeatable from recorded inputs. Unpinned dependencies, undocumented manual steps, and local-only artifacts are not release evidence.
6. **Fail safely**: Boot, installer, update, filesystem, and storage changes must define rollback or recovery behavior before merge.
7. **Least privilege**: Agents, CI jobs, packages, and services receive only the minimum permissions required. No blanket root, network, filesystem, or credential access.
8. **Security review is separate**: Capability enforcement, sandboxing, cryptography, syscalls, memory safety, package execution, networking, and update mechanisms require dedicated security review.
9. **Regression tests are mandatory**: Every bug fix and completed feature must include a test that would fail if the behavior regressed.
10. **No borrowed-status claims**: Reference-project capabilities may inspire design but cannot be recorded as SigmaOS capabilities. The project wiki explicitly warns against presenting imported plans or reference-project features as SigmaOS features.
11. **One owner per component**: Each component has one named owner or agent, one canonical documentation page, one test suite, and one status field.
12. **Human approval for destructive or high-risk operations**: Agents may propose and test; humans approve installer changes, bootloader changes, update mechanisms, package-script execution, security boundaries, releases, and deletion of subsystems.

## Gap-to-Component Map

| Missing component | Depends on | First deliverable | Proof |
|---|---|---|---|
| **Bootloader integration** | Build system | UEFI QEMU boot | Serial log |
| **Kernel memory management** | Bootloader | Frame allocator and page tables | Boot test |
| **Interrupts and timer** | Kernel entry | Timer ticks and keyboard input | Event log |
| **Scheduler** | Interrupts | Two processes alternate | Scheduler trace |
| **Syscalls** | Scheduler | Minimal syscall table | Syscall test |
| **VFS** | Syscalls | File open/read/write/close | Filesystem test |
| **Init** | Syscalls | PID 1 starts shell | Boot-to-shell |
| **Installer** | Bootable image | Clean QEMU install | Install log |
| **Update/rollback** | Installer | Failed update restores prior state | Recovery test |
| **Security capabilities** | Syscalls/processes | Denial tests | Security report |
| **Desktop** | Stable boot/runtime | Graphical session | QEMU UI smoke test |
| **Packages** | Filesystem/security | Metadata parse and dry-run | Package test |
| **Hardware support** | QEMU parity | Device-specific test matrix | Hardware logs |

## Ninety-Day Implementation Roadmap

### Days 1–14: Foundation

- Create build container, `make verify`, CI gates, and evidence reporting.
- Define component ownership and canonical documentation pages.
- Audit current claims and classify each as `Proposed`, `Prototype`, `Integrated`, or `Supported`.

### Days 15–35: Bootable core

- Implement UEFI QEMU boot.
- Implement early console, memory management, interrupts, scheduler, init, and shell.
- Add boot-to-shell smoke tests.
- Fix only the current boot blocker until it is reproducibly solved.

### Days 36–55: Usable base system

- Implement VFS, processes, pipes, signals, and basic userland tools.
- Add filesystem conformance and process lifecycle tests.
- Add service manager and graceful shutdown.
- Connect all core subsystems to the actual boot path.

### Days 56–75: Install and recover

- Implement clean QEMU installation.
- Implement atomic update and rollback.
- Add recovery mode.
- Add installer/update failure tests.

### Days 76–90: Security and first desktop slice

- Implement capability checks and sandboxed package dry-runs.
- Add security denial tests and fuzzing for critical parsers.
- Implement minimal graphical session and application launch.
- Publish a `nightly` artifact only if build, tests, QEMU boot, install, and rollback tests pass.

## Definition of “Missing Component Implemented”

A missing component is implemented only when all of these are true:

1. It has a named owner and canonical documentation page.
2. Its public interface is defined.
3. Its implementation is connected to the real boot/runtime path.
4. Unit and integration tests pass.
5. QEMU or hardware runtime evidence exists.
6. A regression test prevents reversal.
7. Security impact has been reviewed.
8. Rollback or failure behavior is defined where relevant.
9. Its status and evidence links are current.
10. It appears in a reproducible build or release artifact.

## Native Rust Engine Implementation

The bootable vertical slices roadmap, milestone specifications, 12 rules, gap map, 90-day roadmap, and 10-point definition of done evaluator are implemented natively in Rust in `src/distro/bootable_vertical_slices_missing_components_pr.rs` and verified via `tests/test_bootable_vertical_slices.rs`.
