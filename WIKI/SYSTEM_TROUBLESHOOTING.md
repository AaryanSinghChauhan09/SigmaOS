# SigmaOS System Troubleshooting & Automated Diagnostics Architecture Specification (`sigdoctor`)

## 1. Executive Summary

The SigmaOS System Troubleshooting Framework (`sigdoctor` / `sigdiag`) provides automated health auditing, crash report collection, database integrity checks, network diagnostic probes, and interactive auto-repair workflows. It draws architectural inspiration from Linux Fedora ABRT, Ubuntu Apport, Arch chroot recovery, and FreeBSD single-user emergency shell diagnostics.

## 2. Diagnostics & Doctor Architecture

```
+------------------------------------------------------------------+
|               System Component Probes & Event Collectors         |
|  +---------------+  +---------------+  +---------------+  +----+ |
|  | Kernel Dmesg  |  | VFS / Storage |  | Network Stack |  | Cgroup|
|  | Errors        |  | (Fsck / SMART)|  | Connectivity  |  | Memory|
|  +-------+-------+  +-------+-------+  +-------+-------+  +--+-+ |
+----------|------------------|------------------|-------------|---+
           |                  |                  |             |
+----------v------------------v------------------v-------------v---+
|                   Sigma Doctor Daemon (`sigdoctor`)             |
|  +-------------------------------------------------------------+ |
|  | 1. System Health Audit Rule Evaluator                       | |
|  | 2. Automated Diagnostic Report Generator                    | |
|  | 3. Crash Coredump Collector & Stack Unwinder                 | |
|  | 4. Auto-Repair Action Coordinator (Package DB, FS, Network)   | |
|  +-------------------------------------------------------------+ |
+-------------------------------+----------------------------------+
                                |
+-------------------------------+----------------------------------+
|                   CLI / Desktop HUD Output                       |
|           (`sigdoctor audit`, `sigdoctor repair --auto`)         |
+------------------------------------------------------------------+
```

## 3. Automated Diagnostic Checks

`sigdoctor` audits system health across 6 core categories:

1. **Kernel & Memory Health**:
   - Checks kernel ring buffer (`dmesg`) for `OOM-Killer` invocations, hardware MCE errors, or kernel panics.
   - Audits RAM and ZRAM compression allocation efficiency.
2. **File System & Storage Integrity**:
   - Inspects SMART attributes for physical NVMe/SATA drive degradation.
   - Audits Btrfs/ZFS subvolume dirty flags and checks read-only mount states.
3. **Package Database & Merkle Store Consistency**:
   - Verifies package database integrity (`sigpkg verify-db`) against Merkle store hashes (`/sigma/store`).
4. **Network & DNS Diagnostics**:
   - Tests gateway ICMP ping reachability, Encrypted DNS-over-TLS resolution, and WireGuard VPN tunnel handshakes.
5. **Display & Compositor Health**:
   - Audits Zenith compositor rendering framerate, DRM/KMS buffer allocations, and GPU driver resets.
6. **Daemon & Service Supervision**:
   - Detects crashed or rapidly restarting services (`sigma-init` service restarts).

## 4. Crash Coredump Collector & Stack Unwinding

- **Automated Coredump Capture**: When an unhandled process signal occurs (`SIGSEGV`, `SIGABRT`), `sigcoredump` saves process memory states and backtraces to `/var/log/coredump/`.
- **Anonymized Diagnostic Bundles**: Generates compressed diagnostic tarballs (`/tmp/sigdiag-report.tar.zst`) with personal data scrubbed prior to bug report submission.

## 5. Emergency Recovery Modes

- **Pre-Boot Rollback**: Triggered automatically when 3 consecutive boot failures occur (`sigma-boot` snapshot fallback).
- **Single-User Rescue Shell**: Minimal emergency maintenance shell mounted with immutable system defaults.
