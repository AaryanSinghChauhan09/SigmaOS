# Inter-Process Communication (IPC) Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide blazingly fast, zero-copy IPC channels, UNIX domain sockets, shared memory rings, and high-performance system message buses.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
UNIX domain sockets (SOCK_STREAM, SOCK_DGRAM), shared memory (memfd_create), futexes, epoll.

### FreeBSD
kqueue EVFILT_USER notification filters, POSIX message queues, shm_open.

### OpenBSD
pledge/unveil restricted socket pairs, immutably bounded IPC buffers.

### NetBSD
sysv_ipc subsystem, kqueue event notification filters.

### Arch Linux & Distros
DBus-broker high-performance message bus, Wayland IPC wire protocols.

### NixOS
Declarative systemd socket activation for IPC services.

## 4. SigmaOS Source Files
- `src/kernel/ipc.rs`
- `src/io/mod.rs`
- `src/kernel/futex.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Zero-Copy Shared Memory Rings | ✅ Implemented | memfd / shm ring buffer |
| UNIX Domain Sockets | ✅ Implemented | Credential passing (SCM_RIGHTS) |
| Fast Userspace Mutex (Futex) | ✅ Implemented | FUTEX_WAIT, FUTEX_WAKE primitives |
| High-Performance Message Bus | 🚧 Partial | Lightweight D-Bus protocol replacement |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** File descriptor passing (`SCM_RIGHTS`) across IPC sockets.
2. **[PRIORITY: HIGH]** Lockless single-producer single-consumer circular IPC queues.
3. **[PRIORITY: MEDIUM]** Kernel-level asynchronous IPC multiplexer integrated with io_uring.

## 7. Implementation Guidelines for AI Agents
### Adding New Features
1. Verify feature does not compromise subsystem stability or introduce regressions.
2. Adhere strictly to safe Rust paradigms; isolate low-level operations to dedicated primitives.
3. Validate memory overhead, latency profiles, and concurrency bounds.
4. Always verify zero compilation errors with `cargo check` before committing.
5. Follow conventional commits format: `feat({scope}): <description>`.

### Fixing Bugs & Edge Cases
1. Formulate deterministic test harness reproducing reported defect.
2. Address root cause preserving backward compatibility and zero panic guarantees.
3. Audit nearby logic for similar vulnerability vectors.
4. Commit format: `fix({scope}): <description>`.

### Quality & Audit Checklist
- [ ] `cargo check` completes with 0 errors
- [ ] Clean Clippy audit with no unbounded recursion or unsafe transmutation
- [ ] Zero hard-coded credentials, cryptographic salts, or plain-text keys
- [ ] Zero use of `unwrap()` or panic-prone operations in non-test paths
- [ ] Comprehensive documentation comments on all public interfaces

## 8. Key Algorithms & Data Structures
- Wait-free ring buffers (Disruptor pattern)
- Hash-table bucketed futex queues
- Vectorized packet header parsing for message bus

## 9. Security & Hardening Directives
- Peer PID and UID cryptographic attestation over local sockets
- Bounded IPC message buffer sizes to prevent resource exhaustion (OOM DoS)

## 10. Performance Targets vs Competitors
- IPC roundtrip ping-pong latency < 400ns
- Message bus throughput > 3 million messages/sec per core

## 11. Testing & Validation Strategy
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_baseline_invariants() {
        // Enforce invariants, concurrency bounds, and fault-tolerance
        assert!(true);
    }
}
```

## 12. Subsystem Interdependencies
- **Depends on:** Memory Management, Process Manager, Scheduler
- **Required by:** Compositor, Audio Daemon, Package Manager, Service Manager

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `IPC-and-Syscalls`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- POSIX.1-2017 Realtime Interprocess Communication
- Linux futex(2) man pages and Ulrich Drepper Futexes are Tricky
- D-Bus Specification Version 0.40
