# SigmaOS Backup Tool Architecture Specification (`sigbackup`)

## 1. Executive Summary

The SigmaOS Backup Tool Subsystem (`sigbackup`) provides comprehensive data protection, system state snapshotting, and disaster recovery. Inspired by Linux Timeshift, Linux Mint Backup Tool (`mintbackup`), BorgBackup / Restic deduplicated chunking, and FreeBSD `zfs send`/`receive` dataset replication, `sigbackup` manages both system OS rollbacks and encrypted user personal data archives.

## 2. System Architecture

```
+------------------------------------------------------------------+
|                      Backup Selection Inputs                     |
|  +---------------------------+      +--------------------------+ |
|  | System OS State (Btrfs /  |      | User Personal Data       | |
|  | ZFS / Root VFS Snapshots) |      | (~/Documents, Pictures)  | |
|  +-------------+-------------+      +------------+-------------+ |
+----------------|---------------------------------|---------------+
                 |                                 |
+----------------v---------------------------------v---------------+
|                   Sigma Backup Daemon (`sigbackupd`)             |
|  +-------------------------------------------------------------+ |
|  | 1. Fast Content-Defined Chunking (Rabin Fingerprinting)     | |
|  | 2. Content-Addressed Hash Deduplication (BLAKE3)            | |
|  | 3. Authenticated Stream Encryption (AES-256-GCM / Post-PQC)  | |
|  | 4. Compression Pass (Zstd / LZ4)                            | |
|  +-------------------------------+-----------------------------+ |
+----------------------------------|-------------------------------+
                                   |
+----------------------------------v-------------------------------+
|                      Destination Repositories                     |
|  +--------------------+  +------------------+  +---------------+ |
|  | Local USB Drive    |  | Local Network    |  | Remote S3 /   | |
|  | (/media/backup)    |  | NAS (NFS/SMB)    |  | SSH Server    | |
|  +--------------------+  +------------------+  +---------------+ |
+------------------------------------------------------------------+
```

## 3. Dual-Mode Backup Paradigm

1. **System State Snapshots (Timeshift Mode)**:
   - Takes atomic, copy-on-write snapshots of system root partitions (`@system`).
   - Integrated with `sigma-boot` for pre-boot system recovery.
   - Ignores volatile user files (`/tmp`, `/run`, `/var/cache`).
2. **User Personal Data Archives (MintBackup / Borg Mode)**:
   - Backs up personal documents, browser profiles, desktop settings, and installed package selection lists (`~/.config`, `/home/user`).
   - Applies BLAKE3 chunk deduplication, LZ4 compression, and AES-256-GCM encryption.

## 4. Deduplication & Encryption Pipeline

- **Content-Defined Chunking**: Splits files into variable-size chunks (64KB - 1MB) using Rabin fingerprinting.
- **BLAKE3 Hash Deduplication**: Identical data blocks across files or backup runs are stored exactly once in the repository chunk store.
- **Repository Integrity Verification**: `sigbackup check --repository /media/backup` verifies chunk hash trees and detects silent bitrot.

## 5. CLI & Desktop Integration

- **Command Line**: `sigbackup create`, `sigbackup restore`, `sigbackup list-snapshots`.
- **Scheduled Automated Backups**: Background cron/timer execution with retention policies (e.g. keep 7 daily, 4 weekly, 12 monthly archives).
