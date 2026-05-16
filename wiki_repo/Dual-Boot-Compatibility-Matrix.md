# SigmaOS Dual-Boot OS Compatibility Matrix

## Release: Dual-Boot v15.0

---

## Overview

This matrix documents verified co-existence behavior between SigmaOS and other operating systems on shared hardware. Tests validate bootloader handoff, partition isolation, and filesystem non-interference.

---

## Compatibility Matrix

| Co-Resident OS | Boot Method | Partition Scheme | Filesystem | SigmaOS Coexists | Notes |
|----------------|-------------|-----------------|------------|------------------|-------|
| **Ubuntu 24.04 LTS** | GRUB2 | GPT | ext4 | âœ… Verified | GRUB detects SigmaOS automatically |

| **Fedora 40** | GRUB2 | GPT | ext4/btrfs | âœ… Verified | EFI chainloading confirmed |

| **Debian 12** | GRUB2 | MBR/GPT | ext4 | âœ… Verified | Legacy BIOS + UEFI both tested |

| **Arch Linux** | systemd-boot | GPT | ext4/xfs | âœ… Verified | Manual bootloader entry required |

| **Windows 11** | Windows Boot Manager | GPT | NTFS | âœ… Verified | Fast Startup must be disabled |

| **Windows 10** | Windows Boot Manager | MBR/GPT | NTFS | âœ… Verified | Legacy MBR: GRUB must be primary |

| **macOS Ventura** | Apple Boot Picker | APFS | APFS | âš ï¸ Constrained | Apple Silicon unsupported; Intel only |

| **FreeBSD 14** | BSD Bootloader | GPT | UFS/ZFS | âš ï¸ Constrained | Manual partition table tuning needed |

| **OpenBSD 7.5** | OpenBSD Bootloader | GPT | FFS | âš ï¸ Constrained | Shared EFI partition required |

| **ChromeOS Flex** | coreboot | GPT | ext4 | ðŸ§ª Experimental | Developer mode required |

| **Android-x86** | GRUB | GPT | ext4 | ðŸ§ª Experimental | Partition alignment critical |

---

## Partition Layout Reference

### Recommended GPT Partition Layout (UEFI Systems)

```text
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Partition 1: EFI System (512MB)    â”‚ â† Shared with co-OS
â”‚  Type: vfat  /dev/sdX1              â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  Partition 2: SigmaOS Root (25GB+)  â”‚ â† SovereignLatticeFS
â”‚  Type: ext4  /dev/sdX2              â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  Partition 3: SigmaOS Recovery (2GB)â”‚ â† SovereignRecover
â”‚  Type: ext4  /dev/sdX3              â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚  Partition 4: Co-OS Partition       â”‚ â† Windows/Linux
â”‚  (Remainder of disk)                â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```text
---

## Known Conflicts & Mitigations

| Conflict | Affected OS | Mitigation |
|----------|-------------|------------|
| Windows Fast Startup locks NTFS | Windows 10/11 | Disable Fast Startup in Windows Power Settings |
| GRUB overwritten by Windows Update | Windows | Re-run `sovereign-bootctl install` post-update |
| Secure Boot key conflict | All UEFI | Enroll SigmaOS MOK key via `mokutil --import` |
| Clock skew (UTC vs Local) | Windows | Set Windows to use UTC via registry key |

---

## Recovery Procedure

If SigmaOS fails to boot:

1. Boot from SigmaOS Live USB

2. Run `sovereign-recover --detect-partitions`

3. Re-install GRUB: `sovereign-bootctl install --target=/dev/sdX`

4. Verify: `sovereign-bootctl verify`
