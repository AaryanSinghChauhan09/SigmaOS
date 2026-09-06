# AGENTS_BIOS_MANAGEMENT.md — AI Agent BIOS & Firmware Management Guidelines for SigmaOS

Welcome, AI Agent! This document defines the standards, architectural models, memory safety invariants, and verification protocols for managing, developing, and extending **BIOS, UEFI, SMBIOS, ACPI, and Firmware Systems** in **SigmaOS**.

---

## 1. SigmaOS Firmware Architecture Overview

SigmaOS supports dual-boot mechanisms across legacy BIOS, UEFI 2.x/3.x, and multi-architecture firmware interfaces (ARM Device Trees and RISC-V OpenSBI).

### Core Boot & Firmware Modules (`src/boot/`)
* **Legacy BIOS MBR Bootloader (`src/boot/bootloader.rs`)**:
  - Stage-1 512-byte MBR boot sector loading (`0x7C00`).
  - Stage-2 protected/long-mode kernel entry transition.
  - Multiboot2 header parsing and kernel cmdline option parser.
* **UEFI Runtime & GOP Subsystem (`src/boot/uefi.rs`, `src/boot/bootloader.rs`)**:
  - GPT (GUID Partition Table) partition table parsing.
  - UEFI GOP (Graphics Output Protocol) linear framebuffer initialization.
  - UEFI Memory Map descriptor parsing and physical frame allocator seeding.
* **SMBIOS & ACPI Engine (`src/boot/firmware.rs`)**:
  - **SMBIOS (System Management BIOS 2.x / 3.x)**: Structure table parsing (`Type 0: BIOS Info`, `Type 1: System Info`, `Type 2: Baseboard Info`, `Type 4: Processor Info`, `Type 17: Memory Device Info`).
  - **ACPI (Advanced Configuration and Power Interface)**: Root System Description Pointer (`RSDP` signature `RSD PTR `), `RSDT`/`XSDT` table probing, `FADT` (Fixed ACPI Description Table), `MADT` (Multiple APIC Description Table), and `DSDT` AML byte-code parsing.
  - **UEFI NVRAM Non-Volatile Variables**: `GetVariable` and `SetVariable` abstraction layer for UEFI non-volatile boot options and secure keys.
  - **CMOS Real-Time Clock (RTC)**: Port `0x70`/`0x71` CMOS RTC register reader for hardware time synchronization.
* **UEFI Secure Boot & TPM 2.0 Attestation (`src/boot/secure_boot.rs`, `src/boot/secure.rs`)**:
  - Key signature validation for Platform Key (`PK`), Key Exchange Key (`KEK`), Allowed Database (`db`), and Forbidden Database (`dbx`).
  - TPM 2.0 PCR (Platform Configuration Register) measurement logging and Dilithium-5 post-quantum boot attestation.

---

## 2. Firmware Development Guidelines for AI Agents

When modifying or adding firmware, BIOS, or ACPI logic:

### 1. Memory Safety & Address Space Rules
* **Identity Mapping & High-Half Offsets**: Early physical addresses (e.g. BIOS EBDA `0x9FC00`, ACPI RSDP `0x000E0000-0x000FFFFF`, or UEFI Framebuffer addresses) must be accessed through validated physical-to-virtual mapping helpers (`pmm_vmm`).
* **Boundary Validation**: Always verify that SMBIOS structures or ACPI table headers do not exceed the declared table length before parsing inner strings or fields.
* **Pointer Arithmetic**: Never cast raw physical pointers without validating alignment (e.g. 8-byte alignment for RSDP / XSDT).

### 2. SMBIOS Checksum Verification
* When parsing SMBIOS 2.x/3.x entry points:
  - Verify 4-byte anchor string (`_SM_` or `_SM3_`).
  - Calculate byte-wise sum modulo 256 over the entry point length; the checksum must equal `0x00`.

### 3. Non-Volatile NVRAM Write Safety
* **Flash Wear Mitigation**: Avoid excessive NVRAM variable writes (`SetVariable`) in loops or frequent ticks.
* **Variable Attributes**: Ensure `NVRAM_VARIABLE_NON_VOLATILE` and `NVRAM_VARIABLE_BOOTSERVICE_ACCESS` attributes are correctly configured to prevent corrupting UEFI boot entries.

### 4. Multi-Architecture Firmware Parity
* **x86_64**: Legacy BIOS MBR/Multiboot2 + UEFI 2.8+ GPT/GOP/ACPI.
* **AArch64**: UEFI + Device Tree Blob (DTB) device tree node parsing (`/chosen`, `/memory`, `/cpus`).
* **RISC-V 64**: OpenSBI (Supervisor Binary Interface) HART initialization + DTB parsing.

---

## 3. Firmware Verification & Testing Protocols

1. **Standalone Firmware Unit Tests**: Run standalone bootloader and firmware tests:
   ```bash
   rustc --test --edition=2021 --cfg 'feature="standalone_test"' src/boot/sigma_boot.rs -o /tmp/test_boot
   /tmp/test_boot
   ```
2. **QEMU Firmware Simulation**:
   - To test legacy BIOS boot:
     ```bash
     qemu-system-x86_64 -drive file=build/sigmaos.iso,format=raw -m 2048
     ```
   - To test UEFI boot:
     ```bash
     qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -drive file=build/sigmaos.iso,format=raw -m 2048
     ```
3. **Core Test Suite**: Run the full test suite runner:
   ```bash
   ./run_sigma_tests.sh
   ```

---

## 4. Pre-Commit Checklist for Firmware Changes

Before submitting BIOS or firmware changes:
- [ ] Confirmed SMBIOS/ACPI table checksums are verified.
- [ ] Confirmed no unaligned raw pointer dereferences occur in early boot routines.
- [ ] Executed standalone firmware tests (`src/boot/sigma_boot.rs`).
- [ ] Executed `./run_sigma_tests.sh` with 100% test pass rate.
- [ ] Requested automated code review using `request_code_review`.
- [ ] Recorded firmware learnings using `initiate_memory_recording`.
