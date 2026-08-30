# SigmaOS VMM / QEMU / KVM Enhancements

## Overview

SigmaOS ships a Rust-native Virtual Machine Monitor (VMM) built on top of the Linux KVM API, with significant enhancements over upstream QEMU for security, performance, and observability.

***

## Architecture

    +---------------------------+
    |     SigmaOS VMM Layer     |
    |  (Rust, src/vmm/)         |
    +---------------------------+
              |
       +------+------+
       |             |
    +------+    +----------+
    | KVM  |    |  QEMU    |
    | API  |    | (Bridge) |
    +------+    +----------+
       |             |
    +---------------------------+
    |    Hardware (x86-64/ARM)  |
    +---------------------------+

***

## Key Enhancements

### 1. Rust-Native VMM (`src/vmm/`)

*   Written entirely in safe Rust (no C/C++ in the hot path)
*   Memory-safe device emulation (no buffer overflows possible)
*   Structured logging of all VM exit events
*   eBPF-based VM introspection without pausing the guest

### 2. QEMU/KVM Security Hardening

*   QEMU processes run in isolated jails (pledge + Capsicum)
*   VM memory backed by sealed `memfd_create()` with no-execute pages for QEMU process
*   Virtio device ring buffers verified with Rust's borrow checker
*   IOMMU enforcement for all PCI pass-through devices

### 3. Nested Virtualization

*   Support for nested VMs (KVM-in-KVM)
*   Shadow page table optimization for L2 guests
*   VPID/VMID caching for L1/L2 TLB management

### 4. Live Migration

*   Dirty page tracking via `KVM_CAP_DIRTY_LOG_RING`
*   Pre-copy algorithm with post-copy fallback
*   Network bandwidth throttling during migration
*   Automatic checkpointing every 30 seconds

### 5. Performance Optimizations

*   **Huge Pages** (1GB/2MB) for guest physical memory
*   **io\_uring** for all block and network I/O
*   **vhost-user** for network and storage offload to dedicated processes
*   **CPU pinning** with NUMA-aware memory allocation
*   **Balloon driver** for dynamic memory reclamation

### 6. GPU Virtualization

*   VGPU support via mediated device framework (MDEV)
*   AMD ROCm virtualization (research)
*   Software rendering fallback (LLVMpipe/Zink) for headless VMs

***

## Supported Guest OSes

| Guest OS | Status | Notes |
|----------|--------|-------|
| SigmaOS | ✅ Full support | Nested SigmaOS VMs |
| Linux (any distro) | ✅ Full support | virtio-\*, 9p, KVM accel |
| Windows 10/11 | ✅ Full support | Hyper-V enlightenments |
| FreeBSD | ✅ Full support | bhyve compat mode |
| macOS (ARM) | 🔄 Research | Legal/technical challenges |
| Android (AOSP) | 🔄 Planned | For app compat layer |

***

## VM Configuration Format

```toml
# /etc/sigma/vms/myvm.toml
[vm]
name = "dev-machine"
cpus = 4
memory_mb = 8192
arch = "x86_64"

[disk]
image = "/var/sigma/vms/dev.qcow2"
format = "qcow2"
cache = "io_uring"

[network]
type = "virtio"
bridge = "sigma-br0"
zero_trust = true  # Enable ZT network agent for VM

[security]
selinux_label = "vm_t"
sandbox = "capsicum"
iommu = true

[display]
type = "virtio-gpu"
resolution = "1920x1080"
```

***

## VM Management CLI

```bash
# Create a new VM
sigma-vm create --config /etc/sigma/vms/myvm.toml

# Start a VM
sigma-vm start myvm

# Live migrate a VM
sigma-vm migrate myvm --destination host2.sigma.local

# Snapshot a running VM
sigma-vm snapshot myvm --name before-upgrade

# Rollback to snapshot
sigma-vm rollback myvm --snapshot before-upgrade

# Inspect VM internals via eBPF (no pause)
sigma-vm inspect myvm --ebpf syscalls

# Monitor VM resource usage
sigma-vm stats myvm --live
```

***

## Security Model

All VMs operate under the following security guarantees:

1.  **Memory Isolation**: VM memory is backed by `memfd` with KSM disabled by default
2.  **Network Isolation**: Every VM gets its own zero-trust network namespace
3.  **Filesystem Isolation**: VM disk images are locked to the VM process via file capabilities
4.  **Device Emulation Safety**: All QEMU device emulation runs in a separate sandboxed process
5.  **Side-channel Mitigations**: L1TF, MDS, Spectre v1/v2/v4 mitigations enforced
