# Security & Reliability in SigmaOS

SigmaOS is engineered for high-assurance environments where failure is not an option.

## 🔒 Memory Safety & Efficiency
- **Bounds Enforcement**: Use `sigma_size_t` for all buffer operations.
- **Zero-Trust Allocation**: Shards must request memory from the `SovereignMemoryManager` with a valid capability token.
- **Compaction Lattice**: Zenith v15.0 introduces the **Compaction Lattice**, which periodically shifts active heap segments to eliminate fragmentation holes in long-running professional workloads.
- **PQC Attestation**: All sensitive memory regions are attestation-locked using lattice-based crypto.

## 🛡 Fault Tolerance (Self-Healing)
The `S-AUTO` shard monitors the heartbeat of all active shards.
- **Watchdog Lattices**: Autonomous monitors that trigger `auto_heal()` if a shard stops responding.
- **Atomic Rollback**: Every professional action is recorded in a transactional log, allowing the system to revert to a stable state within 10ms of a fault.
- **Race-Safe ISRs**: Interrupt dispatch is protected by fine-grained atomic spinlocks to prevent race conditions during high-concurrency industrial events.

## 🧬 Process Isolation (Sandboxing)
- **S-WASM**: Professional tools run in a WebAssembly sandbox, preventing them from accessing kernel space directly.
- **Syscall Filtering**: Shards can only invoke syscalls registered in their `manifest.json`.

---
*Next: [API Reference](API-Reference.md)*
