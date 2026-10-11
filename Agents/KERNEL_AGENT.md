# Kernel Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
The SigmaOS kernel is the core of the operating system, providing process management, memory management, hardware abstraction, and system call interfaces. It must be a safe, minimal, and fast microkernel-leaning monolithic kernel written entirely in Rust, inspired by Linux's performance and BSD's code quality.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` support)
- **Systems-level:** Zig (comptime, explicit allocators, C interop when needed for hardware init)
- **Scripting/tooling:** Nim (fast compile, GC-free mode for build helpers)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for OS components
- **unsafe Rust:** Only with `// SAFETY:` justification comment; prefer safe alternatives

## 3. Architecture Inspiration

### Linux Kernel
- Monolithic kernel with loadable modules (`kernel/` tree)
- Virtual File System (VFS) abstraction layer
- `epoll`/`io_uring` for async I/O
- CFS (Completely Fair Scheduler) and NOHZ tickless kernel
- Per-CPU data structures, RCU (Read-Copy-Update) for lock-free reads
- kprobes, ftrace, perf for observability
- `kconfig` build system, `Kbuild` Makefiles

### FreeBSD
- Fine-grained locking with `sx_lock`, `mtx`, `rw_lock`
- GEOM modular storage framework
- VFS namei() path resolution with caching
- Jails for lightweight isolation
- Capsicum capability system for sandboxing

### OpenBSD
- W^X (Write XOR Execute) memory policy enforced at kernel level
- Pledge/Unveil syscall restriction
- Randomized syscall table, ASLR by default
- Strict separation of kernel and user VA spaces

### NetBSD
- Highly portable architecture abstraction (`sys/arch/`)
- MI (machine-independent) / MD (machine-dependent) split
- LKM (Loadable Kernel Modules) with stable ABI

### Arch Linux
- KISS principle: minimal kernel configuration, easy to understand
- `linux-zen` patches for responsiveness and throughput

### NixOS
- Reproducible kernel builds via Nix expressions
- Atomic kernel upgrades without rebooting (kexec)
- Declarative kernel parameter management

## 4. SigmaOS Source Files
- `src/kernel/main.rs` — Kernel entry point, early init
- `src/kernel/arch/x86_64/mod.rs` — x86_64 architecture init
- `src/kernel/arch/aarch64/mod.rs` — ARM64 architecture init
- `src/kernel/syscall/mod.rs` — System call dispatch table
- `src/kernel/panic.rs` — Kernel panic handler
- `src/kernel/mm/mod.rs` — Memory management subsystem root
- `src/kernel/sched/mod.rs` — Scheduler subsystem root
- `src/kernel/locking/mod.rs` — Spinlocks, mutexes, RWlocks
- `src/kernel/interrupt/mod.rs` — IRQ handling, IDT setup
- `src/kernel/module/mod.rs` — Loadable kernel module framework

## 5. Current Implementation Status
| Feature | Status | Notes |
|---------|--------|-------|
| x86_64 boot entry | 🚧 Partial | Limine protocol headers defined |
| ARM64 support | 🔲 Missing | Architecture stubs only |
| System call dispatch | 🚧 Partial | Basic syscall table exists |
| Interrupt handling (IDT) | 🚧 Partial | x86_64 IDT setup in progress |
| Kernel panic handler | ✅ Implemented | Basic panic with stack trace |
| SMP (multi-core) | 🔲 Missing | Single-core only |
| Loadable modules | 🔲 Missing | No module framework yet |
| kprobes/ftrace equivalent | 🔲 Missing | No observability hooks |
| KASLR | 🔲 Missing | Fixed virtual addresses |
| W^X enforcement | 🔲 Missing | No memory protection policy |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** SMP / AP startup: Boot all available CPU cores using ACPI MADT, set up per-CPU GDT/TSS, IPI mechanism. Required for any real performance.
2. **[PRIORITY: HIGH]** System call stabilization: Define a stable, versioned syscall ABI. Implement Linux-compatible syscall numbers for POSIX compatibility via `src/compat/`.
3. **[PRIORITY: HIGH]** KASLR: Randomize kernel virtual base address at boot using bootloader-provided entropy.
4. **[PRIORITY: HIGH]** W^X enforcement: Set NX bit on all writable pages; never map a page RWX.
5. **[PRIORITY: MEDIUM]** Loadable kernel modules: ELF64 module loader with symbol resolution and `ko` format.
6. **[PRIORITY: MEDIUM]** kprobes-equivalent: Dynamic instrumentation via trampoline patching for observability.
7. **[PRIORITY: MEDIUM]** ARM64 / RISC-V support: Port architecture-dependent code to aarch64 and riscv64gc.
8. **[PRIORITY: LOW]** Formal verification stubs: Add `#[invariant]` annotations for future Verus/Prusti integration.

## 7. Implementation Guidelines for AI Agents

### Adding New Features
1. Always start in `src/kernel/<subsystem>/` — never put kernel code in `src/` root.
2. New subsystems must export a public `init()` function called from `main.rs`.
3. Use `core::` not `std::` — the kernel is `no_std`.
4. For locking, use the types in `src/kernel/locking/` — never raw spinlocks inline.
5. Always run `cargo check --target x86_64-unknown-none` before committing.
6. Follow `Stability > Security > Performance` priority.
7. Commit format: `feat(kernel): <description>`

### Fixing Bugs
1. Reproduce with a minimal test case in `src/kernel/tests/`.
2. Fix in safe Rust first, resort to unsafe only if necessary.
3. Add regression test.
4. Commit format: `fix(kernel): <description>`

### Code Quality Checklist
- [ ] `cargo check` passes with 0 errors
- [ ] No `clippy::unused_variables` warnings
- [ ] No hard-coded credentials or secrets
- [ ] No `unsafe` without `// SAFETY:` justification comment
- [ ] Error handling uses `Result<T, E>` not `unwrap()` in production code
- [ ] All public functions have doc comments
- [ ] No `std::` imports — kernel is `#![no_std]`

## 8. Key Algorithms & Data Structures
- **Interrupt Descriptor Table (IDT):** 256-entry table mapping interrupts to handlers
- **Page Table:** 4-level (PML4 → PDPT → PD → PT) for x86_64; 4KB pages, 2MB huge pages
- **Slab Allocator:** Object cache with per-CPU magazines (inspired by Solaris SLAB)
- **RCU (Read-Copy-Update):** Lock-free read paths for frequently-read kernel structures
- **Per-CPU variables:** `#[thread_local]` or GS-base segment for per-core data
- **Linked list:** Intrusive doubly-linked list (Linux `list_head` style) in `src/kernel/util/list.rs`
- **Red-Black Tree:** For VMA tracking and scheduler runqueues

## 9. Security Considerations
- **KASLR:** Randomize kernel image base; never hard-code virtual addresses.
- **SMEP/SMAP:** Enable Supervisor Mode Execution/Access Prevention on x86_64.
- **NX bit:** Mark all non-code pages as non-executable.
- **Stack canaries:** Enabled via compiler flags (`-Z stack-protector=strong`).
- **Spectre/Meltdown mitigations:** Retpoline, IBPB, STIBP, KPTI (page table isolation).
- **Syscall filtering:** Baseline seccomp-equivalent built into the kernel, not userspace.
- **Pointer authentication (ARM64):** Enable PAuth for return address signing.

## 10. Performance Targets
| Metric | Linux 6.x | SigmaOS Target |
|--------|-----------|----------------|
| Syscall round-trip (getpid) | ~70ns | < 80ns |
| Context switch latency | ~1.5µs | < 2µs |
| IPI latency (SMP) | ~500ns | < 600ns |
| Page fault (minor) | ~200ns | < 300ns |
| Kernel compile time (allmodconfig) | baseline | ≤ 110% of baseline |

## 11. Testing Strategy
```rust
// Example test structure for kernel components
#[cfg(test)]
mod tests {
    use super::*;

    // Unit tests run in QEMU via cargo test --target x86_64-unknown-none
    #[test_case]
    fn test_idt_load() {
        // Verify IDT can be loaded without GP fault
    }

    #[test_case]
    fn test_page_allocator_alloc_free() {
        // Allocate and free pages, verify no double-free
    }

    #[test_case]
    fn test_syscall_dispatch_invalid() {
        // Verify invalid syscall numbers return ENOSYS
    }
}
```
- Use `bootimage`/`qemu-system-x86_64` for integration tests
- Fuzz syscall dispatcher with `cargo-fuzz`
- KASan-equivalent: use Miri for unsafe code analysis

## 12. Dependencies Between Components
- **Depends on:** Boot (provides memory map, framebuffer), Timekeeping (PIT/HPET for early delays)
- **Used by:** All other components — the kernel is the foundation

## 13. Maintenance Instructions for Future AI Agents
1. When Linux releases a new kernel version, check `Documentation/` and `MAINTAINERS` for changed subsystems.
2. Compare SigmaOS syscall table with Linux `arch/x86/entry/syscalls/syscall_64.tbl`.
3. Prioritize stability: never change a syscall's number or signature once published.
4. Always update this file when making changes to `src/kernel/`.
5. Wiki page for this component: `Kernel-Component-Guide` on GitHub Wiki.

## 14. References & Standards
- Linux source: `kernel/`, `arch/x86/`, `include/linux/`
- FreeBSD source: `sys/kern/`, `sys/amd64/`
- OpenBSD source: `sys/kern/`, `sys/arch/amd64/`
- Relevant specs: Intel SDM Vol. 3, AMD APM Vol. 2, ACPI 6.5 spec
- POSIX.1-2017 (IEEE Std 1003.1)
- SigmaOS Wiki page: `Kernel-Component-Guide`
