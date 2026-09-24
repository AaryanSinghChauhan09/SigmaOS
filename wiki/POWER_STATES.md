# SigmaOS Power State Transitions & Suspend/Hibernate Architecture

## 1. Overview

SigmaOS supports all standard ACPI system power states, providing fast suspend-to-RAM (S3 / Modern Standby S0ix) and crash-resilient hibernate-to-disk (S4) workflows.

## 2. System Power State Taxonomy

| Power State | Name | Description | Power Draw | Resume Latency |
|---|---|---|---|---|
| **S0** | Working | System fully operational; active C/P-state management | 15W - 300W | Instant (0 ms) |
| **S0ix** | Modern Standby / Low Power Idle | Display off; CPU in package C10; PCIe links in L1.2 low-power state | < 1W | < 500 ms |
| **S3** | Suspend to RAM (STR) | CPU powered down; main RAM maintained in self-refresh mode | ~1.5W | ~1 - 2 sec |
| **S4** | Hibernate to Disk (STD) | RAM state compressed and written to swap partition; total power off | 0W | ~10 - 15 sec |
| **S5** | Soft Off / Shutdown | Graceful service termination, file system unmount, poweroff command | 0W | Cold Boot |

## 3. Suspend & Resume Execution Flow

```
+-----------------------------------------------------------+
| 1. User Requests Suspend (Lid Close / Desktop Power Menu) |
+-----------------------------+-----------------------------+
                              |
               Kernel Freeze Userland Threads
                              |
+-----------------------------v-----------------------------+
| 2. Device Driver Pre-Suspend Callbacks                    |
|    (Save GPU context, pause network interfaces, sync VFS) |
+-----------------------------+-----------------------------+
                              |
               Disable Non-Boot CPU Cores (Hotplug)
                              |
+-----------------------------v-----------------------------+
| 3. Execute ACPI Sleep State Vector (`\ _S3`)              |
|    (Put System RAM into Self-Refresh Mode, Power Off CPU) |
+-----------------------------+-----------------------------+
                              |
             Hardware Wake Event (Power Button / Lid Open)
                              |
+-----------------------------v-----------------------------+
| 4. System Resume Flow                                     |
|    (Re-initialize CPU, restore driver states, unfreeze)   |
+-----------------------------------------------------------+
```

## 4. Hibernate to Disk (S4 Engine)

1. **Memory State Compression**: Compresses dirty anonymous pages and cache states using LZ4 / Zstd compression.
2. **Swap Header Target**: Writes compressed memory images to a designated Swap Partition or Swap File (`/swapfile`).
3. **Restoration Verification**: On boot, `sigma-boot` inspects the swap header signature (`SWAP-SPACE-SIGMA-S4`). If a valid hibernate image is detected, `sigma-boot` streams the memory image back into RAM, resuming desktop state instantly without restarting daemons.
