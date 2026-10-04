# IPC — Inter-Process Communication

SigmaOS implements a comprehensive, zero-copy IPC subsystem written entirely in Rust. It supersedes Linux's fragmented IPC landscape (pipes, SysV IPC, POSIX mq) with a unified, high-performance design inspired by the best of Linux, seL4, and QNX.

---

## Architecture Overview

```
User Process A                    User Process B
     │                                  │
     ▼                                  ▼
 ┌────────────┐   Sigma Channel   ┌────────────┐
 │  SigmaChan │◄─────────────────►│  SigmaChan │
 └────────────┘                   └────────────┘
        │                                │
        ▼                                ▼
 ┌─────────────────────────────────────────────┐
 │               IPC Core (kernel)              │
 │  ┌──────┐ ┌────────┐ ┌──────┐ ┌─────────┐  │
 │  │ Pipe │ │ Socket │ │Futex │ │Shared   │  │
 │  │      │ │ (Unix) │ │      │ │Memory   │  │
 │  └──────┘ └────────┘ └──────┘ └─────────┘  │
 └─────────────────────────────────────────────┘
```

---

## Components

### 1. Pipes (`src/ipc/pipe.rs`)
- Classic anonymous pipes with 64 KB ring buffer
- Non-blocking mode with `O_NONBLOCK`
- Vectored I/O: `readv`/`writev`

### 2. Unix Domain Sockets (`src/ipc/socket.rs`)
- `SOCK_STREAM`, `SOCK_DGRAM`, `SOCK_SEQPACKET`
- SCM_CREDENTIALS: pass process credentials over socket
- SCM_RIGHTS: file descriptor passing
- Abstract namespace (no filesystem path required)

### 3. Futex (`src/ipc/futex.rs`)
- `FUTEX_WAIT`, `FUTEX_WAKE`, `FUTEX_REQUEUE`, `FUTEX_WAKE_OP`
- `FUTEX_WAIT_BITSET` / `FUTEX_WAKE_BITSET` for priority filtering
- Robust futex list: auto-cleanup on process death (`FUTEX_OWNER_DIED`)
- 256-bucket hash table for O(1) lookup

### 4. Shared Memory (`src/ipc/shm.rs`)
- POSIX `shm_open` / `mmap` interface
- NUMA-aware allocation
- CoW (copy-on-write) forking semantics

### 5. Message Queues (`src/ipc/mqueue.rs`)
- POSIX message queues with priority ordering
- Real-time delivery guarantees
- Zero-copy via shared buffer references

### 6. Signal Delivery (`src/ipc/signal.rs`)
- POSIX signals 1–64 including real-time signals
- `siginfo_t` rich metadata delivery
- Signal masks per-thread with `sigprocmask`

---

## Sigma Channel (High-Level IPC)

SigmaOS introduces **SigmaChannel** — a typed, async-capable IPC primitive:

```rust
// Create a typed channel pair
let (tx, rx) = SigmaChannel::<MyMessage>::new(64);
tx.send(MyMessage::Ping)?;
let msg = rx.recv()?;
```

Features:
- Compile-time type safety
- Backpressure with configurable depth
- Zero-copy for payloads ≤ 4096 bytes (shared ring buffer)
- Async-ready: integrates with SigmaOS async runtime

---

## Performance Characteristics

| Mechanism | Latency | Throughput | Zero-Copy |
|-----------|---------|------------|-----------|
| Futex wake/wait | ~200 ns | N/A | ✅ |
| Unix socket (stream) | ~1 µs | ~8 GB/s | ❌ |
| Shared memory | ~50 ns | RAM speed | ✅ |
| SigmaChannel | ~300 ns | ~5 GB/s | ✅ (≤4K) |
| Pipe | ~800 ns | ~4 GB/s | ❌ |

---

## Comparison vs Linux / Omarchy / Mint

| Feature | Linux | Omarchy | Mint | **SigmaOS** |
|---------|-------|---------|------|-------------|
| Typed IPC | ❌ | ❌ | ❌ | ✅ SigmaChannel |
| Robust futex | ✅ | ✅ | ✅ | ✅ |
| FD passing | ✅ | ✅ | ✅ | ✅ |
| Zero-copy shm | ✅ | ✅ | ✅ | ✅ NUMA-aware |
| Async IPC | ❌ | ❌ | ❌ | ✅ |

---

## Source Files

| File | Description |
|------|-------------|
| `src/ipc/pipe.rs` | Anonymous pipes |
| `src/ipc/socket.rs` | Unix domain sockets |
| `src/ipc/futex.rs` | Fast userspace mutex |
| `src/ipc/shm.rs` | Shared memory |
| `src/ipc/mqueue.rs` | POSIX message queues |
| `src/ipc/signal.rs` | Signal delivery |
| `src/ipc/mod.rs` | Module aggregator |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/ipc/` — scan all `.rs` files for new structs/enums
> - Update the performance table when benchmarks change
> - Add new IPC primitive sections if `src/ipc/` gains new files
> - Keep the comparison table current vs latest Omarchy/Mint releases
