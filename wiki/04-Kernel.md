# Kernel & Linux/BSD Innovations

SigmaOS kernel is a microkernel-inspired design with modular subsystems, enhanced with native OpenBSD, NetBSD, FreeBSD, and Linux innovations.

## Kernel Architecture

### Core Components

- **Task Scheduler**: CPU scheduling with multiple algorithms (CFS, RT, EEVDF, BORE, IDLE)
- **Memory Management**: Buddy allocator, slab allocator, transparent huge pages, MGLRU page aging
- **IPC**: Inter-process communication with zero-copy shared memory, message queues, and Android Binder IPC
- **VFS**: Virtual filesystem layer with pluggable filesystem drivers (Btrfs, ZFS, F2FS, EROFS, OverlayFS)
- **Security**: Capability-based security, OpenBSD pledge/unveil sandboxing, eBPF CO-RE filters
- **Drivers**: Hardware abstraction layer for PCIe, NVMe 2.0 multi-queue, USB 3.2/USB4 xHCI, Wi-Fi 7 MLO

## Implemented Linux & BSD Innovations Engine

SigmaOS natively integrates core capabilities from top BSD and Linux kernels:

- **OpenBSD Pledge & Unveil Capability Hardening (`PledgeUnveilEnforcer`)**: System call restriction and file path scoping.
- **NetBSD Rump Kernel Subsystem Isolates (`RumpKernelIsolateLauncher`)**: Lightweight userland driver/fs process sandboxes.
- **Linux eBPF CO-RE Bytecode Validator (`EbpfCoReValidator`)**: Portable eBPF validation with BPF Type Format (BTF) relocations.
- **FreeBSD VNET Jail Virtual Network Stack (`FreeBsdVnetJailStack`)**: Dedicated virtual network interface stacks per jail container.

## Kernel Modules & Management

### Loading Modules

Load kernel modules dynamically:

```bash
# Load module
sigmod load module-name

# Unload module
sigmod unload module-name

# List loaded modules
sigmod list

# Show module information
sigmod info module-name
```

### Kernel Parameters

View and modify kernel parameters:

```bash
# View all parameters
sysctl -a

# View specific parameter
sysctl kernel.hostname

# Set parameter
sysctl kernel.hostname="new-hostname"

# Persist parameter
echo "kernel.hostname=new-hostname" >> /etc/sysctl.conf
```

## Security Mitigations

### Kernel Pointer Restriction & Mitigation

Restrict kernel pointer exposure:

```bash
# Set kptr_restrict level
sysctl kernel.kptr_restrict=2

# Set dmesg_restrict level
sysctl kernel.dmesg_restrict=1
```

## Next Steps

- [Filesystems](05-Filesystems.md) - Filesystem management
- [Networking](06-Networking.md) - Network configuration
- [Security](07-Security.md) - Security hardening
