# RFC: Linux Compatibility Layer Design

**Status:** Active
**Version:** 1.0
**Date:** 2026-07-03

---

## Summary

This RFC defines the three-layer Linux compatibility architecture for SigmaOS, enabling Linux binaries and containers to run with minimal friction while maintaining SigmaOS's security and sovereignty guarantees.

---

## Motivation

Without a Linux compatibility story, SigmaOS adoption is blocked by:

- Users can't run their existing Linux applications

- Servers can't migrate without rewriting every service

- Containers (Docker/OCI) are the dominant deployment unit

The goal is **zero-friction migration** for Linux workloads while keeping the SigmaOS kernel clean.

---

## Three-Layer Architecture

```
Layer 3: AI Porter      — study Linux driver structure, generate SDF native
Layer 2: Distro Shim    — export Linux kernel symbols, redirect to SigmaOS HAL
Layer 1: MicroVM/OCI    — run unmodified Linux containers in isolated VM
```

### Layer 1: MicroVM + OCI (ship immediately)

Lightest-weight path. Run Docker/OCI images in a Firecracker-style microVM.

**Implementation:** `virtualization/ocirunner/`

```bash

# Run an OCI image

sigma-compat container ubuntu:22.04 /bin/bash
sigma-compat container nginx:latest

# Run a Linux binary directly

sigma-compat run /path/to/linux-binary --arg1 --arg2
```

Security: microVM is fully isolated. Linux kernel runs in the VM; SigmaOS kernel is not exposed.

### Layer 2: Distro Compat Shim (mid-term)

For pre-built Linux kernel modules that need to run inside SigmaOS kernel-space.

**Implementation:** `drivers/linux_distros/compat.rs`

Exports 25 Linux kernel symbols (see [Linux-Driver-Compat](../wiki_repo/Linux-Driver-Compat.md)):

```c
// These are resolved by the LKM loader
void printk(const char* fmt, ...);       // → sigma_log
void* kmalloc(size_t size, gfp_t flags); // → sigma_slab_alloc
void kfree(void* ptr);                   // → sigma_slab_free
void __iomem* ioremap(phys_addr_t, size_t); // → sigma_iomap
int request_irq(int irq, irq_handler_t); // → sigma_request_irq
```

### Layer 3: Syscall Translation (linuxulator, long-term)

ELF loader + syscall translation table. Linux binary loads and runs on SigmaOS kernel directly.

**Implementation:** `kernel/linux_compat/`

---

## Prioritized Syscall List (Phase 1 — 50 syscalls)

Ordered by frequency across top-100 server workloads (profiler data):

| Priority | Syscall | Status | Complexity |
|---|---|---|---|
| 1 | `read` / `write` | ⬜ stub | Low |
| 2 | `open` / `close` | ⬜ stub | Low |
| 3 | `mmap` / `munmap` | ✅ partial | Medium |
| 4 | `fstat` / `stat` | ⬜ stub | Low |
| 5 | `clone` / `fork` | ⬜ stub | High |
| 6 | `execve` | ⬜ stub | High |
| 7 | `futex` | ✅ partial | Medium |
| 8 | `epoll_create1` / `epoll_ctl` / `epoll_wait` | ⬜ stub | Medium |
| 9 | `socket` / `connect` / `bind` / `listen` / `accept` | ⬜ stub | Medium |
| 10 | `getpid` / `getppid` | ✅ | Low |
| 11 | `nanosleep` | ✅ | Low |
| 12 | `clock_gettime` | ✅ | Low |
| 13 | `brk` | ✅ partial | Low |
| 14 | `ioctl` | ⬜ stub | Medium |
| 15 | `pipe2` / `dup2` | ⬜ stub | Low |
| 16 | `kill` / `sigaction` | ⬜ stub | Medium |
| 17 | `wait4` / `exit_group` | ⬜ stub | Medium |
| 18 | `getcwd` / `chdir` / `mkdir` | ⬜ stub | Low |
| 19 | `getrandom` | ✅ | Low |
| 20 | `uname` | ✅ | Low |
| 21 | `pread64` / `pwrite64` | ⬜ stub | Low |
| 22 | `readv` / `writev` | ⬜ stub | Low |
| 23 | `fcntl` | ⬜ stub | Low |
| 24 | `set_tid_address` | ✅ | Low |
| 25 | `memfd_create` | ⬜ stub | Low |

---

## ELF Loader Design

`kernel/linux_compat/elf_loader.rs` (to be implemented):

1. Validate ELF magic: `\x7fELF`

2. Check `e_machine = EM_X86_64`

3. Parse `PT_LOAD` segments, map to virtual addresses

4. Set up `auxv` (auxiliary vector) for glibc

5. Place `argc`/`argv`/`envp` on stack per Linux ABI

6. Set `%rsp` to stack top, jump to entry point

The process runs in a "linux-compat namespace" — all syscalls routed through `sigma_syscall_dispatch()` which translates them.

---

## vDSO Shim

Linux programs call `clock_gettime()` via vDSO (no syscall overhead). SigmaOS must provide a compatible vDSO page mapped into every linux-compat process.

The vDSO exposes:

- `__vdso_clock_gettime` → `sigma_clock_ns()`

- `__vdso_gettimeofday` → derived from above

- `__vdso_time` → Unix epoch

---

## /proc and /sys Shims

Many Linux programs read `/proc/self/maps`, `/proc/cpuinfo`, `/sys/class/net/`, etc.

Minimum viable shim:
```
/proc/self/maps     → VMA list from current process VmSpace
/proc/cpuinfo       → static content describing the CPU
/proc/meminfo       → sigma_mm_free_pages() converted to Linux format
/sys/class/net/     → enumerate sigma-bus NIC channels
/dev/urandom        → sigma_getrandom()
```

---

## Security Constraints

Linux-compat processes run under:
```
sigma_pledge("stdio rpath wpath exec proc inet")
```

They cannot access kernel internal structures. The syscall translation layer applies pledge checks before forwarding to native kernel primitives.

---

## Test Plan

1. **LTP subset**: Run Linux Test Project on linux-compat ELF loader

2. **nginx**: Multi-threaded server must pass basic load test

3. **glibc**: Verify pthread_create, mutex, condvar work via futex translation

4. **OCI images**: Top-20 server images must boot in microVM

---

*See also: [Linux-Driver-Compat](../wiki_repo/Linux-Driver-Compat.md) · [Architecture.md](../Architecture.md)*
