# Core Shards Architecture

> **Status**: ACTIVE | **Component**: `kernel` & `shards`

The SigmaOS Sovereign Lattice is built upon a decentralized hierarchy of "Shards". The most privileged of these are the **Core Shards**, which operate at Ring 0 or Ring 1 and form the irreducible minimum functionality required for the system to boot and enforce security.

---

## 1. Core Shard Hierarchy

Unlike monolithic kernels where all subsystems share Ring 0 memory, SigmaOS restricts Ring 0 exclusively to `S00_KERNEL` and `S01_MEM_MANAGER`. All other core shards operate in Ring 1 or Ring 3, isolated via hardware paging and interacting strictly via the `sigma-bus`.

| ID | Name | Ring | Required Capabilities | Description |
|---|---|---|---|---|
| **S00** | `S00_KERNEL` | 0 | `CAP_ADMIN` | The microkernel core. Manages interrupts, contexts, and capabilities. |
| **S01** | `S01_MEM_MANAGER` | 0 | `CAP_MEMORY` | Virtual Memory Manager (VMM), Page Allocator, and hardware paging. |
| **S02** | `S02_IPC_BROKER` | 1 | `CAP_IPC` | Routes `sigma-bus` zero-copy messages between all other shards. |
| **S03** | `S03_VFS` | 1 | `CAP_STORAGE` | Virtual File System. Resolves paths and mounts filesystems. |
| **S04** | `S04_CRYPTO` | 1 | `CAP_CRYPTO` | Sovereign Cryptography Shard (PQC signatures, ED25519, BLAKE3). |

## 2. Isolation Properties

Core Shards enforce isolation at two levels:

1.  **Hardware Level (Paging)**: `S01_MEM_MANAGER` creates distinct PML4 tables (x86_64) or translation tables (ARM64) for each shard. `S02` cannot read `S03`'s memory.
2.  **Capability Level (Tokens)**: When a shard invokes a syscall (e.g., `sys_mmap`), the `syscall_dispatcher` validates its cryptographically signed Capability Token. If `S03_VFS` attempts to call `sys_mmap` without `CAP_MEMORY`, the request is rejected with `EACCES`.

## 3. Recovery Protocols

Core Shards are managed by the Self-Healing Engine (`recovery.rs`). If a Core Shard panics or stalls:

-   **S02, S03, S04**: The watchdog triggers an atomic restart of the shard. Existing IPC messages in the ring buffer are re-routed once the shard recovers.
-   **S00, S01**: A panic in the core microkernel or memory manager is unrecoverable at runtime. The system logs a forensic dump to NVS and initiates a hard reboot.
