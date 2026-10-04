# Kernel

**Capability state: Prototype.** The current source tree is not a verified bootable kernel. See the [shared status vocabulary](14-Future-Development.md#work-status-vocabulary) and the latest [main-branch test record](Testing.md).

## Current capability

The repository contains Rust modules for kernel-like services, schedulers, memory, syscalls, and drivers. `cargo check --lib` checks the hosted library target; it does not prove that the kernel boots. The `x86_64-unknown-none` target currently fails because the crate depends on `std` and has unresolved `no_std` build errors. No boot-to-shell path has been verified.

Do not treat Linux command examples (`sysctl`, `dmesg`, `sigps`, `sigmem`, `sigmod`), Linux syscall names, or module filenames as working SigmaOS interfaces unless the repository has a connected runtime implementation and a test demonstrating it.

## Development order

1. Separate the bootable kernel from the hosted library and define one boot protocol and one x86_64 virtual machine profile.
2. Establish serial output, panic handling, memory-map handoff, and reproducible kernel image generation.
3. Boot the real kernel in QEMU and capture a stable kernel-ready marker.
4. Add memory management, interrupts, timer, input, storage, process execution, and syscall support incrementally.
5. Keep APIs marked prototype until exercised through the boot path and failure cases are tested.

## Inspiration and acceptance

- Linux: use narrow, documented subsystem interfaces and test real integration paths.
- OpenBSD: make memory protections and security boundaries enforceable, then test attempted violations.
- FreeBSD and NetBSD: keep driver and subsystem boundaries explicit so implementations can be inspected and replaced.
- Arch Linux: document exact configuration and commands, with no assumed hidden behavior.

**Completion evidence:** a clean build produces a validated kernel image; QEMU boots that image through the actual boot path; serial logs identify the kernel-ready point; module, integration, and failure-path results are recorded. Until then, scheduler, isolation, hardening, and driver support remain prototypes.

## Related pages

- [Boot and installation](01-Installation.md)
- [Memory management](Memory-Management.md)
- [Scheduler](Scheduler.md)
- [Hardware drivers](Hardware-Drivers.md)
- [Security](07-Security.md)
