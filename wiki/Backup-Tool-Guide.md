# Backup Tool Guide: Personal Data & System Snapshots in SigmaOS

## Introduction

SigmaOS provides `sigbackup`, a powerful backup tool designed to protect your personal files and system state. It combines fast, space-saving deduplication with strong encryption and automatic scheduled backups.

## Backup Modes

1. **User Personal Data Backup**: Backs up documents, pictures, music, application settings (`~/.config`), and your installed package selection list.
2. **System State Snapshot**: Captures an atomic snapshot of system files (`/usr`, `/etc`) so you can restore your OS if an update or software change causes issues.

## Quick Start Guide

### 1. Creating a User Personal Data Backup
To back up your home folder to an external USB drive or network share:
```bash
sigbackup create --destination /media/usb/backup_repo --repository-passphrase
```

### 2. Restoring Personal Data
To list available backup snapshots and restore your files:
```bash
sigbackup list-snapshots --repository /media/usb/backup_repo
sigbackup restore --snapshot-id 2026-09-20-1000 --target /home/jules
```

### 3. System State Snapshots (OS Rollback Points)
To create a system snapshot before installing new drivers or upgrading packages:
```bash
sigbackup snapshot create --comment "Before GPU Driver Upgrade"
```

To view and restore prior system snapshots:
```bash
sigbackup snapshot list
sigbackup snapshot restore @snapshot-pre-upgrade
```

## Scheduled Automatic Backups

You can enable background scheduled backups in `~/.config/zenith/backup.toml`:
```toml
[schedule]
enabled = true
frequency = "daily"
keep_daily = 7
keep_weekly = 4
keep_monthly = 12
destination = "/media/backup_disk"
```
