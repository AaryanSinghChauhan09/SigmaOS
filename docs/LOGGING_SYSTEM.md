# SigmaOS Centralized Logging System Specification (`Sigma Journald`)

## 1. Overview

The SigmaOS Centralized Logging Subsystem provides structured, append-only, tamper-evident binary logging across kernel space (`dmesg`) and userland system daemons (`sigma-init`, network managers, desktop services). It combines `journalctl` query compatibility with post-quantum signature verification for audit integrity.

## 2. Architecture & Log Entry Storage

Log records are written to indexed binary journal files located in `/var/log/journal/`:

```
+-----------------------------------------------------------------+
|                       Log Message Sources                       |
|  +--------------------+  +--------------------+  +------------+ |
|  | Kernel printk      |  | System Services    |  | App Logs   | |
|  | (/dev/kmsg)        |  | (stdout / stderr)  |  | (Syslog API| |
|  +---------+----------+  +---------+----------+  +-----+------+ |
+------------|-----------------------|-------------------|--------+
             |                       |                   |
+------------v-----------------------v-------------------v--------+
|                      Sigma Journal Daemon                       |
|  +------------------------------------------------------------+ |
|  | Structured Field Collector (PID, UID, UNIT, TIMESTAMP)      | |
|  | Merkle Tree Hash Chain & Binary Journal Storage Engine     | |
|  +------------------------------------------------------------+ |
+------------------------------------+----------------------------+
                                     |
+------------------------------------v----------------------------+
|             Log Query Interface (`journalctl` / API)            |
+-----------------------------------------------------------------+
```

### 2.1 Structured Field Metadata
Every log entry automatically captures structured metadata:
- **`__REALTIME_TIMESTAMP`**: 64-bit microsecond UTC epoch timestamp.
- **`_PID`**: Process ID of the logging process.
- **`_UID` / `_GID`**: User and group ownership IDs.
- **`_SYSTEMD_UNIT` / `_SERVICE`**: Name of the supervising service unit.
- **`PRIORITY`**: Syslog severity level (`0` Emergency .. `7` Debug).
- **`MESSAGE`**: Primary log text message UTF-8 string.

## 3. Querying Logs (`journalctl`)

The `journalctl` command provides high-speed binary indexing queries:
- **Kernel Logs Only**: `journalctl -k` or `journalctl -b` (current boot).
- **Service Unit Logs**: `journalctl -u zenith-compositor.service`.
- **Priority Filter**: `journalctl -p err..emerg`.
- **Real-Time Follow**: `journalctl -f`.
- **Time Range Query**: `journalctl --since "2026-09-20 08:00:00" --until "2026-09-20 10:00:00"`.

## 4. Log Rotation & Storage Management

- **Storage Limits**: Automatically caps log directory disk footprint (default: 10% of VFS partition or 4GB maximum).
- **Log Compaction**: Inactive journal files are compressed using LZ4 or Zstd compression.
- **Forward-Secure Sealing (FSS)**: Uses HMAC state key rotation to prevent unauthorized retroactive log file modification.
