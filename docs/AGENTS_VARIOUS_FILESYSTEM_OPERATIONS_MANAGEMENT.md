# AI Agent Various File System Operations Management Specification for SigmaOS

This document specifies operational standards for AI agents managing various filesystems (Ext4, Btrfs, ZFS, XFS, F2FS, HAMMER2, SquashFS, OverlayFS, FAT32) in **SigmaOS**.

---

## 1. File System Operations Protocol

AI agents operating across filesystems must adhere to the following rules:

1. **Mount Management**:
   - Manage mounts via `SovereignMountManager`. Apply `MS_NOATIME` on flash storage to prevent write wear.

2. **Btrfs CoW Snapshots**:
   - Create read-only Btrfs subvolume snapshots prior to package updates or configuration modifications.

3. **OverlayFS Stacking**:
   - Stack read-only SquashFS layers (`lowerdir`) with writable ephemeral upper directories (`upperdir`).

4. **EFI Partition Rules**:
   - Restrict FAT32 ESP modifications (`/boot/efi`) to signed bootloader stage-1/stage-2 artifacts.

---

## 2. Verification Protocol

- Verify filesystem operations by running `./run_sigma_tests.sh` and `tests/stress_and_fuzz_tests.rs`.

---

*Maintained by the SigmaOS Storage Engineering Committee.*
