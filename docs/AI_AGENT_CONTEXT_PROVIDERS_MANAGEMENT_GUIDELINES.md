# SigmaOS AI Agent Context Providers Management Guidelines

## 1. Executive Summary & Overview

Context Providers in SigmaOS serve as standardized telemetry, state, and metadata aggregators that feed dynamic runtime context into AI agents and local LLM models. To make informed system management, security enforcement, and resource allocation decisions, AI agents query registered context providers across kernel, userland, hardware, and file system subsystems.

This document establishes the official guidelines and architectural standards for AI agents managing, querying, and extending Context Providers in SigmaOS.

---

## 2. Core Architectural Context Provider Domains

SigmaOS categorizes Context Providers into seven primary operational domains:

| Context Provider Domain | Source Subsystem / Module | Provided Context Data |
| :--- | :--- | :--- |
| **System & Distro State** | `src/distro/` | Active distro mode (Nix, Guix, Arch, Debian, BSD), sysctl parameters, service status |
| **Process & PCB Context** | `src/kernel/` | Process lifecycle, CPU usage, cgroup limits, thread wait states (`TASK_INTERRUPTIBLE`) |
| **Hardware & PMC Telemetry** | `src/hal/`, `src/drivers/` | CPU performance counters, NVMe status, NIC link speeds, temperature/voltage sensors |
| **Memory & Storage Context** | `src/memory/`, `src/filesystem/` | Page frame pressure, Btrfs snapshots, ZFS ARC hit rates, LUKS2 volume states |
| **Security & Capability Context** | `src/security/` | Capability tokens, eBPF LSM audit logs, OpenBSD pledge/unveil bounds, SELinux SECMARK labels |
| **Desktop & UX Context** | `src/desktop/`, `src/ui/` | Active Wayland compositor, window geometry, fractional scale factor, active theme palette |
| **Package & Shard Marketplace** | `src/sigpkg/`, `src/package/` | Installed package manifests, dependency graphs, CAS store hash verification |

---

## 3. Context Provider Interface & Query Protocol

### 3.1 Standardized Context Provider Trait

All context providers implement a zero-allocation, thread-safe interface:

```rust
pub trait IContextProvider: Send + Sync {
    /// Unique provider identifier (e.g. "sys.hardware.pmc", "security.ebpf.lsm")
    fn provider_id(&self) -> &str;

    /// Provider domain classification
    fn domain(&self) -> ContextDomain;

    /// Aggregates current snapshot of context into structured key-value telemetry
    fn fetch_context(&self) -> Result<ContextSnapshot, &'static str>;

    /// Refresh interval frequency in milliseconds
    fn refresh_interval_ms(&self) -> u32;
}
```

---

### 3.2 Context Aggregation & Prompt Injection Pipeline

1. **Non-Blocking Query Execution**:
   - AI agents query context providers asynchronously using non-blocking read-locks or lock-free RCU snapshots (`rcu_read_lock()`).
2. **Context Window Token Budgeting**:
   - Context snapshots are prioritized by domain weight (Security > Kernel > Storage > UX) and trimmed to fit within the local LLM context window bounds.
3. **Data Sanitization & Privacy Boundaries**:
   - Sensitive user secrets (passwords, private cryptographic keys, user tokens) are automatically redacted before passing context snapshots to AI agent prompt pipelines.

---

### 3.3 Dynamic Context Provider Registration

Custom context providers can be registered dynamically with the global `ContextProviderRegistry`:

- **Capability Gate Check**: Registering a new context provider requires the `CAP_CONTEXT_PROVIDER_REGISTER` token.
- **Sanitization Audit**: New context providers must pass eBPF LSM policy validation to prevent unauthorized kernel memory leaks.

---

## 4. Verification & Concurrency Protocols

AI agents modifying or registering Context Providers must pass verification:

1. **Subsystem Suite**: Run `./run_sigma_tests.sh` to confirm zero-regression execution across all context provider modules.
2. **Stress & Fuzzing Matrix**: Run `tests/stress_and_fuzz_tests.rs` to verify zero memory leaks and thread safety under high multi-threaded context query load.

---

*Approved by the SigmaOS AI Agent Architecture Steering Committee.*
