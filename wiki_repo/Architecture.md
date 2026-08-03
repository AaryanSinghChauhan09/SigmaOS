# Architecture

> See also: [docs/ARCHITECTURE.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/docs/ARCHITECTURE.md) in the repository.

## Kernel Design

SigmaOS uses a **microkernel-inspired** architecture. The kernel itself is minimal — scheduling, memory, and IPC — with drivers and services running in separate modules that can be isolated.

### Scheduler

Three scheduler variants are available, selected at compile time via `UseFlags`:

1. **MLFQ** (`sigma_mlfq.rs`) — Multi-Level Feedback Queue; penalizes CPU-bound processes, rewards interactive ones
2. **Thermal-Aware** (`sigma_thermal_sched.rs`) — throttles tasks when CPU temperature is high
3. **Transformer Scheduler** (`sigma_transformer_sched.rs`) — AI-hint driven scheduling using process behavior prediction

### Memory Management

```
Physical Memory Manager (PMM)
    ↓ provides frames to
Virtual Memory Manager (VMM)
    ↓ builds 4-level page tables
    ↓ feeds
Buddy Allocator (coarse)   →   Slab Allocator (fine-grained fixed-size)
```

All allocators in `src/klib/buddy_allocator.rs` and `src/kernel/slab_allocator.rs` are zero-`libc`, zero-`std`.

### Process Model

```
fork() → copy-on-write address space clone
exec() → load ELF, reset stack, jump to entry
exit() → signal parent via wait(), reclaim resources
```

Signals implemented: SIGTERM, SIGKILL, SIGCHLD, SIGSEGV, SIGINT, SIGALRM, SIGUSR1/2.

---

## Security Model

### Principle: Least Privilege by Default

Every process starts with zero capabilities and must explicitly request them through the capability enforcer (`src/security/capability_enforcer.rs`).

### Capability Token

```rust
pub struct CapabilityToken {
    bits: u64,  // each bit = one capability
}
// Bits 0-15:  basic capabilities (read, write, execute, fork)
// Bits 16-31: network port range
// Bits 32-47: device access
// Bits 48-63: security operations
```

The bitmask overlap bug (privilege escalation via OR on port range bits) was fixed: the field is now cleared before setting — `self.bits &= !(0xFFFF_u64 << 16)` before `|=`.

### MAC Policies

```
Process → System Call → Capability Enforcer
                              ↓
                        SELinux check
                              ↓
                        AppArmor check
                              ↓
                        Sigma-Pledge check
                              ↓
                        Allow / Deny + Audit Log
```

---

## VFS File Descriptor Model

SigmaOS implements the OpenBSD/Linux split:

- **`OpenFileDescription`** — system-wide object tracked by the kernel; holds offset, flags, inode ref
- **`FileDescriptor`** — per-process handle; index in process fd table → points to `OpenFileDescription`

This enables:
- `dup2()` — two fds share one `OpenFileDescription` (same offset)
- `fork()` — child inherits fd table pointing to same descriptions
- Correct POSIX semantics for concurrent file access

---

## IPC

- **Pipes** — anonymous in-kernel ring buffer
- **Unix sockets** — stream/datagram in the VFS namespace
- **Sigma Message Bus** — typed message passing for system services
- **Shared Memory** — mapped regions with capability-checked access

---

## Boot Sequence

```
UEFI Firmware
    ↓ (Secure Boot verifies signature)
sigma_boot_efi.rs (bootloader)
    ↓ (TPM 2.0 measures boot components)
    ↓ (loads kernel ELF)
Kernel entry point
    ↓ (init PMM, VMM, slab allocator)
    ↓ (init IDT, APIC, PIC)
    ↓ (init VFS with rootfs)
    ↓ (init scheduler)
    ↓ (spawn init process PID 1)
Runit supervisor / SigmaInit
    ↓ (start daemons: network, SSH, pkg, cron, screen reader)
Zenith Desktop
```
