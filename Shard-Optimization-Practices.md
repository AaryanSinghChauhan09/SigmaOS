# SigmaOS Shard Optimization Practices

This document outlines the strict guidelines required to maintain the ultra-low latency profile of SigmaOS Shards across all deployment formats.

## 1. IPC and Orchestration

* **Minimize Context Switches**: Shards must batch IPC messages when communicating with the Sovereign Orchestrator.

* **Wait-Free Ring Buffers**: Utilize the `SigmaOS::SovereignRingBuffer` for all data streaming between shards instead of blocking synchronization primitives like mutexes.

## 2. Memory Usage (S-MM)

* **Pre-Allocation**: Shards in the `standalone` and `real-time` branches must pre-allocate all memory pools during the `init()` phase. Dynamic allocation (`sigma_malloc`) during critical loop execution is forbidden to ensure deterministic execution.

* **Slab Caching**: Reuse objects using the `SovereignMemoryPool` to prevent fragmentation overhead in long-running processes (e.g., dual-boot persistent background tasks).

## 3. Power and Responsiveness (App/Browser Layer)

* **Event-Driven Execution**: GUI shards must strictly yield execution context (`sigma_yield`) when waiting on asynchronous UI events to optimize battery usage in mobile/laptop configurations.

* **WASM Acceleration**: Any browser plugins should be compiled ahead-of-time (AOT) to WASM and utilize the hardened caching layer to prevent repeated JIT interpretation overhead.

## 4. Formal Verification for Safety Criticality

For industrial/embedded use cases, any shard modifying hardware states or operating in Ring-0 must be audited using the `scripts/fuzz_pqc.sh` and formal verification toolchains to prevent race conditions or unsafe memory escalation.
