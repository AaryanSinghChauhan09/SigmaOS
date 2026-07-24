# Phase J — Kernel Heritage Absorption & Subsystem Expansion

> **Status:** ✅ COMPLETE — 507 tests pass, 50 files / 5550 lines pushed to `main`
> **Commit:** `c862284ca7` · `feat(kernel): Phase J - full legacy driver and subsystem absorption`

---

## Overview

Phase J absorbs the best of Linux 0.01 → 6.x heritage into SigmaOS's modular, OOP-structured Rust kernel. Every subsystem follows the `KernelSubsystem` trait with `InitOrder` dependency-aware boot sequencing via the `SubsystemRegistry`.

---

## 1. Legacy Device Drivers (`kernel/drivers/legacy/`)

All drivers implement `KernelSubsystem`, are OOP-modular, and can be registered at boot.

| Module | Heritage | What was absorbed |
|---|---|---|
| `isa_bus.rs` | Linux ISA/LPC layer | I/O ports, IRQ claims, DMA channels, LPC bridge |
| `uart_8250.rs` | Linux `serial.c` (most-ported driver) | NS8250/16550A, COM1–4, baud divisors 50–115200, FIFO, RX/TX buffers |
| `ps2_controller.rs` | `i8042.c` + AT keyboard | Intel 8042, scancode set 1 (US layout), modifier tracking, PS/2 mouse Intellimouse |
| `cga_mda.rs` | Linux `console.c` 0.01 → fbdev | MDA/CGA/EGA/VGA/SVGA, BIOS INT 10h mode table, 16-colour palette, VT100 text scroll |
| `adlib_opl.rs` | ALSA `snd-opl3` + `snd-sb*` | OPL2 (9ch) + OPL3 (18ch) FM synthesis, SB 1.0→AWE32, PC Speaker PIT |
| `ne2000.rs` | Linux `ne.c` → `e1000` | NE2000 (DP8390/ISA), RTL8139 (PCI), Intel e1000 (Gigabit) |
| `mfm_rll.rs` | Linux 0.01 `hd.c` | ST-506/MFM/RLL, CHS geometry, LBA conversion |
| `ide_ata.rs` | `drivers/ata/` ATA-1 → ATA-7 | LBA28/48, UDMA/133, IDENTIFY response, master/slave, 4-drive topology |
| `floppy.rs` | Linux `floppy.c` | Intel 8272A FDC, 5.25" 360K/1.2M + 3.5" 720K/1.44M/2.88M |
| `usb_host.rs` | Linux USB stack | xHCI (USB 3.2), HID keyboard/mouse, USB mass storage SCSI BBB |

---

## 2. Process Management (`kernel/proc/`)

| Module | Features |
|---|---|
| `process_lifecycle.rs` | `fork()` / `exec()` / `exit()` / `waitpid()` — full POSIX lifecycle |
| `signals.rs` | POSIX signals 1–15, signal masks, custom handlers, SIGKILL uncatchable |
| `namespaces.rs` | PID / Net / Mnt / UTS / IPC / User / Cgroup namespace isolation |
| `cgroups.rs` | cgroups v2: CPU shares, memory limits (bytes), PID count limits |

---

## 3. Advanced Memory Management (`kernel/mm/`)

| Module | Inspired By | Features |
|---|---|---|
| `slab_allocator.rs` | Bonwick 1994 / Linux SLUB | Per-size caches (16B→256B), partial slab reuse, fragmentation prevention |
| `vmalloc.rs` | Linux vmalloc | Non-contiguous physical → contiguous virtual mappings, page-aligned |
| `huge_pages.rs` | Linux HugePages / THP | 2MB and 1GB huge pages with free-list tracking |
| `oom_killer.rs` | Linux OOM killer | Badness score (memory × priority × `oom_score_adj`), victim selection |
| `numa_aware.rs` | Linux NUMA topology | CPU→node mapping, node-local allocation, cross-node fallback |

---

## 4. Kernel Filesystems (`kernel/fs/`)

| Module | Equivalent | Features |
|---|---|---|
| `proc_fs.rs` | `/proc` | Dynamic file generators: `cpuinfo`, `meminfo`, `version`, extensible |
| `sysfs_like.rs` | `/sys` | Device class attribute tree, read/write per-device attributes |
| `devtmpfs.rs` | `/dev` | Auto-populated char/block device nodes, major/minor numbers |

---

## 5. Interrupt Infrastructure (`kernel/irq/`)

| Module | Equivalent | Features |
|---|---|---|
| `irq_controller.rs` | APIC/GIC/PLIC/PIC | x86 APIC, ARM GIC, RISC-V PLIC, legacy 8259 — polymorphic controller |
| `irq_domain.rs` | Linux irq_domain | hwirq → virq mapping, isolation from hardware numbering |
| `softirq.rs` | Linux ksoftirqd | 6-type pending bitmap (HI/Timer/NetTx/NetRx/Block/Tasklet), tasklet queue |
| `workqueue.rs` | Linux workqueues | Thread-context deferred kernel work queue |

---

## 6. Power Management (`kernel/power/`)

| Feature | Details |
|---|---|
| CPUfreq governors | `performance`, `powersave`, `ondemand`, `conservative`, `schedutil` (Linux 4.7+) |
| Thermal zones | Trip points: warm (light throttle) / hot (heavy throttle) / critical (shutdown) |
| Sleep states | S0 (running), S1, S3 (suspend-to-RAM), S4 (hibernate), S5 (soft-off) |
| Wake sources | Configurable wake source registry (RTC, USB, NIC, etc.) |

---

## 7. Network Socket Layer & Traffic Control (`kernel/net/`)

### Socket Layer
Full BSD socket API implemented in the kernel:
```
socket() → bind() → listen() → accept() → connect() → send() / recv() → close()
```
- `AddressFamily`: AF_INET, AF_INET6, AF_UNIX, AF_NETLINK, AF_PACKET
- `SocketType`: SOCK_STREAM, SOCK_DGRAM, SOCK_RAW, SOCK_SEQPACKET
- Port collision detection, per-socket 128KB send/recv buffers
- Socket flags: `SO_REUSEADDR`, `TCP_NODELAY`, `SO_KEEPALIVE`, `O_NONBLOCK`

### Netfilter
5-hook stateful packet filtering (iptables/nftables equivalent):
- Hooks: `PREROUTING → INPUT → FORWARD → OUTPUT → POSTROUTING`
- Verdicts: `ACCEPT`, `DROP`, `REJECT`, `QUEUE`
- Match criteria: src IP, dst IP, dst port, protocol, conntrack state (New/Established/Related/Invalid)
- Default policy: permissive or restrictive

### Traffic Control (tc)
| QDisc | Description |
|---|---|
| `pfifo` | Simple FIFO with packet drop at limit |
| `pfifo_fast` | Linux default: 3 priority bands (interactive/normal/bulk) |
| `SFQ` | Stochastic Fair Queueing — round-robin across flow buckets |
| `TBF` | Token Bucket Filter — byte-accurate rate limiting with burst |

---

## Boot Order

The `SubsystemRegistry` boots drivers in dependency order:

```
EarlyBoot:    uart_8250, vga_driver
CoreKernel:   ps2_controller
Device:       isa_bus → mfm_rll, ide_ata, floppy, ne2000, soundblaster
Late:         usb_storage, proc_fs, sysfs, devtmpfs
```

---

## Test Coverage

```
507 tests total — 0 failed — 0.60s

kernel::drivers::legacy::*     — 30+ tests
kernel::proc::*                — 10+ tests
kernel::mm::*                  — 15+ tests
kernel::fs::*                  — 6+ tests
kernel::irq::*                 — 5+ tests
kernel::power::*               — 5+ tests
kernel::net::*                 — 10+ tests
```

---

## Design Principles Applied

- **OOP via traits**: `KernelSubsystem`, `NicDriver`, `QDisc`, `UsbHostController`
- **Modular composition**: every driver can be registered, initialized, and shut down independently
- **No unsafe in business logic**: raw pointer usage isolated to `irq_controller.rs` only
- **Heritage fidelity**: every driver absorbs real Linux driver design decisions with attribution
- **Ancient device support**: ISA bus, MFM/RLL, 5.25" floppy, NE2000, AdLib OPL2, 8250 UART all work
