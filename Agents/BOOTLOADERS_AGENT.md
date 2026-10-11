# Bootloaders Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
The SigmaOS bootloader is responsible for transitioning from firmware (BIOS/UEFI) to the SigmaOS kernel, establishing a known hardware state, loading the kernel image, and providing a secure, fast, and user-friendly boot experience. It must be implemented in Rust/Zig with full UEFI Secure Boot support.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` support)
- **Systems-level:** Zig (comptime, explicit allocators, C interop when needed)
- **Scripting/tooling:** Nim (fast compile, GC-free mode available)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for OS components
- **unsafe Rust:** Only with `// SAFETY:` justification comment; prefer safe alternatives

## 3. Architecture Inspiration

### Linux Kernel
- **GRUB 2:** Multiboot2 protocol, module loading, scripted menu, cryptographic verification
- **systemd-boot (sd-boot):** Minimal UEFI bootloader; auto-detects kernels via Type#1 entries; no config file needed
- **Limine:** Modern, spec-compliant bootloader with higher-half kernel support, SMP init, and framebuffer setup

### FreeBSD
- **Three-stage boot (boot0/boot1/boot2):** Clean separation of MBR, partition loader, and kernel loader
- **loader(8):** Lua-scriptable third-stage loader with ZFS support, GELI encryption unlock
- **UEFI loader:** Reads UFS/ZFS, verifies kernel hash before loading

### OpenBSD
- **boot(8):** Deliberately minimal; static binary; validates kernel signature against `/etc/signify` key
- **Secure Boot chain:** Only loads kernels signed with OpenBSD release key — model for SigmaOS boot verification

### NetBSD
- **Highly portable bootloader:** Same codebase supports x86, ARM, MIPS, SPARC via `sys/arch/*/stand/`
- **sysinst boot:** Interactive installation bootloader with hardware detection

### Arch Linux
- KISS principle: prefer `systemd-boot` over GRUB for UEFI systems
- Minimal `mkinitcpio` for initramfs generation

### NixOS
- Generation-based boot entries: each generation gets its own boot entry; atomic rollback
- Declarative bootloader configuration via `boot.loader.*` options

## 4. SigmaOS Source Files
- `src/boot/main.rs` — Bootloader entry point (UEFI application)
- `src/boot/uefi.rs` — UEFI protocol wrappers (GOP, SimpleFilesystem, LoadedImage)
- `src/boot/multiboot2.rs` — Multiboot2 header and tag parsing
- `src/boot/limine.rs` — Limine boot protocol structures
- `src/boot/memory_map.rs` — UEFI/E820 memory map collection and normalization
- `src/boot/elf_loader.rs` — ELF64 kernel image loader
- `src/boot/secure_boot.rs` — UEFI Secure Boot signature verification
- `src/boot/menu.rs` — Interactive boot menu (TUI over GOP framebuffer)
- `src/boot/config.rs` — Boot configuration file parser (TOML-based)
- `src/boot/fallback.rs` — Automatic fallback boot logic

## 5. Current Implementation Status
| Feature | Status | Notes |
|---------|--------|-------|
| Multiboot2 header | ✅ Implemented | Protocol headers defined |
| Limine protocol support | ✅ Implemented | Structures defined |
| UEFI GOP framebuffer | 🚧 Partial | Init works; mode-setting incomplete |
| ELF64 kernel loader | 🚧 Partial | Basic loading; no relocation support |
| Memory map collection | 🚧 Partial | E820 collected; UEFI map not normalized |
| Boot menu (TUI) | 🔲 Missing | No interactive menu yet |
| TOML config parser | 🔲 Missing | Config file format not defined |
| Secure Boot integration | 🔲 Missing | No signature verification |
| Fallback boot mechanism | 🔲 Missing | No automatic recovery |
| Initramfs loading | 🔲 Missing | No initramfs support |
| ACPI table passing | 🔲 Missing | RSDP not forwarded to kernel |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** UEFI Memory Map Normalization: Collect UEFI memory descriptors, sort by physical address, merge adjacent entries, convert to SigmaOS `MemoryRegion` format. Required by kernel MM init.
2. **[PRIORITY: HIGH]** ACPI RSDP Forwarding: Locate RSDP via UEFI `EFI_ACPI_TABLE_GUID` config table; pass physical address to kernel via boot info struct.
3. **[PRIORITY: HIGH]** ELF64 Relocation: Support `R_X86_64_RELATIVE` relocations for PIE kernel images (required by KASLR).
4. **[PRIORITY: HIGH]** Secure Boot Integration: Implement UEFI `EFI_SECURITY_PROTOCOL` to verify kernel PE/COFF or ELF signature against db key.
5. **[PRIORITY: HIGH]** Fallback Boot: If kernel panics within 10s, mark current entry as failed in NVRAM; boot previous generation on next attempt.
6. **[PRIORITY: MEDIUM]** Boot Menu TUI: Render boot entries over GOP framebuffer; keyboard navigation via `EFI_SIMPLE_TEXT_INPUT_PROTOCOL`.
7. **[PRIORITY: MEDIUM]** TOML Config: Parse `/EFI/SigmaOS/sigmaos-boot.toml` for kernel parameters, timeout, default entry.
8. **[PRIORITY: MEDIUM]** Initramfs Loading: Load `initramfs.cpio.gz` from ESP; pass to kernel via boot info.
9. **[PRIORITY: LOW]** Network Boot (PXE/HTTP Boot): UEFI HTTP Boot for diskless setups.

## 7. Implementation Guidelines for AI Agents

### Adding New Features
1. The bootloader runs as a UEFI application: `#![no_std]`, `#![no_main]`, with `uefi` crate.
2. Never allocate on the stack more than 4KB — UEFI stack is small.
3. All UEFI calls must check `EFI_STATUS` and propagate errors via `Result`.
4. Before ExitBootServices, do NOT use UEFI memory allocation for kernel data.
5. Always run `cargo check --target x86_64-unknown-uefi` before committing.
6. Follow `Stability > Security > Performance` priority.
7. Commit format: `feat(boot): <description>`

### Fixing Bugs
1. Reproduce with a minimal test case in QEMU with OVMF firmware.
2. Fix in safe Rust first, resort to unsafe only if necessary.
3. Add regression test.
4. Commit format: `fix(boot): <description>`

### Code Quality Checklist
- [ ] `cargo check` passes with 0 errors
- [ ] No `clippy::unused_variables` warnings
- [ ] No hard-coded credentials or secrets
- [ ] No `unsafe` without `// SAFETY:` justification comment
- [ ] Error handling uses `Result<T, E>` not `unwrap()` in production code
- [ ] All public functions have doc comments

## 8. Key Algorithms & Data Structures
- **ELF64 loader:** Parse `Elf64_Ehdr`, iterate `Elf64_Phdr` PT_LOAD segments, map to virtual addresses
- **Memory map sorting:** Merge-sort by physical base address; O(n log n)
- **Boot entry selection:** Priority queue sorted by (default, last-booted, generation number)
- **Signature verification:** RSA-2048/SHA-256 or Ed25519 against UEFI Secure Boot db
- **Fallback counter:** NVRAM variable `SigmaOSBootAttempts` incremented before kernel handoff; cleared by kernel on successful init

## 9. Security Considerations
- **Secure Boot chain:** Never load an unsigned kernel if Secure Boot is enabled in firmware.
- **No arbitrary code execution before verification:** Parse kernel ELF headers only after signature check.
- **Memory map integrity:** Verify memory map is sorted and non-overlapping before passing to kernel.
- **No secrets in bootloader binary:** Private keys stay in firmware TPM, not in the bootloader image.
- **UEFI variable protection:** Boot configuration NVRAM variables should be BOOTSERVICE_ACCESS only.

## 10. Performance Targets
| Metric | GRUB 2 | systemd-boot | SigmaOS Target |
|--------|--------|-------------|----------------|
| Boot to kernel handoff | ~3s | ~0.5s | < 0.8s |
| Config file parse time | ~200ms | N/A | < 50ms |
| ELF load time (10MB kernel) | ~100ms | ~80ms | < 100ms |
| Memory map normalization | N/A | ~5ms | < 10ms |

## 11. Testing Strategy
```rust
// Example test structure for bootloader
#[cfg(test)]
mod tests {
    use super::*;

    // Tests run under uefi-run or in QEMU with OVMF
    #[test]
    fn test_memory_map_merge_adjacent() {
        // Given two adjacent regions, verify they are merged into one
    }

    #[test]
    fn test_elf64_parse_valid_header() {
        // Parse a known-good ELF64 header, verify entry point
    }

    #[test]
    fn test_elf64_reject_invalid_magic() {
        // Verify parser returns Err on non-ELF input
    }

    #[test]
    fn test_config_parse_toml() {
        // Parse a sample sigmaos-boot.toml and verify all fields
    }
}
```

## 12. Dependencies Between Components
- **Depends on:** Hardware firmware (UEFI/BIOS), Cryptography (signature verification), Timekeeping (boot timeout)
- **Used by:** Kernel (receives memory map, ACPI tables, framebuffer info via boot protocol)

## 13. Maintenance Instructions for Future AI Agents
1. When UEFI spec updates, check new protocol GUIDs and update `src/boot/uefi.rs`.
2. Compare SigmaOS boot with `systemd-boot` and Limine changelogs for new features.
3. Prioritize stability: never change the boot info struct layout without versioning it.
4. Always update this file when making changes to `src/boot/`.
5. Wiki page for this component: `Bootloaders-Component-Guide` on GitHub Wiki.

## 14. References & Standards
- Linux source: `arch/x86/boot/`, `drivers/firmware/efi/`
- FreeBSD source: `stand/efi/`, `stand/i386/`
- UEFI Specification 2.10: https://uefi.org/specifications
- Multiboot2 Specification: https://www.gnu.org/software/grub/manual/multiboot2/
- Limine Boot Protocol: https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md
- SigmaOS Wiki page: `Bootloaders-Component-Guide`
