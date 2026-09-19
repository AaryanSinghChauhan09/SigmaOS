# SigmaOS Real Hardware Maturity & Subsystem Capability Matrix

This machine-readable TOML specification provides a truthful baseline of all SigmaOS subsystems, tracking maturity states (`implemented`, `partial`, `prototype`, `design_proposal`, `not_started`), release gating requirements, and target hardware test verification.

```toml
[matrix_metadata]
specification_version = "1.0.0"
last_updated = "2026-09-18"
architecture_freeze = "x86_64-uefi-qemu"

[feature.kernel_uefi_boot]
status = "implemented"
tested_on = ["qemu-system-x86_64", "ovmf-uefi"]
required_for_release = true
description = "UEFI Boot protocol, Secure Boot validation, command-line parsing, initramfs"

[feature.kernel_smp_topology]
status = "implemented"
tested_on = ["qemu-system-x86_64-smp4"]
required_for_release = true
description = "CPUID topology detection, LAPIC/IO-APIC/HPET IRQ routing, AP startup via IPI"

[feature.kernel_paging_isolation]
status = "implemented"
tested_on = ["qemu-system-x86_64"]
required_for_release = true
description = "4-level PML4 page tables, Ring 0 -> Ring 3 transition, SMEP/SMAP memory boundaries"

[feature.kernel_posix_syscalls]
status = "implemented"
tested_on = ["qemu-system-x86_64"]
required_for_release = true
description = "Core syscall ABI table (exec, exit, wait, read, write, open, close, mmap, poll, PTYs)"

[feature.pci_express_bus]
status = "implemented"
tested_on = ["qemu-virtio-pci"]
required_for_release = true
description = "PCI ECAM configuration space, BAR memory allocation, PCIe Gen 1-6 speed negotiation"

[feature.msi_msix_interrupts]
status = "implemented"
tested_on = ["qemu-virtio-pci"]
required_for_release = true
description = "64-bit MSI/MSI-X MMIO table entry programming, PBA polling, x2APIC vector steering"

[feature.storage_nvme_native]
status = "implemented"
tested_on = ["qemu-nvme"]
required_for_release = true
description = "Native PCIe NVMe Submission/Completion Queue rings, doorbells, 64-byte SQE commands"

[feature.usb_xhci_native]
status = "implemented"
tested_on = ["qemu-xhci"]
required_for_release = true
description = "USB 3.x xHCI TRB ring processing, slot context state, USB HID input packet handling"

[feature.drm_kms_modesetting]
status = "implemented"
tested_on = ["qemu-stdvga", "qemu-virtio-gpu"]
required_for_release = true
description = "Direct Rendering Manager atomic modesetting, CRTC state, GOP/VESA framebuffer fallback"

[feature.sched_deadline_realtime]
status = "implemented"
tested_on = ["qemu-system-x86_64"]
required_for_release = true
description = "SCHED_DEADLINE Earliest-Deadline-First (EDF) scheduler with Constant Bandwidth Server"

[feature.ebpf_x86_64_jit]
status = "implemented"
tested_on = ["qemu-system-x86_64"]
required_for_release = true
description = "Native x86_64 JIT compiler translating eBPF bytecode to machine code"

[feature.confidential_computing]
status = "partial"
tested_on = ["qemu-system-x86_64"]
required_for_release = false
description = "AMD SEV-SNP C-bit and Intel TDX shared bits in page table flags"

[feature.sel4_ipc_verification]
status = "implemented"
tested_on = ["qemu-system-x86_64"]
required_for_release = true
description = "seL4-inspired formal IPC capability rights masks, badge identity, buffer bounds check"

[feature.live_kernel_patching]
status = "implemented"
tested_on = ["qemu-system-x86_64"]
required_for_release = true
description = "Function redirection trampoline engine (5-byte relative JMP) for hot kernel updates"

[feature.efi_panic_nvram]
status = "implemented"
tested_on = ["ovmf-uefi"]
required_for_release = true
description = "Persistent crash logging to EFI NVRAM variable space"

[feature.sysv_runlevel_target_mapper]
status = "implemented"
tested_on = ["qemu-system-x86_64"]
required_for_release = true
description = "SysVInit runlevels 0-6 to systemd target symlink mapper and telinit/runlevel CLI parity"

[feature.emergency_target_gate]
status = "implemented"
tested_on = ["qemu-system-x86_64"]
required_for_release = true
description = "TPM 2.0 / PQC signature authenticated single-user emergency maintenance shell gate"

[feature.package_post_install_sandbox]
status = "implemented"
tested_on = ["qemu-system-x86_64"]
required_for_release = true
description = "Landlock LSM / Capsicum parity post-install script hook network & write path sandbox"

[feature.wayland_zenith_compositor]
status = "partial"
tested_on = ["qemu-virtio-gpu"]
required_for_release = true
description = "Wayland protocol engine, X11 XCB event translation, software rendering pipeline"

[feature.wifi_networking]
status = "partial"
tested_on = ["qemu-virtio-net"]
required_for_release = true
description = "VirtIO network fabric, socket API, OpenBSD PF firewall, eBPF/XDP packet shaper"
```
