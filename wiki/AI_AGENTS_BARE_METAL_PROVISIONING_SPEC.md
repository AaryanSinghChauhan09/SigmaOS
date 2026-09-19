# AI Agents Bare-Metal Provisioning Specification for SigmaOS

## Abstract
This specification defines the bare-metal hardware provisioning framework for AI agents operating within SigmaOS. AI agents (such as Claude Code, Codex, Grok, Gemini, and local LLM models managed by `OmarchyHerdrAiAgentManager`) perform automated bare-metal server discovery, BIOS/UEFI firmware verification, disk layout partitioning, network PXE/TFTP boot orchestration, atomic image staging, and post-installation validation using zero-dependency sovereign abstractions.

---

## 1. Bare-Metal Hardware Discovery & Inspection

```
[ AI Agent Provisioning Task ]
             │
             ▼
[ SovereignDeviceManager / PCI Bus Probe ]
             │
 ┌───────────┼───────────┬───────────┐
 ▼           ▼           ▼           ▼
[CPU ISA]  [RAM/NVMe]  [GPU/NIC]  [TPM2/UEFI]
 (x86/ARM)  (Storage)  (Network)  (Security)
```

1. **Hardware Auto-Probing**:
   - `SovereignDeviceManager` queries PCI/PCIe Express buses, USB controllers, and NVMe drives to inspect target server specifications.
   - Microarchitecture levels (`x86-64-v1` through `v4`, ARM64, RISC-V) and GPU acceleration capabilities (NVIDIA, AMD RDNA, Intel Xe) are identified for ISA-optimized binary deployment.
2. **Firmware & Security Handshake**:
   - UEFI NVRAM variables and Secure Boot keys are validated prior to OS installation.
   - TPM 2.0 PCR registers are recorded to establish measured boot chains.

---

## 2. Disk Layout & Partitioning

1. **Partitioning & Sector Alignment**:
   - GPT and MBR partition tables are initialized with 2048-sector (1MB) SSD/NVMe alignment via `FdiskPartedEngine`.
   - Partition GUIDs are set for EFI System Partition (ESP), Linux Root (`x86_64` / `aarch64`), LVM2, and Swap partitions.
2. **LVM2 & CoW Volume Provisioning**:
   - Physical Volumes (`pvcreate`), Volume Groups (`vgcreate`), and Logical Volumes (`lvcreate`) are created with thin provisioning and CoW snapshot capabilities (`SovereignLvmEngine`).

---

## 3. Network PXE Boot Staging & Automation

1. **PXE & TFTP Stage**:
   - PXE boot menus (`SigmaBootloaderEngine`) serve initramfs cpio images (`SovereignFastInitramfsGenerator`) and kernel binaries over network interfaces.
2. **Kickstart & Cloud-Init Declarative Automation**:
   - Automated server installations parse Kickstart configuration files and cloud-init metadata for user account creation, network interfaces, and SSH key deployment.

---

## 4. Atomic System Generation Deployment

1. **Immutable Image Staging**:
   - System rootfs deployments use OSTree-style atomic commit staging (`SovereignOstreeEngine`).
2. **Nix-Style Generation Registration**:
   - System builds create deterministic generations (`SovereignDeclarativeSystemEngine`) with full Merkle closure tree verification.
3. **Sub-Millisecond Rollback Guarantee**:
   - If post-install health checks or smoke tests fail, the server automatically rolls back to the previous stable system generation.

---

## 5. Post-Installation Verification & Audit Logging

- **Cryptographic Verification**:
  - Installed kernel binaries and drivers require Dilithium-5 post-quantum signatures or GPG trust signatures.
- **Audit Ledger**:
  - Every provisioning event (disk format, network config, package installation) is logged to the append-only journal (`SovereignJournaldBinaryStorageEngine`).

---

## 6. Wiki Synchronization

This document is synchronized across all documentation hubs via `./scripts/sync_wiki.sh`:
- `WIKI/AI_AGENTS_BARE_METAL_PROVISIONING_SPEC.md`
- `wiki/AI_AGENTS_BARE_METAL_PROVISIONING_SPEC.md`
- `wiki_repo/AI_AGENTS_BARE_METAL_PROVISIONING_SPEC.md`

---

*Specification Version: 1.0.0 — SigmaOS Bare-Metal Provisioning Architecture*
