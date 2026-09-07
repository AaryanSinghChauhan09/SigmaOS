# AI Agent Guidelines: Bootloader Management in SigmaOS

## Overview
This document defines guidelines for AI agents working on **Bootloader Management**, UEFI EFI entry generation, Multiboot2 protocol headers, GRUB2 configuration generation, systemd-boot configuration, UEFI Secure Boot signature verification, and ChromeOS-style A/B slot partition updates in SigmaOS.

SigmaOS provides native bootloader engines (`SigmaBootloaderEngine`) supporting both standard UEFI (primary) and legacy BIOS/Multiboot2 boot paths with zero external crate dependencies.

---

## 1. Bootloader Architecture & Subsystems

AI agents interacting with bootloader management in SigmaOS must interface with the following core subsystems:

| Subsystem / Engine | Location | Description |
| :--- | :--- | :--- |
| **`SigmaBootloaderEngine`** | `src/distro/linux_bsd_distro_gaps.rs` | Universal bootloader entry generator supporting `Grub2`, `SystemdBoot`, `EfiDirect`, and `Multiboot2`. |
| **EFI Boot Application (`sigma-boot.efi`)** | `docs/bootloader.md` | UEFI Stage 1 application initializing GOP framebuffer, reading `sigma.toml`, and passing `SigmaBootInfo` to kernel. |
| **`SigmaCryptographicBootChain`** | `src/drivers/sovereign_hardware_roadmap.rs` | Phase 3 cryptographic boot chain enforcing Secure Boot key signature checks on kernel ELF & initramfs payloads. |
| **A/B Slot Manager** | `docs/bootloader.md` | Dual-slot A/B partition boot state evaluator reading slot priority EFI variables (`SlotA`, `SlotB`). |

---

## 2. Boot Partition Layout & Configuration

The primary EFI System Partition (ESP) follows a standardized structure:

```
/EFI/
└── SigmaOS/
    ├── sigma-boot.efi      ← Primary UEFI bootloader binary
    ├── sigma-kernel.elf    ← Signed Sovereign Kernel ELF executable
    ├── sigma.toml          ← Bootloader configuration & entry definitions
    └── initramfs.cpio.gz   ← Compressed initial RAM filesystem
```

### Configuration (`sigma.toml`) Format

```toml
[boot]
kernel = "/EFI/SigmaOS/sigma-kernel.elf"
initramfs = "/EFI/SigmaOS/initramfs.cpio.gz"
timeout = 5

[kernel]
cmdline = "root=/dev/sda1 rw quiet splash"
heap_size = "64M"
scheduler = "eevdf-bore"
kaslr = true
loglevel = 4

[[entries]]
name = "SigmaOS (default)"
kernel = "/EFI/SigmaOS/sigma-kernel.elf"

[[entries]]
name = "SigmaOS (recovery)"
kernel = "/EFI/SigmaOS/sigma-kernel-recovery.elf"
cmdline = "root=/dev/sda1 ro recovery single"
```

---

## 3. Configuration Generation Workflows

When updating or regenerating bootloader entry configurations:

### 1. GRUB2 Configuration Generation
Use `SigmaBootloaderEngine::generate_grub_cfg()` to produce valid `/boot/grub/grub.cfg` snippets:

```rust
let mut engine = SigmaBootloaderEngine::new(BootloaderType::Grub2);
let grub_cfg = engine.generate_grub_cfg();
// Outputs valid `menuentry` blocks for default and fallback kernels.
```

### 2. systemd-boot Entry Generation
Use `SigmaBootloaderEngine::generate_systemd_boot_entries()` to yield loader entry tuples `(filename, content)` targeting `/boot/loader/entries/`:

```rust
let sd_entries = engine.generate_systemd_boot_entries();
// Generates b"sigma.conf" and b"sigma-fallback.conf"
```

---

## 4. Secure Boot & A/B Partition Slot Management

1. **Secure Boot Signature Verification:**
   - Every kernel payload MUST be verified against `sigma-vendor.cer` before execution.
   - Bootloader updates must re-evaluate signatures on modified kernel ELFs and CPIO initramfs images.

2. **A/B Slot Partition Updates:**
   - Partition 2 (`Root A`) and Partition 3 (`Root B`) maintain dual boot slots.
   - When staging system updates, write to the inactive slot, verify payload signature, set the boot priority EFI variable, and mark trial boot counter.
   - Automatically fall back to the alternate slot if trial boot encounters a kernel panic.

---

## 5. AI Agent Self-Assessment Checklist

Before marking any bootloader management task as complete:

- [ ] Are boot parameters verified across `sigma.toml`, `grub.cfg`, and systemd-boot entry files?
- [ ] Is fallback/recovery kernel entry preserved when modifying primary boot entries?
- [ ] Are Secure Boot signatures validated on updated kernel ELF binaries?
- [ ] Has `./run_sigma_tests.sh` been executed and confirmed passing with 0 failures?
