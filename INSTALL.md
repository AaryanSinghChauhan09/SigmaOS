# SigmaOS Installation Guide

This document describes the process of installing SigmaOS from a live lattice (ISO) to physical or virtual storage.

## 📀 Booting the Live ISO

1. Download the latest `sigmaos.iso`.
2. Flash to a USB drive or attach to a VM.
3. Boot and select **"SigmaOS Live Canvas"**.

## 🚀 Running the Installer

Once in the live environment, open the **OmniShell** (`Alt+T`) and run:

```bash
sudo /installer/install.sh
```

## 🛠️ Manual Partitioning (Advanced)

SigmaOS requires a specific partition layout for optimal shard performance:

| Partition | Size | Type | Format |
| :--- | :--- | :--- | :--- |
| **S-BOOT** | 512MB | EFI System | FAT32 |
| **S-SWAP** | 4GB | Swap | - |
| **S-LATTICE** | Remainder | Sovereign Root | SovFS |

## 🔄 Recovery & Rollback

If the installation fails, boot into **Recovery Mode** from the bootloader to access the `SovereignHealer` recovery shell.

---
*For post-install configuration, see the [User Manual](docs/users/manual.md).*
