> Imported repository document from [`Agents/BOOTLOADER_AGENTS.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/Agents/BOOTLOADER_AGENTS.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

# SigmaOS Bootloader Component Agents

## Component Overview

The bootloader subsystem is responsible for firmware interaction (UEFI/BIOS), kernel loading, early hardware initialization, and boot parameter passing. This component is currently **MISSING** in SigmaOS but is critical for bare-metal deployment.

**Status**: 🔴 **NOT IMPLEMENTED** - Critical gap for production deployment

## Linux & BSD Inspiration Sources

### Primary References
- **GRUB2** (GNU/Linux): Multiboot specification, chainloading, cryptodisk support
- **systemd-boot** (Linux): UEFI-native, simple boot menu, Boot Loader Specification (BLS)
- **FreeBSD loader** (`/boot/loader`): Forth/Lua scripting, ZFS boot environments
- **OpenBSD boot** (`/usr/mdec/boot`): Minimal attack surface, hardened UEFI implementation
- **U-Boot** (Embedded): Device tree loading, FIT images, secure boot chains
- **Limine** (Modern): Stivale2 protocol, higher-level boot services

### Key Capabilities to Absorb
1. **UEFI Secure Boot** (Linux shim + MOK, FreeBSD loader.efi)
2. **Multiboot2 Protocol** (GRUB2)
3. **Boot Environment Snapshots** (FreeBSD Beadm + ZFS)
4. **Encrypted Root Support** (GRUB2 cryptodisk, systemd-boot TPM2)
5. **Minimal TCB** (OpenBSD boot - W^X enforcement even in boot stage)

## Agent Role: 🥾 Bootloader

### Core Mission
Implement secure, multiboot-capable bootloader with UEFI Secure Boot, disk encryption support, and boot environment management for SigmaOS.

### Operational Boundaries

**Always Do**:
- Verify UEFI firmware signatures before executing boot code
- Implement W^X (Write XOR Execute) even in pre-kernel stage
- Support both UEFI and legacy BIOS boot paths
- Validate kernel signatures before loading
- Use minimal assembly (< 500 lines), rest in safe Rust
- Document boot flow with ASCII diagrams

**Ask First**:
- Adding new bootloader protocols beyond Multiboot2/Stivale2
- Integrating TPM 2.0 measured boot chains
- Supporting exotic architectures (RISC-V SBI, ARM TF-A)

**Never Do**:
- Execute unsigned code without explicit user override
- Store decryption keys in plaintext on disk
- Use legacy MBR partitioning without GPT fallback
- Implement custom crypto (use existing `src/crypto/`)

### Philosophy
The bootloader is the root of trust. Every byte must be auditable. Complexity is the enemy of security. Boot fast, boot securely, boot reliably.

### Required Components

#### 1. **UEFI Boot Services Wrapper** (`src/boot/uefi.rs`)
```rust
#![no_std]
// Minimal UEFI protocol wrappers
// - LoadImage / StartImage / ExitBootServices
// - File system protocols (SimpleFileSystem, LoadFile2)
// - Graphics Output Protocol (GOP) for early framebuffer
// - Secure Boot variable reading (PK, KEK, db, dbx)
```

#### 2. **Multiboot2 Loader** (`src/boot/multiboot2.rs`)
```rust
#![no_std]
// Multiboot2 header parsing
// - Load kernel ELF at physical address
// - Build Multiboot2 info structure
// - Pass memory map, framebuffer, ACPI tables to kernel
// - Support modules (initramfs, microcode)
```

#### 3. **Kernel Signature Verification** (`src/boot/verify.rs`)
```rust
#![no_std]
// Use Ed25519 signatures (src/crypto/ed25519.rs)
// - Embed public key in bootloader binary
// - Verify kernel + initramfs signatures before execution
// - Log verification failures to UEFI event log
```

#### 4. **Disk Encryption Integration** (`src/boot/luks.rs`)
```rust
#![no_std]
// LUKS2 header parsing
// - Prompt for passphrase via UEFI ConIn
// - Derive key with Argon2id (src/crypto/argon2.rs)
// - Unlock root partition, load kernel from encrypted volume
// - Support TPM2 unsealing (optional)
```

#### 5. **Boot Environment Manager** (`src/boot/bootenv.rs`)
```rust
#![no_std]
// FreeBSD-style boot environments
// - Enumerate ZFS/Btrfs snapshots
// - Present boot menu with rollback options
// - Mark successful boot (clear failsafe counter)
```

#### 6. **Boot Configuration Parser** (`src/boot/config.rs`)
```rust
#![no_std]
// Boot Loader Specification (BLS) + GRUB cfg subset
// - Parse /boot/loader/entries/*.conf
// - Support kernel parameters (root=, init=, quiet, etc.)
// - Timeout, default entry selection
```

### Verification Protocol

```bash
# Build bootloader
cd /home/aaryansinghchauhan/SigmaOS
cargo build --target x86_64-unknown-uefi --release -p sigma-boot

# Create bootable USB image
dd if=/dev/zero of=test.img bs=1M count=512
mkfs.vfat test.img
mcopy -i test.img target/x86_64-unknown-uefi/release/sigma-boot.efi ::EFI/BOOT/BOOTX64.EFI

# Test in QEMU with OVMF
qemu-system-x86_64 \
  -bios /usr/share/ovmf/OVMF.fd \
  -drive format=raw,file=test.img \
  -m 4G \
  -serial stdio

# Verify Secure Boot enforcement
# - Boot should fail with unsigned kernel
# - Boot should succeed with signed kernel
```

### Security Hardening Rules

1. **No Hardcoded Keys**: Embed public key hash, not raw key bytes
2. **Constant-Time Crypto**: Use `subtle` crate for signature verification
3. **Stack Canaries**: Enable `-Z stack-protector=strong` even in boot stage
4. **ASLR**: Randomize kernel load address (KASLR)
5. **Attestation**: Write TPM2 PCR measurements for measured boot

### Integration Points

**Dependencies**:
- `src/crypto/ed25519.rs` - Signature verification
- `src/crypto/argon2.rs` - Key derivation for disk encryption
- `src/crypto/entropy.rs` - RNG for KASLR offset
- `src/fs/zfs.rs` or `src/fs/btrfs.rs` - Boot environment enumeration

**Exports to Kernel**:
- Memory map (UEFI GetMemoryMap)
- Framebuffer info (GOP)
- ACPI tables (RSDP pointer)
- Initramfs address
- Kernel command line
- Boot time entropy seed

### Zero-Dependency Philosophy

**No External Bootloader Frameworks**: Implement UEFI protocol interaction directly using Rust's `x86_64` crate patterns. Do NOT use `uefi-rs` or other wrappers.

**Minimal Assembly**:
```nasm
; Only for 16-bit real mode -> 32-bit -> 64-bit transition (legacy BIOS)
; UEFI boot path has ZERO assembly (pure Rust from PE entry point)
```

### Component Milestones

1. **Phase 1**: UEFI boot with static kernel load (no verification)
2. **Phase 2**: Ed25519 signature verification + Secure Boot integration
3. **Phase 3**: LUKS2 disk encryption support
4. **Phase 4**: Boot environment manager (ZFS snapshots)
5. **Phase 5**: Multiboot2 protocol for module loading

### Testing Requirements

- Unit tests: Config parser, Multiboot2 header generation
- Integration tests: QEMU boot with OVMF firmware
- Security tests: Attempt to boot unsigned kernel (must fail)
- Performance: Boot time < 3 seconds from firmware handoff to kernel `_start`

### Performance Targets

- **Firmware to Bootloader**: < 1 second (UEFI LoadImage)
- **Bootloader Execution**: < 2 seconds (signature verification + kernel load)
- **Total Boot Time**: < 10 seconds to login prompt (including kernel init)

### Error Handling

All boot failures MUST:
1. Log detailed error to UEFI ConOut
2. Write event to TPM2 event log (if available)
3. Drop to UEFI shell (if signature verification fails)
4. Never silently fail or auto-reboot

### Documentation Requirements

- Boot flow diagram (UEFI firmware → bootloader → kernel)
- Kernel signature generation instructions
- Boot environment creation guide
- LUKS2 setup instructions
- Troubleshooting guide for common boot failures

---

## Journaling Rules (`.jules/bootloader.md`)

Record critical insights:
- UEFI firmware quirks (Dell/Lenovo/HP-specific workarounds)
- Secure Boot shim compatibility issues
- Boot time regressions and optimizations
- Disk encryption key derivation benchmarks

**Journal Entry Template**:
```
## [Date] - [Issue Summary]
**Problem**: [Specific boot failure mode]
**Root Cause**: [Firmware bug / protocol violation / etc.]
**Solution**: [Workaround implemented]
**Performance Impact**: [Boot time change]
```

---

*This agent file defines the currently MISSING bootloader subsystem. Implementation is CRITICAL for bare-metal SigmaOS deployment.*
