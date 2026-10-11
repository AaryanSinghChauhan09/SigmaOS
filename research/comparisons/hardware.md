# Hardware Enablement Comparison Matrix

| Project | Driver Architecture | Device Discovery | Minimum Target Hardware | Validation Evidence |
|---|---|---|---|---|
| **Linux** | In-tree C modules | PCI/ACPI/USB match tables | x86_64 / ARM64 real & virt | Hardware matrix & KernelCI |
| **FreeBSD** | Kernel Loadable Modules | `device_t` probe/attach | Tier 1 x86_64 / ARM64 | TART & CI boot matrix |
| **Redox** | Userspace drivers (Scheme) | PCI scheme bus enumeration | QEMU virtio / real x86_64 | Boot-to-Orbital smoke tests |
| **SigmaOS** | Native `#![no_std]` Rust drivers | PCI MMIO & ACPI MADT | QEMU virtio-blk/net/VGA | QEMU serial boot logs & CI |
