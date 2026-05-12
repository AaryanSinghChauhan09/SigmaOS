# Security & Reliability in SigmaOS

SigmaOS is engineered for high-assurance environments where failure is not an option.

## 🔒 Memory Safety
- **Bounds Enforcement**: Use `sigma_size_t` for all buffer operations.
- **Zero-Trust Allocation**: Shards must request memory from the `SovereignMemoryManager` with a valid capability token.
- **PQC Attestation**: All sensitive memory regions are attestation-locked using lattice-based crypto.

## 🛡 Fault Tolerance (Self-Healing)
The `S-AUTO` shard monitors the heartbeat of all active shards.
- **Watchdog Lattices**: Autonomous monitors that trigger `auto_heal()` if a shard stops responding.
- **Atomic Rollback**: Every professional action is recorded in a transactional log, allowing the system to revert to a stable state within 10ms of a fault.

## 🧬 Process Isolation (Sandboxing)
- **S-WASM**: Professional tools run in a WebAssembly sandbox, preventing them from accessing kernel space directly.
- **Syscall Filtering**: Shards can only invoke syscalls registered in their `manifest.json`.

---
*Next: [API Reference](API-Reference.md)*
