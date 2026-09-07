# SigmaOS AI Agent Context Providers Operation Management Guide

This guide defines protocols, context provider architecture, and state propagation standards for AI agents implementing and operating Context Providers across SigmaOS kernel, shell, and userland subsystems.

---

## 1. Overview of Context Providers in SigmaOS

Context Providers are structured components that capture, format, and supply execution environment state, hardware CPU register states, shell auto-completion contexts, system profile metrics, or Model Context Protocol (MCP) data payloads.

AI agents developing or modifying Context Providers MUST observe the following rules:

1. **Zero-Allocation Snapshot Providers:** Core kernel execution context providers (e.g., `TaskControlBlock`, process PCID/TLB context) MUST capture context snapshots without allocating heap memory on context switch paths.
2. **Thread-Safe Read Access:** Context providers shared across threads MUST expose thread-safe, lock-free or read-optimized interfaces (`AtomicRwLock`, RCU, or `AtomicUsize` version counters).
3. **Model Context Protocol (MCP) Compliance:** Agent-facing AI context bridges (e.g. `KimiCodeAgent`) MUST follow standardized JSON-framed Model Context Protocol schemas for tool definitions and context exchange.
4. **Context Invalidation Notifications:** When environment or hardware profile context changes (e.g., thermal throttling, power profile switch), the context provider MUST publish invalidation events to subscriber state graphs (`DeclarativeStateGraph` / `StateStore`).

---

## 2. Core Context Provider Categories

### A. Kernel & Process Execution Context Provider
Manages CPU hardware contexts (`TaskControlBlock`, register files, Process Context Identifiers - PCID, CR3 page table base):
* Captures user-to-kernel and thread-to-thread context switches with minimal latency ($\sim 13$ ns).
* Preserves vector/SIMD registers (`XMM`/`YMM`/`ZMM`) lazily on demand (`#NM` device not available fault).

### B. Shell & User Environment Context Provider (`ShellContext` & `ContextualCompleter`)
Supplies real-time shell environment context:
* Current working directory (`pwd`), active git branch, previous command status ($?$), active background jobs.
* Context-aware tab completion suggestions (`ContextualCompleter`) based on sub-command context.

### C. Model Context Protocol (MCP) Context Provider (`KimiCodeAgent`)
Exposes structured system context to AI coding agents and external tools:
* Returns JSON-framed schemas of available system tools, active terminal context, and kernel diagnostics.

---

## 3. Context Provider Interface Pattern

```rust
pub trait ContextProvider {
    type ContextData;

    /// Captures a fresh context snapshot
    fn capture_context(&self) -> Self::ContextData;

    /// Returns the provider's unique context category name
    fn provider_name(&self) -> &'static str;

    /// Checks if the cached context remains valid
    fn is_valid(&self) -> bool;
}
```
