# SigmaOS Rufus USB Installation & Media Checksum Subsystem - Master Development Plan

## 1. Executive Summary & Vision

`Sigma-MediaVerify` (`checkisomedia`) is the native, zero-dependency installation media integrity verification and Rufus USB flashing compatibility subsystem for **SigmaOS**. Inspired by Fedora's Anaconda `checkisomedia` (`rd.live.check`), Arch Linux `archiso` `checksum` validation, Ubuntu `casper` live media verification, and OpenBSD `signify`/GPG cryptographic release attestations, `Sigma-MediaVerify` guarantees that bootable USB drives flashed via Rufus, Etcher, or `dd` are 100% bit-exact, uncorrupted, and cryptographically authentic before operating system installation commences.

---

## 2. Inspirations from Linux & BSD Ecosystems

| Ecosystem Origin | Feature & Flashing / Verification Mechanism Absorbed | Target Subsystem / Module |
| :--- | :--- | :--- |
| **Fedora & RHEL Anaconda `checkisomedia`** | ISO application area embedded sector checksums (`isomd5sum`/`checkisomedia`), boot-time `rd.live.check` initramfs sector verification. | `src/installer/` & `src/boot/` |
| **Rufus Flashing Utility Parity** | Hybrid ISO 9660 + El Torito + MBR/GPT partition layout, dual Rufus **DD Mode** and **ISO Mode** flashing compatibility, UEFI:NTFS boot loader shim. | `iso_root/` & `installer/` |
| **Arch Linux `archiso` & Ubuntu `casper`** | Live initramfs block-by-block `airootfs.sha512` validation, progress reporting, corrupted block identification. | `src/distro/arch.rs` & `src/installer/gui_wizard.rs` |
| **FreeBSD & OpenBSD Release Integrity** | `SHA256SUMS` manifest verification, OpenBSD `signify` & GPG detached signature verification (`SHA256SUMS.sig`), PE/COFF UEFI Secure Boot signatures. | `src/security/pki.rs` & `src/security/integrity.rs` |
| **Calamares & Zenith GUI Installer Wizard** | Graphical pre-install media check screen, progress bar, corrupt sector recovery prompt, checksum verification report. | `src/installer/gui_wizard.rs` |

---

## 3. 5-Layer Rufus USB & Checksum Verification Architecture

```
┌────────────────────────────────────────────────────────────────────────┐
│ Layer 5: Cryptographic Release Attestation & UEFI Secure Boot Signatures│
├────────────────────────────────────────────────────────────────────────┤
│ Layer 4: Graphical & CLI Installer Media Verification Wizard           │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 3: Boot-Time `rd.live.check` Initramfs Media Integrity Engine    │
├────────────────────────────────────────────────────────────────────────┤
│ Layer 2: High-Performance Multi-Digest Checksum Engine (SHA256/BLAKE3)│
├────────────────────────────────────────────────────────────────────────┤
│ Layer 1: Hybrid ISO/IMG & Rufus Flashing Compatibility Layout          │
└────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: Hybrid ISO/IMG & Rufus Flashing Compatibility Layout
- **Isohybrid Layout:** ISO 9660 image structured with an MBR and GPT partition table so it can be written directly to a USB drive via `dd` or Rufus DD mode.
- **Rufus ISO Mode Compatibility:** El Torito FAT12/FAT16 EFI system partition containing `EFI/BOOT/BOOTX64.EFI` and `EFI/BOOT/BOOTAA64.EFI` for Rufus ISO extraction mode.
- **UEFI:NTFS Shim:** Support for Rufus UEFI:NTFS bootloader shim when flashing large ISO images onto NTFS formatted USB drives.

### Layer 2: High-Performance Multi-Digest Checksum Engine
- **Supported Digests:** SHA-256, SHA-512, BLAKE3, and legacy MD5/CRC32 sector checksums.
- **Embedded ISO Checksum Header:** Preserving checksums directly in the ISO 9660 Primary Volume Descriptor (PVD) application area (`isomd5sum` format).

### Layer 3: Boot-Time `rd.live.check` Initramfs Media Integrity Engine
- **Boot Option Parsing:** Detecting `checkisomedia` or `rd.live.check` on the boot command line.
- **Block-by-Block Sector Verification:** Reading raw CD-ROM/USB sectors (`/dev/sr0` or `/dev/sdX`) in initramfs prior to mounting the squashed rootfs (`sigma.sqfs`).
- **Pass/Fail Halting:** Halting boot and displaying clear diagnostic error messages if media corruption or bad USB flash blocks are detected.

### Layer 4: Graphical & CLI Installer Media Verification Wizard
- **Calamares-Inspired Pre-Check:** Pre-install step in the GUI installer wizard (`src/installer/gui_wizard.rs`) allowing users to verify USB media integrity before partitioning target drives.
- **Progress & Bad Sector Reporting:** Real-time percentage progress bar, MB/s read speed, and bad sector block list.

### Layer 5: Cryptographic Release Attestation & UEFI Secure Boot
- **`SHA256SUMS.sig` Detached Signatures:** Verifying image authenticity against official SigmaOS OpenBSD-signify or GPG master keys.
- **PE/COFF Secure Boot Signing:** EFI binaries (`bootx64.efi`, `grubx64.efi`, `kernel.elf`) signed with Microsoft-compatible or custom MOK UEFI Secure Boot keys.

---

## 4. Implementation Roadmap

| Milestone | Target Phase | Objectives | Status |
| :--- | :--- | :--- | :--- |
| **Milestone 1** | Hybrid ISO/IMG Layout | Build ISO 9660 + El Torito + MBR/GPT hybrid image generator compatible with Rufus DD/ISO modes. | Implemented |
| **Milestone 2** | Checksum Engine | Implement SHA-256, SHA-512, BLAKE3, and embedded ISO volume descriptor checksum reader. | Implemented |
| **Milestone 3** | Boot-Time Check | Implement `checkisomedia` / `rd.live.check` sector verification in early initramfs boot phase. | Implemented |
| **Milestone 4** | Installer Wizard GUI | Integrate media checksum verification wizard and progress bar into Calamares GUI installer screen. | Implemented |
| **Milestone 5** | Cryptographic Signatures| Implement `SHA256SUMS.sig` signify/GPG signature verification and PE/COFF UEFI Secure Boot attestation. | Implemented |

---

## 5. Verification & Testing Strategy

1. **Unit Tests:** Standalone test runners in `src/installer/gui_wizard.rs`, `src/boot/`, and `src/security/integrity.rs`.
2. **Rufus & Etcher Flash Testing:** Verifying ISO images flashed via Rufus (DD mode and ISO mode) boot cleanly and pass `checkisomedia`.
3. **Corrupt Media Injection Testing:** Simulating bad USB sectors and verifying that `rd.live.check` halts boot safely.
4. **Automated Verification:** Continuous validation via `./run_sigma_tests.sh`.
