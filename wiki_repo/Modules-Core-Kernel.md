# Σ core/kernel — Sovereign Microkernel Core

Minimal kernel handling **scheduling, memory management, IPC, syscalls, and
watchdog supervision** — structured as composable sovereign shards rather than a
monolithic blob.

## Architecture

```
              ┌─────────────────────────────────┐
              │          kernel_main.rs          │
              │  (boot → init subsystems → idle) │
              └───────────┬─────────────────────┘
        ┌─────────────────┼──────────────────────┐
     Scheduler         Memory               Security
   (scheduling/)     (memory/)            (security/)
        │                │                     │
      ipc.rs         res_alloc.rs        syscalls.rs
   interrupts.rs     rollback_manager    watchdog.rs
   kernel.rs         self_heal.rs        audit_shard.rs
```

## Key Source Files

| File | Description |
|---|---|
| `kernel.rs` / `kernel_main.rs` | Boot entry — wires all sovereign subsystems |
| `init.rs` | Early hardware init (GDT, IDT, TSS) |
| `interrupts.rs` | IRQ/exception handler table |
| `ipc.rs` | Zero-copy inter-shard message passing |
| `syscalls.rs` | Sovereign syscall dispatcher (non-POSIX ABI) |
| `res_alloc.rs` | Deterministic resource allocator |
| `res_alloc_ai.rs` | AI-assisted resource allocation hints |
| `watchdog.rs` | Hardware-backed watchdog timer integration |
| `self_heal.rs` | Autonomous shard fault detection & restart |
| `rollback_manager.rs` | Snapshot-based kernel state rollback |
| `audit_shard.rs` | Immutable audit log for all syscalls |
| `elf_loader.rs` | ELF binary loader for user-space processes |

## API Interface

```rust
// Kernel entry point (called from bootloader)
#[no_mangle]
pub unsafe extern "C" fn kernel_main() { ... }

// Spawn a new sovereign shard
pub fn shard_spawn(name: &str, caps: &[Capability]) -> ShardId;

// Send a message to another shard (zero-copy)
pub fn shard_send(dst: ShardId, msg: &SovereignMsg) -> KernelResult;

// Syscall dispatcher
#[no_mangle]
pub unsafe extern "C" fn sigma_syscall(nr: u64, args: *const u64) -> i64;
```

## Capability System

Every shard is granted a minimal capability set at spawn time:

```json
{
  "capabilities_required": ["CAP_KERNEL_ROOT"],
  "capabilities_provided": ["CAP_BOOT_CONTEXT"],
  "entry_point": "kernel_main"
}
```

## Roadmap

- [x] IRQ handler table (`interrupts.rs`)
- [x] Zero-copy IPC (`ipc.rs`)
- [x] Syscall dispatcher (`syscalls.rs`)
- [x] Watchdog timer (`watchdog.rs`)
- [x] Self-healing shard restart (`self_heal.rs`)
- [x] AI resource allocation hints (`res_alloc_ai.rs`)
- [ ] Full NUMA-aware memory allocator
- [ ] Formal Kani proofs for IPC non-interference
- [ ] Live kernel patching (hot-patch without reboot)
- [ ] Microkernel split: move drivers fully out of Ring 0

## Sub-Directories

- [`memory/`](memory/) — Paging, slab allocator, NUMA topology
- [`scheduling/`](scheduling/) — Round-Robin, EDF, real-time lanes
- [`security/`](security/) — Capability enforcement hooks
- [`syscalls/`](syscalls/) — Per-syscall implementation shards
- [`hypervisor/`](hypervisor/) — Type-1 hypervisor (VT-x / AMD-V)

## Related Modules

- [`modules/core/drivers`](../drivers/README.md) — Hardware drivers
- [`modules/core/net`](../net/README.md) — Network stack
- [`modules/tools/diag`](../../tools/diag/README.md) — Kernel diagnostics
