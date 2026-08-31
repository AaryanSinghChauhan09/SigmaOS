# SigmaOS Virtualization

## Architecture

    Guest OS
      ↓
    SigmaOS VMM (Rust QEMU/KVM bridge)
      vCPU Loop | Memory | virtio Devices | QMP
      ↓
    KVM Kernel Module
      ↓
    Hardware: Intel VT-x / AMD-V

## QEMU/KVM Bridge

### vCPU Exit Handling

| Exit Reason | Handler |
|------------|---------|
| KVM\_EXIT\_IO | Port I/O (IN/OUT) |
| KVM\_EXIT\_MMIO | Memory-mapped I/O |
| KVM\_EXIT\_HLT | Guest HLT |
| KVM\_EXIT\_SHUTDOWN | Guest shutdown |
| KVM\_EXIT\_HYPERCALL | Paravirtual call |

### virtio Devices

| Device | Use Case |
|--------|---------|
| virtio-blk | Disk storage |
| virtio-net | Networking |
| virtio-fs | Host↔Guest file sharing |
| virtio-gpu | Display output |
| virtio-vsock | Host↔Guest sockets |
| virtio-rng | Entropy |
| virtio-mem | Memory ballooning |

## Container Runtime

### Namespace Isolation

| Namespace | Isolates |
|-----------|---------|
| PID | Process tree |
| Mount | Filesystem |
| Network | Network stack |
| UTS | Hostname |
| IPC | System V IPC |
| User | UID/GID mappings |
| Cgroup | Resource view |

### Security Layers

    1. User namespace (UID 0 → UID 100000 on host)
    2. Seccomp-BPF (~50 safe syscalls)
    3. Linux capabilities (drop all except needed)
    4. Landlock LSM (filesystem access)
    5. Network namespace
    6. Read-only root (EROFS)
    7. AppArmor profile

## VM Templates

| Template | RAM | vCPUs | Disk |
|----------|-----|-------|------|
| minimal | 256 MB | 1 | 2 GB |
| desktop | 4 GB | 4 | 40 GB |
| server | 8 GB | 8 | 100 GB |
| dev | 16 GB | 8 | 200 GB |

## Live Migration

    Source Host                    Dest Host
    Pre-copy dirty pages ────────→
    Continue pre-copy ───────────→
    Stop VM briefly
    Final dirty pages ───────────→
    CPU state ───────────────────→
                               VM starts on dest
    Redirect network ────────────→

Typical downtime: 10–100ms.
