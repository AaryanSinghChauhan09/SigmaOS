# sigma-boot — Bootloader Specification

**Status:** Draft · Target: v0.1
**Owner:** arch/boot team
**Canonical source:** `arch/boot/`, `boot/sigma-boot/`

---

## Overview

sigma-boot is a UEFI application (`sigma-boot.efi`) that handles GOP framebuffer initialisation, memory map collection, kernel ELF loading, ACPI table location, and the final jump to `kernel_main()`. It supports measured boot via TPM2 PCR extension and A/B slot selection for atomic OS updates.

## Goals

- Zero external firmware dependencies beyond standard UEFI services
- Kernel loaded in < 1 second on typical NVMe hardware
- Measured boot: extend TPM2 PCR[11] with SHA-256 of kernel ELF before jumping
- A/B slot: read slot variable from UEFI NVRAM; boot slot B if slot A marked bad
- GRUB fallback: if sigma-boot.efi fails, EFI boot manager falls back to GRUB entry

---

## sigma-boot.efi — UEFI Application

Entry point: `EfiMain(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable)`

### Phase 1 — Console + GOP Init

1. Open `gEfiGraphicsOutputProtocolGuid` on all handles
2. Query supported modes; select highest-resolution 32-bpp mode
3. Save framebuffer base address, stride, width, height → pass to kernel in `SigmaBootInfo`
4. Clear screen to dark background; draw sigma-boot splash (raw RGBA bitmap from `.rodata`)

### Phase 2 — Memory Map

1. Call `GetMemoryMap()` to retrieve UEFI memory descriptor array
2. Convert descriptor types to `SigmaMemType` enum (usable / reserved / ACPI / MMIO)
3. Retry with larger buffer if `EFI_BUFFER_TOO_SMALL`

### Phase 3 — Filesystem + Kernel Load

1. Open `EFI\SigmaOS\kernel.elf` from EFI System Partition (FAT32)
2. Verify Dilithium-5 signature against `/EFI/SigmaOS/kernel.elf.sig`
3. Parse ELF64 headers; allocate `EfiLoaderData` pages at each `PT_LOAD` vaddr
4. Copy segments; zero BSS
5. Resolve kernel entry point address (`e_entry`)

### Phase 4 — ACPI Tables

1. Locate RSDP via ACPI 2.0 config table GUID in EFI System Table
2. Parse RSDT/XSDT for MADT (interrupt routing), SRAT (NUMA), DSDT (device tree)
3. Store table physical addresses in `SigmaBootInfo`

### Phase 5 — TPM2 Measured Boot

1. Compute SHA-256 of loaded kernel image bytes
2. `TPM2_PCR_Extend(PCR_INDEX=11, digest=sha256_kernel)`
3. Log extension event to TCG2 event log (EFI_TCG2_PROTOCOL)
4. sigma-trustd validates PCR chain during first OS boot second

### Phase 6 — A/B Slot Selection

1. Read UEFI NVRAM variable `SigmaOSSlot` (namespace GUID `{sigma-boot-guid}`)
2. Values: `"A"` (default), `"B"`, `"A_bad"`, `"B_bad"`
3. If current slot marked `_bad`: switch to other slot; update NVRAM variable
4. Load `kernel.elf` from corresponding slot directory: `/EFI/SigmaOS/slotA/` or `/slotB/`
5. On successful boot: OS writes `"A"` or `"B"` back to clear `_bad` flag

### Phase 7 — Exit Boot Services + Jump

1. Call `ExitBootServices()` with final memory map key
2. Disable interrupts; set up identity-mapped page table (minimal 4-level, 2 MB pages)
3. Jump to `kernel_main(SigmaBootInfo *info)` — never returns

---

## SigmaBootInfo Structure

```c
typedef struct {
    uint64_t     magic;              // 0x5349474D42545F21 ("SIGMABT!")
    uint32_t     version;
    // Framebuffer
    uint64_t     fb_base;
    uint32_t     fb_width, fb_height, fb_stride;
    // Memory map
    uint64_t     memmap_base;
    uint32_t     memmap_count;
    // ACPI
    uint64_t     rsdp_addr;
    // Kernel
    uint64_t     kernel_entry;
    uint64_t     kernel_phys_base;
    // Slots
    uint8_t      active_slot;        // 0=A, 1=B
    uint8_t      reserved[7];
} SigmaBootInfo;
```

---

## GRUB Fallback

EFI boot order: `1. sigma-boot.efi` → `2. GRUB2 entry (shimx64.efi)`

If sigma-boot.efi returns non-zero EFI status (load failure, sig verify fail), EFI boot manager proceeds to GRUB entry. GRUB loads a minimal recovery kernel with `init=/sigma-recovery`.

---

## Implementation Plan

- [ ] 1. UEFI toolchain: `llvm-mingw` or EDK2 headers; `sigma-boot.efi` CMake target
- [ ] 2. GOP init + splash draw (`boot/gop.c`)
- [ ] 3. Memory map collection + type conversion (`boot/memmap.c`)
- [ ] 4. FAT32 file reader via UEFI SimpleFileSystem protocol (`boot/fs.c`)
- [ ] 5. ELF64 parser + loader (`boot/elf.c`)
- [ ] 6. Dilithium-5 signature verification (`boot/verify.c`)
- [ ] 7. ACPI RSDP/XSDT/MADT parser (`boot/acpi.c`)
- [ ] 8. TPM2 PCR extend via EFI_TCG2_PROTOCOL (`boot/tpm2.c`)
- [ ] 9. A/B slot NVRAM read/write (`boot/slot.c`)
- [ ] 10. ExitBootServices + minimal page table + jump (`boot/jump.c`)
- [ ] 11. QEMU OVMF test: boot sigma-boot.efi → print "Jumping to kernel"
- [ ] 12. Real hardware test: x86_64 laptop with UEFI Secure Boot disabled

---

## Status

| Feature | State |
|---------|-------|
| GOP framebuffer | ⬜ Not started |
| ELF loader | ⬜ Not started |
| Sig verification | ⬜ Not started |
| TPM2 measured boot | ⬜ Not started |
| A/B slot | ⬜ Not started |
| GRUB fallback entry | ⬜ Not started |
