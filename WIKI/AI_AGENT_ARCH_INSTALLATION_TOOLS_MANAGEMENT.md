# AI Agent Directive: Arch Linux Installation Tools Management

## Overview

The `ArchGenfstabGenerator` and `ArchPacstrapInstaller` (`src/distro/arch.rs`) implement Arch Linux `genfstab` and `pacstrap` installation script parity for SigmaOS.

## Key Architectural Structures

1. **`ArchGenfstabGenerator`**:
   - Parses mount entries (`ArchGenfstabMountEntry`) and generates standard static filesystem table (`/etc/fstab`) entries.
   - Supports formatting modes:
     - `ArchGenfstabFormatMode::Uuid` (`-U` flag): Formats block device specifiers using UUIDs (`UUID=...`).
     - `ArchGenfstabFormatMode::Label` (`-L` flag): Formats block device specifiers using filesystem labels (`LABEL=...`).
     - `ArchGenfstabFormatMode::DevicePath` (`-p` flag): Formats block device specifiers using traditional `/dev/` paths.

2. **`ArchPacstrapInstaller`**:
   - Manages base system bootstrapping (`base`, `linux`, `linux-firmware`) into specified target chroot mount points (e.g., `/mnt`).

## Directives for AI Agents

- **Zero-Dependency Rule**: Maintain native Rust string formatting without invoking external shell binaries.
- **Verification**: Run standalone unit tests using:
  ```bash
  rustc --test src/distro/arch.rs --edition=2021 -o build/arch_test && ./build/arch_test
  ```
