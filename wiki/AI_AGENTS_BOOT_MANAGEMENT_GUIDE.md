# SigmaOS AI Agents Boot Management & Firmware Architecture Guide

Welcome to the **SigmaOS AI Agents Boot Management Guide**. This document details UEFI/BIOS firmware handoff, Multiboot2 specification parsing, Secure Boot verification, boot optimization, and init handoff for autonomous AI agents and bootloader developers in SigmaOS.

---

## 1. Boot Architecture & Execution Lifecycle

SigmaOS boots across `x86_64`, `aarch64`, and `riscv64` platforms through a modular 6-stage boot sequence (`src/boot/`):

### 6-Stage Boot Sequence
1. **Firmware Initialization & Handoff (`src/boot/uefi.rs`, `src/boot/firmware.rs`)**:
   - Native UEFI 2.x protocol binding & GOP (Graphics Output Protocol) framebuffer setup.
   - Legacy BIOS MBR/VBR fallback execution.
2. **Multiboot2 Specification Parsing (`src/boot/multiboot2.rs`)**:
   - Memory map parsing, ACPI RSDP table discovery, and ELF section header extraction.
3. **Secure Boot & Attestation (`src/boot/secure.rs`, `src/boot/verified.rs`)**:
   - Cryptographic verification of kernel image signatures using Dilithium-5 / RSA-4096 keys against TPM 2.0 PCR registers.
4. **Boot Optimization & Fast Path (`src/boot/optimization.rs`)**:
   - Parallel service pre-warm, device tree caching, and early kernel page table allocation aiming for < 180 ms boot times.
5. **Plymouth Splash Screen (`src/boot/plymouth.rs`)**:
   - Hardware-accelerated early boot splash screen, progress bar rendering, and emergency console fallback.
6. **Init Handoff & Microkernel Launch (`src/boot/sigma_boot.rs`, `src/boot/system_init.rs`)**:
   - Transition from bootloader environment to Systemd Betsy Init Supervisor (`PID 1`).

---

## 2. Bootloader APIs & Code Snippets

AI agents managing boot configurations or verifying boot integrity should interface with `SigmaBootEngine` (`src/boot/sigma_boot.rs`):

```rust
use sigmaos::boot::sigma_boot::{SigmaBootEngine, BootMode};

let mut boot_engine = SigmaBootEngine::new();

// Set target boot mode to FastBoot or SecureBoot
boot_engine.set_boot_mode(BootMode::FastBoot);

// Execute boot verification and memory map initialization
assert!(boot_engine.verify_boot_signature().is_ok());
assert!(boot_engine.initialize_system_memory().is_ok());
```

---

## 3. Autonomous AI Agent Boot Diagnostics & Recovery

When AI agents detect bootloader or kernel signature validation failures:

1. **A/B Slot Fallback**: Immediately switch active boot slot from `Slot_A` to secondary backup `Slot_B` (`src/boot/verified.rs`).
2. **Recovery Shell Fallback**: Trigger Plymouth emergency console fallback if kernel initialization halts (`src/boot/plymouth.rs`).
3. **Secure Boot Event Logging**: Record PCR measurement mismatches to the immutable security log.

---

## 4. Checklist for AI Agents Managing Boot Components

- [ ] Verified UEFI GOP framebuffer format (32-bit BGRA/RGBA) before graphical splash screen launch.
- [ ] Checked Multiboot2 memory tag alignment (8-byte boundary).
- [ ] Confirmed Secure Boot signature check verifies against trusted PK/KEK/db certificate store.
- [ ] Tested A/B boot slot recovery under simulated boot failure conditions.
- [ ] Executed `./run_sigma_tests.sh` to confirm boot test suites (`test_boot`) pass cleanly.
