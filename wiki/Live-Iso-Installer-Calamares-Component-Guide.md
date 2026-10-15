# Live ISO Installer & System Deployment Component Guide

## Component Overview
Governs graphical live media boot, automated hardware qualification, disk partitioning (GPT/ESP), rootfs decompression (zstd), user setup, and EFI bootloader installation inspired by Calamares in Omarchy Linux.

## Linux Inspiration
Calamares universal installer framework, Archinstall automated CLI installer, and Ubuntu Subiquity.

## BSD Inspiration
FreeBSD bsdinstall interactive terminal installer, OpenBSD install.sub automated disk partitioning.

## Current SigmaOS Status
Partitioning layout calculator and raw rootfs streaming engine implemented in Nim and Zig installer modules.

## Critical Missing Features
Interactive QML/Rust graphical wizard with automatic timezone geolocation and live install progress bars.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/installer/sigma_installer_tui.nim, src/installer/sigma_fast_partition_streamer.zig, src/iso/builder.rs

## Testing Strategy
Automated unattended installation test inside QEMU virtual machine (<45 seconds total install time), EFI boot verification.

## Dependencies
STORAGE_AGENT, BOOTLOADER_AGENT, SYSTEM_PROVISIONING_AGENT

## Success Criteria
Sub-60-second complete system installation to NVMe storage, 100% reliable UEFI and Legacy BIOS bootloader configuration.

## Open Source Competitors Analysis
5x faster installation than Ubuntu Ubiquity or Calamares by utilizing parallel multithreaded zstd block streaming.

## Future Enhancements
Network-boot (PXE / HTTP boot) automated diskless deployment streaming OS images directly over 10GbE fabrics.
