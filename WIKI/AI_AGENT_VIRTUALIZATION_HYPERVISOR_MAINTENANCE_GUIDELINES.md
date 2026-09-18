# AI Agent Virtualization & Hypervisor Subsystem Maintenance Guidelines

This document provides architectural standards and maintenance protocols for AI agents developing and maintaining virtualization technologies, hypervisor bridges, VirtIO drivers, FreeBSD Jails, and container engines within SigmaOS.

---

## 1. Subsystem Overview & Core Architecture

SigmaOS delivers multi-tier virtualization capabilities including hardware-assisted Type-1/Type-2 hypervisor passes (KVM / Bhyve parity), capability-isolated FreeBSD Jails, Linux cgroups v2 + namespaces container runtimes, and VirtIO high-speed device emulation.

### Core Modules:
- `src/virtualization/` / `src/compatibility/bsd.rs`: FreeBSD Jails virtualized network stacks (`VNET`) and process isolation.
- `src/container/runtime.rs`: Lightweight container engines, namespace unsharing, and cgroups v2 resource controllers.
- `src/drivers/virtio/` / `src/drivers/distro_device_expansion.rs`: VirtIO-Net, VirtIO-Block, VirtIO-SCSI, and VirtIO-GPU drivers.
- `src/kernel/hypervisor.rs`: Hardware virtualization extensions (Intel VT-x / AMD-V / ARM VE).

---

## 2. Maintenance & Development Guidelines for AI Agents

### 2.1 Hardware Hypervisor Extensions (VT-x / AMD-V)
1. **EPT/NPT Page Table Invariants:** Nested page tables must enforce non-executable stack bits and track guest-physical to host-physical frame mappings without allocation panics.
2. **VM-Exit Handling:** VM-exit loops must process I/O instructions, MMIO intercepts, and hypercalls with sub-microsecond latency.

### 2.2 FreeBSD Jails & Container Security
1. **Capability Mode Gating:** Processes inside Jails or sandboxed containers must be checked against `Capsicum` rights matrices before executing privileged system calls.
2. **VNET Network Stack Virtualization:** Virtual network interfaces assigned to Jails must be isolated with separate route tables, socket lookup spaces, and firewall rule contexts.

### 2.3 Diagnostic Protocol & Self-Verification
AI agents must run the following verification suite after modifying virtualization components:
```bash
# Verify BSD Jails sovereign test suite
cargo test --test bsd_jails_sovereign

# Verify container runtime suite
cargo test --lib container::runtime

# Execute complete system tests
./run_sigma_tests.sh
```

---

## 3. Safe Rust Code Blueprints

### 3.1 VirtIO Ring Queue Dispatcher Blueprint
```rust
use core::sync::atomic::{AtomicU16, Ordering};

pub struct VirtioRingQueue {
    pub ring_size: u16,
    pub free_head: AtomicU16,
}

impl VirtioRingQueue {
    pub const fn new(ring_size: u16) -> Self {
        Self {
            ring_size,
            free_head: AtomicU16::new(0),
        }
    }

    pub fn allocate_descriptor(&self) -> Result<u16, &'static str> {
        let head = self.free_head.load(Ordering::Relaxed);
        if head >= self.ring_size {
            return Err("VirtIO Queue Full");
        }
        self.free_head.fetch_add(1, Ordering::SeqCst);
        Ok(head)
    }
}
```

---

*Verified & Enforced for SigmaOS AI Agents.*
