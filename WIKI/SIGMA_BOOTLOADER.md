# SigmaOS Boot Protocol

## Overview

SigmaOS implements a sovereign multi-stage bootloader protocol supporting UEFI, legacy BIOS, ARM TF, and RISC-V OpenSBI. The boot manager provides a GRUB2/systemd-boot-compatible menu interface.

**Location:** `src/boot/sigma_bootloader.rs`

---

## Boot Stages

```
Stage 0: Firmware (UEFI/BIOS)
   ↓
Stage 1: Sigma-Boot (GPT/MBR parser, EFI application)
   ↓
Stage 2: Boot Manager (menu, kernel selection)
   ↓
Stage 3: Kernel handoff (SigmaBootInfo → kernel entry)
```

---

## SigmaBootInfo Structure

Passed from bootloader to kernel at entry point:

```rust
let info = SigmaBootInfo::new_uefi_x86_64("root=/dev/nvme0n1p1 quiet ro");
info.validate().unwrap();

// Access memory map
for region in info.memory_map.available_regions() {
    println!("Available: 0x{:x} - 0x{:x}", region.base, region.end());
}

// Access kernel cmdline
let root = info.cmdline.root_device(); // Some("/dev/nvme0n1p1")
let quiet = info.cmdline.is_quiet();  // true
```

---

## Boot Manager

```rust
let mut mgr = SigmaBootManager::new();
mgr.set_timeout(5);

mgr.add_entry(
    BootEntry::new("SigmaOS", "/boot/sigmaos", "root=/dev/sda1 quiet", BootArch::X86_64)
        .with_initrd("/boot/initrd.img")
        .set_default()
);
mgr.add_entry(
    BootEntry::new("SigmaOS (recovery)", "/boot/sigmaos", "root=/dev/sda1 recovery", BootArch::X86_64)
);

println!("{}", mgr.menu_string());
```

---

## Supported Architectures & Platforms

| Arch | UEFI | Legacy BIOS | ARM TF | OpenSBI |
|------|------|-------------|--------|---------|
| x86_64 | ✅ | ✅ | - | - |
| AArch64 | ✅ | - | ✅ | - |
| RISC-V 64 | ✅ | - | - | ✅ |

---

## Comparison

| Feature | GRUB2 | systemd-boot | limine | SigmaOS |
|---------|-------|-------------|--------|---------|
| Multi-arch | Yes | Limited | Yes | Yes |
| UEFI | Yes | Yes | Yes | Yes |
| Secure Boot | Yes | Yes | Yes | Planned |
| no_std | No | No | Yes | **Yes** |
| Memory map | Yes | Yes | Yes | Yes |
