# Boot

SigmaOS implements a fully custom boot chain written in Rust (and Zig for the earliest stage). It supports UEFI Secure Boot, BIOS legacy boot, measured boot via TPM2, early microcode loading, and a sub-2-second path from firmware handoff to `sigma-init` PID 1.

---

## Boot Chain Overview

```
 ┌──────────────────────────────────────────────────────┐
 │  1. UEFI Firmware  (vendor)                           │
 │     └── Loads EFI bootloader from EFI System Part.   │
 └─────────────────────┬────────────────────────────────┘
                       │
 ┌─────────────────────▼────────────────────────────────┐
 │  2. SigmaEFI Bootloader  (Zig/Rust, UEFI app)        │
 │     ├── TPM2 PCR extend (measured boot)               │
 │     ├── Verify kernel Ed25519 signature               │
 │     ├── Load kernel + initramfs into memory           │
 │     └── Enter long mode, handoff to kernel entry      │
 └─────────────────────┬────────────────────────────────┘
                       │
 ┌─────────────────────▼────────────────────────────────┐
 │  3. SigmaOS Kernel Entry  (src/boot/)                 │
 │     ├── Early console (framebuffer / serial)          │
 │     ├── CPU feature detection (CPUID)                 │
 │     ├── GDT / IDT / TSS setup                         │
 │     ├── Physical memory map parsing (UEFI memmap)     │
 │     ├── Paging: identity map + kernel virtual map      │
 │     ├── Microcode update                              │
 │     └── Jump to Rust kernel_main()                    │
 └─────────────────────┬────────────────────────────────┘
                       │
 ┌─────────────────────▼────────────────────────────────┐
 │  4. Kernel Initialization                             │
 │     ├── ACPI tables parse                             │
 │     ├── SMP bringup (AP trampoline)                   │
 │     ├── IRQ controller init (xAPIC / x2APIC)          │
 │     ├── Memory subsystems init                        │
 │     └── Mount initramfs → exec /sbin/sigma-init        │
 └─────────────────────┬────────────────────────────────┘
                       │
 ┌─────────────────────▼────────────────────────────────┐
 │  5. sigma-init  (PID 1)  — see Init-and-Services.md  │
 └──────────────────────────────────────────────────────┘
```

---

## SigmaEFI Bootloader

Written in Zig for the firmware-facing UEFI layer, transitioning to Rust after ExitBootServices:

### Features
- **Secure Boot**: kernel signed with Ed25519 (not RSA — smaller, faster)
- **Measured Boot**: extends TPM2 PCRs 8–15 with kernel + cmdline hash
- **Multi-boot menu**: detects other OSes via GPT label scan
- **Network boot**: HTTP/TFTP boot via UEFI NetworkStack
- **Recovery mode**: boots to minimal shell without init

### Boot Menu
```
SigmaOS 1.0              (default, 3s timeout)
SigmaOS 1.0 (recovery)
SigmaOS snapshot-2025-10-01
Windows 11
UEFI Firmware Settings
```

---

## BIOS Legacy Boot

For systems without UEFI:
- Stage 1: 512-byte MBR (x86 assembly, Zig-generated)
- Stage 2: GPT-aware sector loader
- Chainloads into GRUB2 → kernel (fallback path)

---

## Kernel Boot Parameters

Key `sigma.cmdline` options:

| Parameter | Default | Description |
|-----------|---------|-------------|
| `root=UUID=...` | required | Root filesystem UUID |
| `rootfstype=sigmafs` | sigmafs | Filesystem type |
| `loglevel=3` | 3 | Kernel log verbosity (0=quiet, 7=debug) |
| `noapic` | off | Disable APIC (fallback) |
| `nomodeset` | off | Disable modesetting (recovery) |
| `sigma.nohz=1` | 1 | Enable tickless kernel |
| `sigma.hugepages=1` | 1 | Enable transparent huge pages |
| `sigma.ai=1` | 1 | Enable AI scheduler/predictor |
| `sigma.tpm=auto` | auto | TPM2 seal mode |

---

## Early Boot Console

SigmaOS provides a framebuffer console before any GPU driver is loaded:
- UEFI GOP (Graphics Output Protocol) framebuffer
- Fixed-size bitmap font (PSF2 format)
- Color output: warnings in yellow, errors in red
- Serial console fallback: 115200 8N1

---

## Secure Boot Chain of Trust

```
Vendor UEFI CA
  └── SigmaOS Signing Key (Ed25519)
        └── SigmaEFI.efi  (bootloader)
              └── sigma-kernel (kernel image)
                    └── initramfs (compressed cpio)
                          └── sigma-init (PID 1)
```

TPM2 PCR banks:
| PCR | Content |
|-----|---------|
| 0 | UEFI firmware |
| 1 | UEFI configuration |
| 8 | SigmaEFI bootloader |
| 9 | Kernel image |
| 10 | Initramfs |
| 11 | Kernel command line |

---

## Boot Time Optimization

| Phase | Time (NVMe) | Optimization |
|-------|------------|--------------|
| UEFI POST | 0.5s | Firmware (not controllable) |
| SigmaEFI | 0.3s | Parallel signature verify |
| Kernel init | 0.5s | Parallel AP bringup |
| initramfs | 0.2s | zstd compressed, tiny (~8MB) |
| sigma-init | 1.5s | Parallel service start |
| **Total** | **< 3s** | |

---

## Comparison vs GRUB / systemd-boot / Limine

| Feature | GRUB2 | systemd-boot | Limine | **SigmaEFI** |
|---------|-------|-------------|--------|--------------|
| Language | C | C | C | **Zig/Rust** |
| Secure Boot | ✅ | ✅ | ✅ | ✅ Ed25519 |
| Measured Boot | ❌ | ✅ | ❌ | ✅ TPM2 |
| Network boot | ✅ | ✅ | ❌ | ✅ |
| Multi-OS | ✅ | ✅ | ✅ | ✅ |
| Memory safe | ❌ | ❌ | ❌ | ✅ |

---

## Source Files

| File | Description |
|------|-------------|
| `src/boot/` | Kernel entry, early init |
| `src/arch/` | Architecture-specific (x86_64) |
| `src/kernel/interrupt_controller.rs` | IDT + APIC setup |
| `src/kernel/acpi_pm.rs` | ACPI table parsing |
| `src/loader/` | ELF/kernel image loader |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/boot/`, `src/arch/`, `src/loader/`
> - Update PCR table if TPM2 policy changes
> - Update boot time table with new hardware measurements
> - Document new boot parameters as they are added to the kernel
