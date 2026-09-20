# System Troubleshooting Guide: Diagnostics & Repair in SigmaOS

## Introduction

SigmaOS includes `sigdoctor`, an automated diagnostic and troubleshooting tool. `sigdoctor` scans system health, identifies broken packages or misconfigured network interfaces, captures crash coredumps, and provides one-click automated repairs.

## Running System Health Audits

To run a full system health audit:
```bash
sigdoctor audit
```

Sample output:
```
 SigmaOS Doctor Health Audit
 [OK] Kernel & Memory Subsystem (RAM: 8.2GB free, ZRAM: OK)
 [OK] File System Integrity (/ & /home mounted read-write)
 [WARN] Network DNS: Primary DNS lookup failed, falling back to DoT
 [ERROR] Package Database: Found 1 orphan package entry
```

To automatically fix detected issues:
```bash
sigdoctor repair --auto
```

## Troubleshooting Common Issues

### 1. Network / Internet Connection Failure
Run network-specific diagnostic probes:
```bash
sigdoctor net-test
```
This tests network interface status, gateway ping, DNS resolution, and WireGuard VPN tunnel handshakes.

### 2. Broken Package Dependencies or Database Corruption
To verify Merkle store hash integrity and repair package database indices:
```bash
sigpkg verify-db
sigdoctor repair --packages
```

### 3. Application Crashes & Collecting Core Dumps
When an application crashes, view recent core dumps and backtraces:
```bash
sigdoctor coredump list
sigdoctor coredump info <pid>
```

To export an anonymized diagnostic bundle for bug reports:
```bash
sigdoctor export-bundle /tmp/bug-report.tar.zst
```

## Emergency Recovery & Boot Rollback

If the system fails to boot into the desktop environment:
1. Reboot the computer.
2. At the `sigma-boot` menu, select **"SigmaOS Recovery / Rollback"**.
3. Pick the latest snapshot (e.g. `@snapshot-pre-update`) to instantly restore system state.
