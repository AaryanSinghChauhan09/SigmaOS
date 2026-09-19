# SigmaOS AI Agent BIOS Supervisor System Management Guidelines

## 1. Overview
SigmaOS incorporates a low-level firmware and bootloader management engine supervised by AI agents (such as `BiosSupervisorAgent`, `BootloaderManager`, `SmbiosAcpiInspector`, and `TpmSecureBootValidator`). These guidelines define BIOS/UEFI stage-1/stage-2 boot loading, Multiboot2 header parsing, Device Tree (DTB) blob parsing, initramfs RAM disk extraction, SMBIOS/ACPI table inspection, UEFI NVRAM variable security, and TPM 2.0 PCR attestation for AI agents in SigmaOS.

## 2. Core BIOS Supervisor Management Principles

### 2.1 Stage-1 / Stage-2 Bootloader Supervision
- **Dual BIOS/UEFI Loading**: AI agents supervise bootloader stage-1 (MBR / UEFI PE32+) and stage-2 (`src/boot/bootloader.rs`).
- **Multiboot2 & DTB Parsing**: Agents validate Multiboot2 information structures (memory maps, ELF section headers, framebuffer info) and Flattened Device Tree (DTB) blobs for ARM64/RISC-V hardware initialization.

### 2.2 Initramfs & Kernel Cmdline Supervision
- **RAM Disk Extraction**: AI agents verify initramfs archive checksums prior to CPIO payload extraction into early memory (`pmm_vmm`).
- **Cmdline Parameter Validation**: Kernel cmdline flags parsed by the BIOS supervisor are sanitized to prevent unauthorized root or init overrides.

### 2.3 SMBIOS & ACPI Table Inspection
- **SMBIOS Structure Parsing**: Agents query SMBIOS tables (Type 0 BIOS, Type 1 System, Type 4 Processor, Type 17 Memory) to construct hardware profile topologies.
- **ACPI Table Management**: Agents inspect ACPI RSDP, RSDT/XSDT, FADT, MADT (Multiple APIC Description Table), and DSDT tables to map LAPIC/IOAPIC interrupt routing and power management states.

### 2.4 UEFI NVRAM & CMOS RTC Security
- **NVRAM Variable Protection**: UEFI NVRAM variables (`BootOrder`, `BootNext`, `PK`, `KEK`, `db`) are protected against unauthorized modification using authenticated variable writes.
- **CMOS RTC Clock Sync**: Agents synchronize in-kernel system time with CMOS Real-Time Clock (RTC) hardware registers during early boot.

### 2.5 TPM 2.0 Measured Boot & PQC Attestation
- **TPM 2.0 PCR Measurement**: BIOS supervisor agents log stage-1, stage-2, and kernel image hashes into TPM 2.0 Platform Configuration Registers (PCRs 0-7).
- **Post-Quantum Attestation**: Boot artifacts are signed and validated using Dilithium-5 / Ed25519 signatures (`scripts/sign_release.sh` and `sigma attest`).

---
*Maintained by the SigmaOS Firmware & Boot Steering Committee.*
