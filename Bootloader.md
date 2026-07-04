# SigmaOS Bootloader (sigma-boot.efi)

SigmaOS uses a Zig-based UEFI bootloader (`sigma-boot/sigma_boot.zig`) that
loads the kernel ELF, gets the memory map, sets up the framebuffer, and
hands off to `sigma_kernel_main()`.

---

## Boot Flow

```
Power on
  │
  ▼
UEFI Firmware (OVMF in QEMU, vendor firmware on hardware)
  │ loads
  ▼
sigma-boot.efi  (sigma-boot/sigma_boot.zig)
  │ 1. Clear screen, print banner
  │ 2. Locate GOP framebuffer
  │ 3. Open SimpleFileSystem on ESP
  │ 4. Load /boot/sigma-kernel.elf into RAM at 0x200000
  │ 5. Get UEFI memory map
  │ 6. ExitBootServices (UEFI is done)
  │ 7. Jump to kernel ELF entry point
  ▼
arch/x86_64/head64.asm  (sigma_kernel_entry)
  │ 1. Load our GDT (kernel CS/DS/TSS)
  │ 2. Set up 4-level page tables (identity map 4 GB, 2MB huge pages)
  │ 3. Enable NXE bit in EFER (no-execute)
  │ 4. Set kernel stack (64 KB)
  │ 5. Call sigma_kernel_main(boot_info*)
  ▼
kernel/core/sovereign_kernel_main.rs  (sigma_kernel_main)
  │ 1. Initialize serial debug
  │ 2. Rollback gate check
  │ 3. Load GDT+IDT
  │ 4. Init slab allocator (from BootInfo memory map)
  │ 5. PIT timer 1000 Hz
  │ 6. PS/2 keyboard
  │ 7. VFS + tmpfs
  │ 8. Process manager, IPC, scheduler
  │ 9. Mark boot successful
  │ 10. Enable interrupts (sti)
  ▼
Event loop (hlt; keyboard handler)
```

---

## BootInfo Structure

The bootloader passes this struct to the kernel:

```rust
pub const BootInfo = extern struct {
    magic:          u64,   // 0x5369676D61_424F4F54 "SigmaBOOT"
    memory_map:     u64,   // Physical address of EFI memory map
    memory_map_sz:  usize,
    desc_sz:        usize,
    rsdp_addr:      u64,   // ACPI RSDP (for ACPI parsing)
    framebuffer:    u64,   // GOP framebuffer physical address
    fb_width:       u32,
    fb_height:      u32,
    fb_stride:      u32,
    kernel_phys:    u64,   // Where kernel was loaded
    kernel_sz:      u64,
    initramfs_phys: u64,   // Optional initramfs
    initramfs_sz:   u64,
};
```

---

## Building

```bash
# Build the UEFI EFI stub
cd sigma-boot
zig build -Dtarget=x86_64-uefi

# Output: zig-out/bin/sigma-boot.efi
```

---

## Testing with QEMU OVMF

```bash
# Install OVMF (UEFI firmware for QEMU)
# Ubuntu: apt install ovmf
# Arch:   pacman -S edk2-ovmf

# Create ESP (EFI System Partition)
mkdir -p esp/EFI/BOOT esp/boot
cp sigma-boot/zig-out/bin/sigma-boot.efi esp/EFI/BOOT/BOOTX64.EFI
cp build/sigma-kernel.elf esp/boot/sigma-kernel.elf

# Run with QEMU OVMF
qemu-system-x86_64 \
  -bios /usr/share/OVMF/OVMF.fd \
  -drive format=raw,file=fat:rw:esp \
  -serial stdio \
  -m 256M \
  -nographic

# Expected output:
# SigmaOS Boot v15.0
# [BOOT] Kernel loaded at 0x200000 (sz: ...)
# [BOOT] ExitBootServices — jumping to kernel
# Σ SigmaOS Zenith Kernel Initializing (Rust)
```

---

## Fallback: Multiboot2 (GRUB)

For legacy BIOS systems, SigmaOS supports Multiboot2 via GRUB:

```
# /boot/grub/grub.cfg
menuentry "SigmaOS v15.0" {
    multiboot2 /boot/sigma-kernel.elf
    module2 /boot/sigma-initramfs.cpio.gz
    boot
}
```

The `arch/boot/multiboot_header.asm` provides the Multiboot2 magic header.

---

## Secure Boot (Phase G)

Planned: sigma-boot.efi will:
1. Verify kernel ELF signature (Dilithium-5)
2. Extend TPM PCR[0..4] with hashes of each boot stage
3. Seal disk encryption key against PCR values
4. Refuse to boot unsigned kernels

---

*Source: `sigma-boot/sigma_boot.zig` · `arch/x86_64/head64.asm` · `arch/x86_64/gdt.asm` · `arch/x86_64/idt.asm`*
