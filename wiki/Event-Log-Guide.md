# Event Log Guide: System Diagnostics & Logging in SigmaOS

## Introduction

SigmaOS uses a centralized, high-performance binary journal logging system (`sigma-journald`). All kernel messages, daemon logs, and system events are indexed in real time, making system inspection, troubleshooting, and audit logging simple and efficient.

## Querying Logs with `journalctl`

The primary command for viewing and filtering system logs in SigmaOS is `journalctl`.

### 1. Basic Log Viewing

To view all system logs from newest to oldest or page through them:
```bash
journalctl
```

To stream real-time logs as events occur (like `tail -f`):
```bash
journalctl -f
```

### 2. Viewing Kernel Logs (`dmesg` equivalent)

To view only kernel-space messages (hardware detection, driver initialization, kernel panics):
```bash
journalctl -k
```

To inspect kernel logs from the current boot session:
```bash
journalctl -b
```

### 3. Filtering Logs by Service or Application

To view logs for a specific service daemon (such as the Zenith display server or network manager):
```bash
journalctl -u zenith-compositor.service
```

To filter logs by executable path or PID:
```bash
journalctl /usr/bin/zenith-compositor
journalctl _PID=1240
```

### 4. Filtering by Priority / Severity Level

Journal entries are classified into 8 priority levels:
- `0` - Emergency (system unusable)
- `1` - Alert (immediate action required)
- `2` - Critical
- `3` - Error
- `4` - Warning
- `5` - Notice
- `6` - Informational
- `7` - Debug

To filter logs to error levels or higher:
```bash
journalctl -p err..emerg
```

### 5. Filtering by Time Range

To search for logs within a specific timeframe:
```bash
journalctl --since "2026-09-20 08:00:00" --until "2026-09-20 10:00:00"
journalctl --since "1 hour ago"
journalctl --since "yesterday"
```

## Exporting & Formatting Logs

To output logs as formatted JSON for automated analysis or security monitoring:
```bash
journalctl -u zenith-compositor.service -o json-pretty
```

To export raw dmesg log buffers to a plain text file:
```bash
dmesg > /tmp/dmesg_boot.log
```

## Troubleshooting Common Scenarios

1. **Boot Failure Analysis**: Run `journalctl -b -1 -p err` to view errors from the previous failed boot session.
2. **Device Connection Issues**: Run `journalctl -f -k` before plugging in a USB device or peripheral to watch driver events live.
3. **Application Crash Investigation**: Check `/var/log/journal/` or run `journalctl -e` (jump to end of journal) after an application closes unexpectedly.
