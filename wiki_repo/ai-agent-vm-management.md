# AI Agent Virtual Machine Management in SigmaOS

## Overview

SigmaOS virtualization infrastructure (`src/virtualization/`, `src/virtualization/vm_manager.rs`, `src/virtualization/kvm_vcpu.rs`) provides hardware-assisted virtual machine guest management using native hypervisor interfaces (Linux KVM/QEMU, FreeBSD Bhyve, OpenBSD vmm, and macOS Hypervisor.framework interop).

AI agents (such as Herdr agentic task executors, automated build checkers, and security sandbox runners) can programmatically spin up, configure, and isolate guest virtual machines.

---

## Supported Hypervisor Backends

| Hypervisor Backend | Platform Parity | Description |
|--------------------|-----------------|-------------|
| **KVM / QEMU** | Linux / SigmaOS Native | Hardware VT-x/AMD-V acceleration with `KvmVcpuManager` |
| **Bhyve** | FreeBSD Parity | Minimalist BSD hypervisor with virtio device emulation |
| **vmm(4)** | OpenBSD Parity | Capability-pledged micro-VM sandbox manager |
| **Hypervisor.framework** | macOS Interop | macOS Virtualization.framework guest bridge |

---

## Programmatic VM Guest Provisioning

AI agents invoke `VirtualizationOrchestrator` to create and launch virtual machines:

```rust
use sigmaos::virtualization::{VirtualMachine, VirtualizationTech, VirtualizationOrchestrator};

let mut orchestrator = VirtualizationOrchestrator::new();

// Create isolated guest VM for agentic untrusted code execution
let mut vm = VirtualMachine::new(
    "vm-agent-sandbox-01".to_string(),
    "Untrusted Code Sandbox".to_string(),
    VirtualizationTech::KVM
).with_resources(
    4,    // 4 vCPUs
    2048, // 2048 MB RAM
    20    // 20 GB VirtIO Disk
);

// Boot guest VM
vm.start()?;
orchestrator.add_virtual_machine(vm)?;
```

---

## VirtIO Devices & Device Passthrough

AI agents configure isolated device passthrough for high-performance guest execution:

- **virtio-net**: Zero-copy network interface attached to a dedicated `VNET` or tap bridge.
- **virtio-blk**: Copy-On-Write (CoW) backing storage backed by `Hammer2PfsSnapshot` or `ZfsDataset`.
- **virtio-fs**: Shared host-guest directory tree bounded by OpenBSD `unveil()` restrictions.

---

## Guest Lifecycle Management & Health Monitoring

```rust
// Query guest VM state and resource usage
if let Some(vm) = orchestrator.get_vm("vm-agent-sandbox-01") {
    println!("Guest State: {:?}, RAM Usage: {} MB", vm.state, vm.allocated_ram_mb);
}

// Gracefully terminate guest upon task completion
orchestrator.stop_vm("vm-agent-sandbox-01")?;
```
