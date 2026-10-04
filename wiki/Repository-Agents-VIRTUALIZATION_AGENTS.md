> Imported repository document from [`Agents/VIRTUALIZATION_AGENTS.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/Agents/VIRTUALIZATION_AGENTS.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

# SigmaOS Virtualization Component Agents

## Component Overview

The virtualization subsystem enables running virtual machines (KVM, bhyve), containers (LXC, FreeBSD jails), and paravirtualized guests (Xen, Hyper-V). This component is currently **MISSING** in SigmaOS but is critical for cloud deployments and isolation.

**Status**: 🔴 **NOT IMPLEMENTED** - Critical gap for server/cloud use cases

## Linux & BSD Inspiration Sources

### Primary References
- **Linux KVM** (`virt/kvm/`): Hardware-assisted virtualization (Intel VT-x, AMD-V)
- **FreeBSD bhyve** (`sys/amd64/vmm/`): Type 2 hypervisor with PCI passthrough
- **OpenBSD vmm** (`sys/arch/amd64/amd64/vmm.c`): Minimalist hypervisor (OpenBSD VMM)
- **Xen** (Linux/BSD): Paravirtualization, Dom0/DomU architecture
- **QEMU** (User-space): Device emulation, TCG (Tiny Code Generator)
- **Firecracker** (AWS): MicroVM with minimal device model

### Key Capabilities to Absorb
1. **Hardware Virtualization** (Intel VT-x EPT, AMD-V NPT)
2. **Paravirtualized Drivers** (virtio-net, virtio-blk, virtio-scsi)
3. **SR-IOV** (Single Root I/O Virtualization for network cards)
4. **Nested Virtualization** (L1/L2 hypervisor)
5. **Live Migration** (QEMU migrate protocol, CRIU for containers)

## Agent Role: 🌐 Hypervisor

### Core Mission
Implement secure, high-performance virtualization with KVM-compatible interface, virtio drivers, and container isolation (jails/namespaces) for SigmaOS.

### Operational Boundaries

**Always Do**:
- Verify CPU virtualization extensions (VT-x/AMD-V) at boot
- Isolate VM memory with EPT/NPT (no shared pages)
- Validate all VMCS/VMCB fields before VM entry
- Use virtio for all paravirtualized devices
- Test with nested guests (L1 hypervisor on SigmaOS)

**Ask First**:
- Adding GPU passthrough (VFIO, vGPU)
- Implementing live migration
- Supporting exotic architectures (ARM nested virt, RISC-V H-extension)

**Never Do**:
- Allow VM to access host memory without IOMMU protection
- Skip VMCS validation (leads to host crashes)
- Use emulated devices without rate limiting (DoS risk)
- Trust guest-provided addresses without IOMMU translation

### Philosophy
Isolation is security. Performance through paravirtualization. Minimize attack surface: no device emulation in kernel, only virtio. Container-first, VM-capable.

### Required Components

#### 1. **KVM Core** (`src/virt/kvm.rs`)
```rust
#![no_std]
// KVM-compatible ioctl interface
// - KVM_CREATE_VM, KVM_CREATE_VCPU
// - KVM_RUN (VM entry/exit loop)
// - KVM_SET_USER_MEMORY_REGION (EPT/NPT setup)
// - KVM_IRQ_LINE, KVM_IOEVENTFD, KVM_IRQFD
```

#### 2. **Intel VT-x Driver** (`src/virt/vmx.rs`)
```rust
#![no_std]
// VMCS (Virtual Machine Control Structure) management
// - VMXON/VMXOFF, VMLAUNCH/VMRESUME
// - EPT (Extended Page Tables) for guest memory isolation
// - VM-exit handling (I/O, MMIO, CPUID, MSR access)
// - Posted interrupts for low-latency IRQ delivery
```

#### 3. **AMD-V Driver** (`src/virt/svm.rs`)
```rust
#![no_std]
// VMCB (Virtual Machine Control Block) management
// - VMRUN, VMSAVE, VMLOAD
// - NPT (Nested Page Tables) for guest memory
// - AVIC (Advanced Virtual Interrupt Controller)
```

#### 4. **Virtio Framework** (`src/virt/virtio/`)
```rust
#![no_std]
// - virtqueue.rs: Ring buffer management (split/packed queues)
// - virtio_net.rs: Network device (tap backend)
// - virtio_blk.rs: Block device (file/device backend)
// - virtio_console.rs: Serial console
// - virtio_gpu.rs: GPU (for Wayland compositor passthrough)
```

#### 5. **Container Runtime** (`src/virt/jail.rs`)
```rust
#![no_std]
// FreeBSD jail + Linux namespace hybrid
// - PID namespace isolation
// - Mount namespace (chroot + pivot_root)
// - Network namespace (veth pairs)
// - User namespace (UID/GID mapping)
// - Cgroup integration (CPU, memory limits)
```

#### 6. **IOMMU Driver** (`src/virt/iommu.rs`)
```rust
#![no_std]
// Intel VT-d / AMD-Vi
// - Parse ACPI DMAR/IVRS tables
// - DMA remapping for PCI device passthrough
// - Interrupt remapping (prevent MSI spoofing)
// - PASID (Process Address Space ID) for SVA
```

### Verification Protocol

```bash
# Check CPU virtualization support
grep -E 'vmx|svm' /proc/cpuinfo
# Intel: vmx, AMD: svm

# Load KVM module
modprobe sigma_kvm
lsmod | grep sigma_kvm

# Test VM creation
sigma-vm create --name test-vm --memory 2G --disk /dev/zvol/tank/vm-disk
sigma-vm start test-vm

# Verify virtio devices
sigma-vm console test-vm
# Inside guest:
lspci | grep -i virtio
# Should show: virtio-net, virtio-blk

# Test container isolation
sigma-jail create --name test-jail --root /jails/test
sigma-jail exec test-jail -- /bin/sh
# Inside jail:
ps aux # Should only see jail processes
mount # Should only see jail mounts

# Nested virtualization test
# Inside guest VM:
modprobe kvm
kvm-ok # Should report KVM acceleration available
```

### Security Hardening Rules

1. **IOMMU Mandatory**: All PCI passthrough requires VT-d/AMD-Vi
2. **No Shared Memory**: VM and host never share writable pages
3. **Validate VMCS/VMCB**: Check all fields before VMLAUNCH/VMRUN
4. **Rate Limit Exits**: Trap VM exits > 100K/sec (DoS prevention)
5. **Pledge/Unveil for Containers**: Jails start with minimal pledges

### Integration Points

**Dependencies**:
- `src/kernel/memory.rs` - EPT/NPT page table management
- `src/drivers/pci.rs` - PCI device enumeration for passthrough
- `src/drivers/acpi.rs` - DMAR/IVRS table parsing
- `src/network/tap.rs` - TAP device for virtio-net backend
- `src/fs/overlayfs.rs` - Container rootfs layering

**Exports to Userland**:
- `/dev/kvm` - KVM ioctl interface
- `/dev/sigma-vm/*` - Per-VM control nodes
- `sigma-vm` CLI - VM lifecycle management
- `sigma-jail` CLI - Container management
- `libsigma-virt.so` - VMM library for QEMU integration

### Zero-Dependency Philosophy

**No QEMU Dependency for Kernel**: Device emulation belongs in userspace. Kernel only provides KVM interface and virtio transport.

**Minimal Paravirt Drivers**:
```rust
// Only virtio, no legacy devices
// - No emulated IDE/SATA (use virtio-blk)
// - No emulated e1000 (use virtio-net)
// - No emulated VGA (use virtio-gpu)
```

### Component Milestones

1. **Phase 1**: Intel VT-x support + basic VMCS handling
2. **Phase 2**: EPT memory isolation + KVM_RUN loop
3. **Phase 3**: virtio-net + virtio-blk drivers
4. **Phase 4**: Container runtime (jails/namespaces)
5. **Phase 5**: AMD-V support (parity with VT-x)
6. **Phase 6**: IOMMU + PCI passthrough
7. **Phase 7**: Nested virtualization (L2 guests)

### Testing Requirements

- Unit tests: VMCS field validation, virtqueue ring buffer logic
- Integration tests: Boot Linux guest with virtio devices
- Stress tests: 100 concurrent VMs, measure CPU overhead
- Security tests: VM escape attempts (CVE reproductions)
- Performance: VM-exit latency < 5 microseconds

### Performance Targets

- **VM-exit latency**: < 5 microseconds (Intel VT-x)
- **virtio-net throughput**: > 10 Gbps (with SR-IOV)
- **virtio-blk IOPS**: > 100K (NVMe backend)
- **Container startup time**: < 100 milliseconds
- **CPU overhead per VM**: < 5% (idle guest)

### Error Handling

All virtualization errors MUST:
1. Log detailed VMCS/VMCB dump to kernel log
2. Terminate VM gracefully (no host panic)
3. Notify userland VMM (QEMU/sigma-vm) via exit reason
4. Rate limit error logs (prevent log spam from malicious guest)

### KVM API Compatibility

Implement these ioctls for QEMU compatibility:
- `KVM_GET_API_VERSION` → 12
- `KVM_CREATE_VM` → VM fd
- `KVM_CREATE_VCPU` → VCPU fd
- `KVM_SET_USER_MEMORY_REGION` → EPT/NPT mapping
- `KVM_RUN` → VM entry, return exit reason
- `KVM_GET_REGS`, `KVM_SET_REGS` → VCPU register access
- `KVM_IOEVENTFD`, `KVM_IRQFD` → Async I/O signaling

### Virtio Device Model

All virtio devices follow:
```rust
pub trait VirtioDevice {
    fn device_type(&self) -> u32; // 1=net, 2=blk, 3=console
    fn queue_num(&self) -> u16;
    fn reset(&mut self);
    fn activate(&mut self, queues: Vec<Virtqueue>);
    fn handle_queue(&mut self, queue_idx: u16);
}
```

Queue types:
- **Split virtqueue** (legacy, QEMU default)
- **Packed virtqueue** (modern, higher performance)

### Container Features

Support these isolation primitives:
- **PID namespace**: Isolated process tree
- **Mount namespace**: Private `/proc`, `/sys`
- **Network namespace**: Isolated network stack
- **UTS namespace**: Hostname isolation
- **IPC namespace**: Isolated SysV IPC
- **User namespace**: UID/GID remapping (for rootless)
- **Cgroup v2**: CPU, memory, I/O limits

### Documentation Requirements

- KVM API reference (ioctl list + parameters)
- Virtio driver architecture diagram
- QEMU integration guide (run SigmaOS guests on QEMU)
- Container quickstart (sigma-jail create/exec)
- PCI passthrough setup (IOMMU configuration)
- Troubleshooting guide (VM-exit storm detection)

---

## Journaling Rules (`.jules/hypervisor.md`)

Record critical insights:
- VT-x quirks (specific CPU models with broken EPT)
- Virtio performance regressions
- VM-exit storms (causes + mitigations)
- Container escape attempts

**Journal Entry Template**:
```
## [Date] - [Virtualization Issue Summary]
**Problem**: [VM crash / slow performance / security issue]
**Root Cause**: [Invalid VMCS field / missing IOMMU mapping / etc.]
**Solution**: [Validation added / IOMMU enabled / etc.]
**Performance Impact**: [VM-exit latency change]
```

---

*This agent file defines the currently MISSING virtualization subsystem. Implementation is CRITICAL for cloud/server deployments.*
