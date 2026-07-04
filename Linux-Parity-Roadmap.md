# SigmaOS Linux Parity Roadmap

Complete mapping of missing Linux-equivalent components to SigmaOS implementation status.

---

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Implemented (functional) |
| 🔄 | Partial (stub/headers exist) |
| 🆕 | Just implemented (this sprint) |
| ⬜ | Not started |

---

## Tier 1 — Bootability (Critical Path)

### 1.1 Bootloader & Kernel Startup

| Component | Linux Equivalent | Status | File |
|-----------|-----------------|--------|------|
| UEFI Bootloader | GRUB2, systemd-boot | 🔄 | `sigma-boot/sigma_boot.zig` |
| Kernel Entry (x86-64) | `arch/x86_64/boot/head.S` | 🆕 | `arch/x86_64/head64.asm` |
| GDT + TSS | `arch/x86_64/kernel/cpu.c` | 🆕 | `arch/x86_64/gdt.asm` |
| IDT (256 entries) | `arch/x86_64/kernel/traps.c` | 🆕 | `arch/x86_64/idt.asm` |
| Multiboot2 | GRUB multiboot spec | 🔄 | `arch/boot/multiboot_header.asm` |
| Early serial log | `earlyprintk=serial` | ✅ | `kernel/core/sigma_irq.rs` |
| UEFI GOP framebuffer | `drivers/video/fbmem.c` | 🔄 | `sigma-boot/sigma_boot.zig` |

### 1.2 Scheduler

| Component | Linux Equivalent | Status | File |
|-----------|-----------------|--------|------|
| Round-robin (base) | `SCHED_OTHER` | 🆕 | `kernel/core/sigma_sched.rs` |
| MLFQ (4 levels) | Enhanced CFS | 🆕 | `kernel/core/sigma_sched.rs` |
| CFS (vruntime) | `kernel/sched/fair.c` | 🆕 | `kernel/core/sigma_sched.rs` |
| EDF (hard RT) | `SCHED_DEADLINE` | 🆕 | `kernel/core/sigma_sched.rs` |
| Context switch | `__switch_to()` | ⬜ | `arch/x86_64/switch.asm` |
| Process fork | `do_fork()` | ⬜ | `kernel/core/process_manager.rs` |
| Process table | `struct task_struct` | ⬜ | `kernel/core/process_manager.rs` |
| Timer interrupt | `timer_interrupt()` | 🆕 | `kernel/core/sigma_irq.rs` (PIT 1kHz) |

### 1.3 Memory Manager

| Component | Linux Equivalent | Status | File |
|-----------|-----------------|--------|------|
| Buddy allocator | `mm/page_alloc.c` | 🆕 | `kernel/core/sigma_mm.rs` |
| Slab allocator | `mm/slub.c` | 🆕 | `kernel/core/sigma_mm.rs` |
| 4-level paging | `arch/x86_64/mm/` | 🔄 | `arch/x86_64/paging.zig` |
| ASLR | `mm/mmap.c` entropy | 🆕 | `kernel/core/sigma_mm.rs` (42-bit) |
| W^X enforcement | `mm/pageattr.c` | 🆕 | `kernel/core/sigma_mm.rs` |
| VMA management | `mm/mmap.c` | 🆕 | `kernel/core/sigma_mm.rs` |
| Page fault handler | `mm/fault.c` | 🔄 | `kernel/core/sigma_mm.rs` |

### 1.4 Interrupt Controller

| Component | Linux Equivalent | Status | File |
|-----------|-----------------|--------|------|
| 8259 PIC init/remap | `arch/x86_64/kernel/i8259.c` | 🆕 | `kernel/core/sigma_irq.rs` |
| IDT dispatch | `arch/x86_64/entry_64.S` | 🆕 | `arch/x86_64/idt.asm` |
| PIT timer (1kHz) | `drivers/clocksource/i8253.c` | 🆕 | `kernel/core/sigma_irq.rs` |
| IRQ registration | `request_irq()` | 🆕 | `kernel/core/sigma_irq.rs` |
| Exception handler | `traps.c` | 🆕 | `kernel/core/sigma_irq.rs` |
| APIC | `arch/x86_64/apic/apic.c` | ⬜ | Phase B |
| GIC (ARM64) | `arch/arm64/kernel/irq.c` | ⬜ | Phase C |

### 1.5 Syscall Dispatch

| Component | Linux Equivalent | Status | File |
|-----------|-----------------|--------|------|
| Syscall gate (int 0x80) | `arch/x86_64/entry_64.S` | 🆕 | `arch/x86_64/idt.asm` |
| SYSCALL instruction | `MSR_LSTAR` setup | ⬜ | Phase B |
| 50+ syscall handlers | `kernel/sys.c` | 🔄 | `kernel/core/syscall_dispatch.rs` |
| read/write/open/close | core I/O | ⬜ → VFS | `kernel/core/syscall_dispatch.rs` |
| fork/exec/exit | process control | ⬜ | Phase B |
| socket/connect | networking | ⬜ | Phase C |
| sigma_pledge | SigmaOS-specific | ✅ | `kernel/security/sigma_pledge.rs` |

---

## Tier 2 — Filesystem & I/O

| Component | Linux Equivalent | Status | File |
|-----------|-----------------|--------|------|
| VFS core | `fs/inode.c` | 🆕 | `kernel/vfs/sigma_vfs.rs` |
| Tmpfs | `fs/tmpfs/` | 🆕 | `kernel/vfs/sigma_tmpfs.rs` |
| Ext4 | `fs/ext4/` | 🔄 | `kernel/fs/ext4/` |
| FAT32 | `fs/fat/` | ✅ | EFI partition |
| Procfs | `fs/proc/` | 🆕 | `kernel/linux_compat/proc_shim.rs` |
| Sysfs | `fs/sysfs/` | ⬜ | Phase C |
| Console device | `drivers/char/tty.c` | ⬜ | Phase B |
| dm-verity | `drivers/md/dm-verity.c` | 🔄 | `sigmad/updater/main.rs` (hash) |
| dm-crypt | `drivers/md/dm-crypt.c` | 🔄 | Key derivation stub |

---

## Tier 3 — Networking

| Component | Linux Equivalent | Status | File |
|-----------|-----------------|--------|------|
| IPv4/IPv6 | `net/ipv4/`, `net/ipv6/` | 🔄 | `kernel/net/` |
| TCP state machine | `net/ipv4/tcp.c` | ⬜ | Phase B |
| UDP | `net/ipv4/udp.c` | ⬜ | Phase B |
| ICMP | `net/ipv4/icmp.c` | ⬜ | Phase B |
| ARP | `net/ipv4/arp.c` | ⬜ | Phase B |
| DNS | `net/dns_resolver.c` | ✅ | `net/sigma_dns.rs` |
| DHCP | `net/ipv4/dhcp.c` | ✅ | `net/sigma_dhcp.rs` |
| TLS 1.3 | `net/tls/` | ✅ design | `net/sigma_tls.rs` |
| e1000 NIC | `drivers/net/ethernet/intel/` | ✅ | `kernel/linux_compat/e1000_main.rs` |
| VirtIO-net | `drivers/net/virtio_net.c` | 🔄 | `drivers/net/sigma_virtio_net.rs` |
| Linux compat shim | n/a | ✅ | `drivers/linux_distros/compat.rs` |

---

## Tier 4 — Storage

| Component | Linux Equivalent | Status | File |
|-----------|-----------------|--------|------|
| NVMe | `drivers/nvme/host/` | ✅ | `drivers/sovereignnvme.rs` |
| AHCI/SATA | `drivers/ata/libahci.c` | 🔄 | `drivers/storage/sigma_ahci.rs` |
| VirtIO-blk | `drivers/block/virtio_blk.c` | ✅ | `sdk/driver/examples/virtio_blk.rs` |
| USB mass storage | `drivers/usb/storage/` | 🔄 | `drivers/usb/` |

---

## Tier 5 — Desktop & GPU

| Component | Linux Equivalent | Status | File |
|-----------|-----------------|--------|------|
| Framebuffer (VESA) | `drivers/video/vesa.c` | 🔄 | `drivers/display/sigma_vesa.zig` |
| VirtIO-GPU | `drivers/gpu/drm/virtio/` | 🔄 | `drivers/gpu/sigma_virtio_gpu.zig` |
| KMS/DRM core | `drivers/gpu/drm/drm_*.c` | 🔄 | `drivers/graphics/kms.zig` |
| Intel i915 | `drivers/gpu/drm/i915/` | ⬜ | Phase C |
| AMD amdgpu | `drivers/gpu/drm/amd/` | ⬜ | Phase C |
| Wayland compositor | `weston`, `gnome-shell` | 🔄 | `desktop/compositor/` |
| Input driver | `drivers/input/evdev.c` | 🔄 | `drivers/input/sigma_hid.zig` |
| HD Audio | `sound/pci/hda/` | 🔄 | `drivers/audio/sigma_hda.rs` |

---

## Tier 6 — Package & Ecosystem

| Component | Linux Equivalent | Status | File |
|-----------|-----------------|--------|------|
| Package format | `.deb`, `.rpm` | 🔄 | `docs/SIGPKG_SPEC.md` |
| Package manager | `apt`, `dnf`, `pacman` | 🔄 | `sigma-pkg/` |
| A/B updates | rpm-ostree, Talos | 🆕 | `sigmad/updater/main.rs` |
| Build system | `make`, `cmake` | ✅ | `Makefile`, `CMakeLists.txt` |
| Rust toolchain | `rustc` | ✅ | `rust-toolchain.toml` |
| Shell | `bash`, `sh` | 🔄 | `sigma-sh/` |
| Core utilities | GNU coreutils | ⬜ | Phase D |
| sigma-ddk (DDK) | WDM, Linux module API | ✅ | `drivers/ddk/sigma_ddk.rs` |

---

## Implementation Progress

```
Tier 1 Bootability:   ████████░░  ~75% (GDT+IDT+IRQ+Sched+MM done; fork/exec pending)
Tier 2 Filesystem:    ██████░░░░  ~55% (VFS+Tmpfs done; ext4 write + console pending)
Tier 3 Networking:    █████░░░░░  ~45% (DNS+DHCP+TLS+e1000 done; TCP state machine pending)
Tier 4 Storage:       ███████░░░  ~65% (NVMe done; AHCI partial)
Tier 5 Desktop:       ███░░░░░░░  ~25% (KMS framework; GPU drivers pending)
Tier 6 Ecosystem:     █████░░░░░  ~45% (DDK+sigpkg+updater done; coreutils pending)
```

---

## Comparison to Linux Distros

| Metric | Alpine 3.19 | Debian 12 | SigmaOS v15 | Target |
|--------|------------|-----------|-------------|--------|
| Boot time | 1s | 15s | N/A (not bootable yet) | <5s |
| ISO size | 5 MB | 350 MB | N/A | <50 MB standalone |
| Syscalls | 300+ | 300+ | 50 (stubs) | 100+ by v15.1 |
| Packages | 6000+ | 60000+ | 0 | 100 by v15.1 |
| PQC crypto | ❌ | ❌ | ✅ Kyber+Dilithium | ✅ |
| Stable ABI | ❌ | ❌ | ✅ kabi v1.0 | ✅ |

---

*See also: [12-Week-Milestone-Plan](12-Week-Milestone-Plan) · [Linux Compat RFC](../docs/LINUX_COMPAT_RFC.md) · [Architecture](../Architecture.md)*
