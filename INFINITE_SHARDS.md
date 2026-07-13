# Infinite Sharding Specification

This specification outlines the architecture of **Infinite Sharding**, the dynamic module-loading model of SigmaOS that allows features to be registered, unloaded, and updated at runtime without system reboots.

---

## 🌀 Concept & Dynamic Registration

In traditional microkernels, changing system components requires recompiling the root system. SigmaOS replaces this constraint with the **Sovereign Lattice Shard Model**, where each component runs as a decoupled state machine communicating via the `sigma-bus` IPC event loop.

```
       [sigma-bus (Zero-copy IPC Shared Ring Buffer)]
         ▲                     ▲                     ▲
         │                     │                     │
         ▼                     ▼                     ▼
┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐
│   Core Shard    │   │ Essential Shard │   │  Dynamic Shard  │
│  (Memory / PMM) │   │ (Network Stack) │   │  (e.g., WiFi)   │
└─────────────────┘   └─────────────────┘   └─────────────────┘
                                                     ▲
                                                     │  Dynamic Load
                                             [sigpkg install]
```

---

## ⚡ Zero-Copy Messaging (`sigma-bus`)

Shards pass message structures via shared page frames. Rather than copying bytes across address spaces, the kernel swaps physical page pointers in translation tables (Page Table manipulation), achieving **zero-copy** latency.

```rust
// include/ipc/SovereignEventBus.h (Rust wrapper equivalent)
pub struct EventBusDescriptor {
    pub ring_buffer_address: usize,
    pub ring_buffer_size: usize,
    pub head_pointer: *mut u32,
    pub tail_pointer: *mut u32,
}

impl EventBusDescriptor {
    pub unsafe fn publish_event(&mut self, event_type: u32, payload_page: usize) {
        // Enqueue event referencing the physical memory page frame base address.
        // Swap read/write ownership tokens without copying memory payload.
    }
}
```

---

## 🛠️ Security and Isolation Verification

Dynamic modules present safety risks. To prevent unstable or malicious shards from corrupting system states, the loader (`SovereignInit`) applies strict gates:

1. **SPARK Validation**: Shards compiled from Ada/SPARK must contain complete verification proofs proving memory safety and absence of run-time exceptions before they are accepted.
2. **PQC Signature Check**: Every shard `.spkg` contains a digital signature verified against the Root Sovereign Key using the **Dilithium5** post-quantum algorithm.
3. **Hardware Sandbox**: Ring 3 optional shards run with paging structures that block access to other shard memory regions, monitored by the memory manager.
