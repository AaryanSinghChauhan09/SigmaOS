## Component
- [ ] Kernel Core / Architecture (`src/kernel/`, `src/arch/`)
- [ ] Memory Management / Allocator (`src/memory/`, `src/mm/`, `src/swap/`)
- [ ] Scheduler & Process Control (`src/scheduler/`, `src/process/`)
- [ ] Virtual File System / Storage (`src/vfs/`, `src/fs/`, `src/storage/`)
- [ ] Drivers & Hardware Abstraction (`src/drivers/`, `src/hardware/`)
- [ ] Userland Primitives & Shell (`src/userland/`, `src/shell/`)
- [ ] Security & Capability System (`src/security/`, `src/tpm/`)
- [ ] Desktop & Innovations (`src/desktop/`, `src/creative/`)
- [ ] Distro & Open-Source OS Adoption (`src/distro/`)

## Open-Source OS Idea Adoption Reference & Upstream Study
- Upstream OS Reference / Reference study: Redox, seL4, Tock, Fuchsia, Linux, FreeBSD, OpenBSD, Plan 9, Haiku, Debian
- What SigmaOS adopts:
- Open-Source Idea Issue Link / Study document:

## Architectural & Kernel Safety Review
- [ ] **`#![no_std]` Compliance**: Maintained in kernel and driver modules without unallowed `std::` imports.
- [ ] **Capability Tokens**: Entrypoints for system calls include explicit `CapabilityToken` verification.
- [ ] **Driver Lifecycle**: New/modified drivers follow `IoManager` / `DriverObject` / `DeviceObject` lifecycle guidelines with unit test coverage.
- [ ] **Memory Pools**: Proper isolation and handling between `Paged` and `NonPaged` memory allocations.
- [ ] **Type Annotations**: Explicit type signatures on public APIs and key kernel collections.
- [ ] **Memory Safety**: Bounds-checked copy operations and audited `unsafe` block invariants.
- [ ] **ABI Stability**: Highlight any system call number or ABI modifications.

## Shell safety review
- [ ] Interactive shell, lexer/parser, builtins, and POSIX conformance verified.

## Kernel safety review
- [ ] Kernel panic handlers, register contexts, and memory isolation audited.

## Safety and rollback
- [ ] Rollback strategy, feature flags, and fail-safe recovery plans documented (Risks and rollback).

## Reviewer checklist
- [ ] All test suites passed, architectural constraints satisfied, documentation updated.

## Testing & Evidence
- [ ] Standalone file unit test executed (`rustc --test`)
- [ ] Repository test suite executed (`./run_sigma_tests.sh`)
- [ ] QEMU boot smoke test executed (if kernel/boot modified)
- [ ] Benchmarks / Fuzzing executed (if applicable)

## Additional Notes
<!-- Detail any potential risks, performance impact, or safety and rollback strategy -->
