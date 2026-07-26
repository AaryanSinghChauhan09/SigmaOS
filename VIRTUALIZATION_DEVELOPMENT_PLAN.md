# 🐳 SigmaOS Virtualization & Container Sandboxing Development Plan

This document details the architectural design and implementation plan for the **SigmaOS Virtualization & Container Subsystem**, taking inspiration from the lightweight hypervisor concepts of **AWS Firecracker** (minimal KVM microVMs) and the immutable orchestration layers of **Talos Linux**.

---

## 🗺️ Architectural Inspiration
*   **AWS Firecracker:** Implements minimalist microVMs over standard kernel virtualization, dropping legacy device support to achieve millisecond-level boot times.
*   **Talos Linux:** Completely removes SSH/console shells, enforcing immutable API-driven operations where containers run directly under strict security control.

---

## 🏗️ OOP Design & MicroVM State Machine

SigmaOS leverages native capability gates to orchestrate virtualization without massive daemon dependencies:

```text
  [Orchestration API]
          |
          v
  +-------------------------------------------------+
  |            VirtualizationOrchestrator           |
  +-------------------------------------------------+
          |
          +---> [MicroVmInstance]  --> direct hardware page sharing (jail)
          |
          +---> [NamespaceContainer] --> capability-pledge isolated jail
```

### MicroVM State Transitions:
```text
  State::Defined ➡️ State::Booting ➡️ State::Running ➡️ State::Paused ➡️ State::Exited
```

### Polymorphic Sandbox Interface:
```rust
pub trait VirtualSandbox {
    fn start(&mut self) -> Result<(), SandboxError>;
    fn stop(&mut self) -> Result<(), SandboxError>;
    fn configure_limits(&mut self, limits: ResourceLimits) -> Result<(), SandboxError>;
    fn get_status(&self) -> SandboxStatus;
}
```

---

## 🛠️ Multi-Language Architecture (Rust, Zig, Nim)

### ⚡ Rust: MicroVM Context Launcher
```rust
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_cores: u32,
    pub max_memory_mb: u64,
}

pub struct MicroVmInstance {
    pub id: String,
    pub limits: ResourceLimits,
    pub running: bool,
}

impl MicroVmInstance {
    pub fn new(id: String, limits: ResourceLimits) -> Self {
        Self { id, limits, running: false }
    }

    pub fn execute_boot(&mut self) -> Result<(), &'static str> {
        // Enforce limits and boot directly into bare-metal guest image
        self.running = true;
        Ok(())
    }
}
```

### ⚡ Zig: Jail/Namespace Controller (Clone/Unshare)
```zig
const std = @import("std");

pub fn createSandboxJail(uid: u32, gid: u32) !void {
    // Mimicking Linux unshare/pivot_root using native system boundaries
    const flags = std::os::linux.CLONE.NEWPID | std::os::linux.CLONE.NEWNET | std::os::linux.CLONE.NEWNS;
    const rc = std::os::linux.unshare(flags);
    if (rc != 0) {
        return error.UnshareFailed;
    }
}
```

### ⚡ Nim: Sandboxed Resource Poller
```nim
type
  SandboxMetrics* = object
    cpuUsagePercent*: float64
    memoryUsageBytes*: uint64

proc querySandboxMetrics*(sandboxId: string): SandboxMetrics {.exportc, cdecl.} =
  # Query native cgroups/resource metrics
  result.cpuUsagePercent = 12.5
  result.memoryUsageBytes = 256 * 1024 * 1024
```

---

## 📈 Quality Assurance & Limit Audits

1.  **Isolation Boundary Test:** Attempt to execute privileged syscalls from within guest microVMs and verify complete containment.
2.  **Startup Benchmark:** Audit bootup latencies to ensure containers and microVMs launch in < 10ms.
