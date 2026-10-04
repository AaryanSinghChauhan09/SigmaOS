# Inspiration and Reference Projects

This guide turns ideas from established open-source operating systems into a reviewable SigmaOS engineering agenda. It complements the [comparative gap analysis](OPEN_SOURCE_OS_COMPARATIVE_GAP_ANALYSIS.md): a file or type with a familiar name is not evidence that the corresponding system capability works. Treat each status below as a direction to investigate, then verify behavior in code and on a booted image before claiming completion.

## How to use these references

Borrow a design principle when it fits SigmaOS's goals; do not copy a project's architecture wholesale. For each proposed change, record the source design, the SigmaOS constraint it addresses, the code path to change, and a test or observable acceptance condition. Prefer small vertical slices that can be built, booted, and reviewed. Separate implemented, partial, stubbed, and unverified behavior in status reports.

## Project-to-SigmaOS map

| Reference | Useful lesson | SigmaOS areas to examine | Adaptation and evidence |
|---|---|---|---|
| [Linux kernel](https://www.kernel.org/doc/html/latest/) | Stable subsystem boundaries, explicit interfaces, mature device and filesystem abstractions, and broad hardware enablement | `src/mm/`, `src/scheduler/`, `src/vfs/`, `src/fs/`, `src/drivers/`, `src/net/` | Prioritize coherent end-to-end paths over feature-name parity. Add tests for boundary behavior and run representative hardware or QEMU workloads. |
| [Omarchy](https://github.com/omacom/omarchy) | Opinionated desktop defaults, discoverable keyboard workflows, coherent theming, and practical install/update flows | `src/desktop/`, installer and update components, user documentation | Keep defaults usable and customization discoverable. Validate with a clean install, keyboard-only navigation, theme change, and recovery from a failed update. |
| [os-tutorial](https://github.com/cfenollosa/os-tutorial) | Small, staged examples make boot, interrupts, memory, and user/kernel transitions understandable | `src/boot/`, interrupt handling, syscall entry, architecture docs | Add minimal runnable examples and explain invariants alongside production code. Verify each stage boots under the documented emulator configuration. |
| [Linux Mint](https://github.com/linuxmint) | Reduce friction through approachable setup, familiar system tools, sensible defaults, and clear help | installer, settings, software management, backup/recovery, onboarding | Test common tasks with a new user profile and document failures in plain language. Avoid assuming that naming a Mint-inspired tool means its workflow is complete. |
| [Redox OS](https://www.redox-os.org/) | Rust-first system design, explicit separation of services, and resource-oriented interfaces | process/service boundaries, IPC, VFS, driver isolation, Rust safety | Evaluate which services can be isolated without undermining SigmaOS's current kernel design. Prototype one boundary and measure fault containment and IPC cost before expanding it. |
| [xv6 (RISC-V)](https://github.com/mit-pdos/xv6-riscv) | A compact reference for reasoning about processes, traps, paging, files, and locking | syscall/process lifecycle, virtual memory, filesystem, SMP synchronization | Use xv6 as a review and teaching reference. Port concepts, not code blindly; document changed invariants and test concurrency and failure paths. |

## Recommended work, in dependency order

These are candidate priorities based on the current source layout. They are not claims that each subsystem is absent; existing modules need implementation-level and boot-level validation before gaps can be stated precisely.

### 1. Establish a trustworthy capability baseline

- Choose a small set of user-visible boot scenarios and record which currently work end to end: boot, allocate memory, start a process, make a syscall, read/write a file, and recover from a failed boot/update.
- For each subsystem, classify the path as `working`, `partial`, `stub`, or `unverified`, with a source location and a reproducible check.
- Reconcile README feature claims and the status tracker with those checks. Remove unsupported numerical parity claims or attach reproducible methodology and results.

**Acceptance:** a clean checkout can reproduce each claimed capability using documented commands; status terms have consistent meanings.

### 2. Make the boot-to-shell path reliable

- Use the staged clarity of os-tutorial and xv6 to document firmware/bootloader handoff, early memory setup, interrupt enablement, allocator initialization, and first user process.
- Add emulator smoke scenarios for successful boot and expected failure cases (missing boot media, invalid configuration, allocation failure where injectable).
- Keep serial logs actionable and preserve the earliest failure reason.

**Acceptance:** documented QEMU invocation reaches a usable shell repeatedly, and failures identify the stage that stopped.

### 3. Complete one vertical process and storage slice

- Use xv6's compact process/syscall/filesystem model as a checklist for lifecycle semantics, while using Linux documentation for mature interface boundaries.
- Trace one program from executable loading through process creation, syscall dispatch, file descriptor use, and exit/reaping. Trace writes through VFS to a block device and back.
- Identify stubs and missing error paths in these traces before adding more API surface.

**Acceptance:** integration coverage demonstrates the full trace, including invalid inputs and resource cleanup.

### 4. Turn desktop components into a coherent daily workflow

- Draw on Omarchy's opinionated key-driven workflow and Mint's approachable setup and system tools.
- Pick a minimal first-session path: configure display/input, connect to a network, launch an app, find help, install/update software, and recover settings.
- Ensure keyboard focus, visible feedback, accessible labels, and clear rollback/error handling across the path.

**Acceptance:** a clean-install walkthrough completes without editing internal configuration files; every failed operation offers a useful next step.

### 5. Evaluate isolation boundaries before broad feature expansion

- Use Redox as a source of ideas for service and driver separation; compare the isolation boundary against SigmaOS's current kernel architecture and threat model.
- Prototype one non-critical service behind a narrow IPC/interface boundary. Specify ownership, failure behavior, permissions, and restart policy.

**Acceptance:** a service failure is contained, authorization is tested, and the interface can evolve without exposing kernel internals.

### 6. Grow hardware support through tested device classes

- Borrow Linux's driver model discipline: explicit bus/device lifecycle, resource ownership, capability checks, and consistent error reporting.
- Prioritize one device class at a time (for example virtio block/network, then common real hardware) and keep a compatibility matrix tied to actual test runs.

**Acceptance:** each support claim names the device/emulator, tested operations, and known limitations.

## Contribution checklist

For work inspired by another project, include:

1. The upstream design or documentation link and the specific idea being adapted.
2. Why it fits SigmaOS's architecture and what is intentionally out of scope.
3. The concrete source modules and user-visible behavior affected.
4. An acceptance check that exercises behavior, including a failure path.
5. Updated capability status only after the check passes; distinguish emulator validation from physical hardware validation.

## Primary references

- Linux kernel documentation: <https://www.kernel.org/doc/html/latest/>
- Omarchy source and documentation: <https://github.com/omacom/omarchy>
- os-tutorial source: <https://github.com/cfenollosa/os-tutorial>
- Linux Mint projects: <https://github.com/linuxmint>
- Redox OS book and source: <https://doc.redox-os.org/book/> and <https://gitlab.redox-os.org/redox-os/redox>
- xv6 RISC-V source and book: <https://github.com/mit-pdos/xv6-riscv> and <https://pdos.csail.mit.edu/6.1810/2025/xv6/book-riscv-rev5.pdf>
