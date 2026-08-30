# ⚙️ System Administration

This document outlines core system administration tasks in SigmaOS, following **Arch Linux Wiki paradigms** for service supervision, journal logging, storage management, and networking.

---

## 🚦 Service Supervision (`systemctl` Parity)

SigmaOS features a **Sovereign Systemd Parity Engine** (`SovereignSystemdParityEngine`) supporting `.service`, `.socket`, `.target`, `.mount`, and `.slice` units.

### Basic Commands

```bash
# Start a service
systemctl start sshd.service

# Stop a service
systemctl stop sshd.service

# Restart a service
systemctl restart sshd.service

# Query service status
systemctl status sshd.service

# Enable service at boot
systemctl enable sshd.service
```

### System Target Runlevels
SigmaOS maps targets cleanly across SysVInit and Systemd runlevels (`SystemTarget`):

- `emergency.target` (Runlevel 0) - Emergency maintenance shell
- `rescue.target` (Runlevel 1) - Single-user recovery mode
- `multi-user.target` (Runlevel 3) - Multi-user non-graphical console
- `graphical.target` (Runlevel 5) - Full Zenith Desktop GUI environment

---

## 📜 System Logging (`journalctl` Parity)

Unified system logging is managed by the zero-copy, compressed journald subsystem (`UnifiedLogEntry`).

### Log Querying Commands

```bash
# View all recent logs
journalctl -n 50

# Follow real-time log stream
journalctl -f

# Filter logs by unit
journalctl -u sshd.service

# Filter logs by priority level
journalctl -p err
```

---

## 💾 Storage & Filesystem Management

SigmaOS supports advanced copy-on-write (CoW) filesystems, volume management, and partition tools.

### Supported Filesystems

1. **Ext4:** Fast-commit journal recovery (`Ext4FastCommit`).
2. **Btrfs:** Differential snapshot send/receive streams (`BtrfsSendReceiveEngine`).
3. **ZFS:** Native pool replication and dataset snapshots.
4. **FAT32 / NVMe:** Zero-copy DMA transfers for high-speed NVMe storage.

### Volume & Snapshot Commands

```bash
# Create Snapper CoW Pre-Snapshot before system update
snapper create --type pre --description "Pre-Kernel-Upgrade"

# Create Snapper Post-Snapshot after system update
snapper create --type post --pre-id 1 --description "Post-Kernel-Upgrade"

# List snapshots
snapper list
```

---

## 🌐 Network Configuration

Network interfaces and routing tables are managed via the dual-stack IPv4/IPv6 networking stack and BGP/mesh router engines.

```bash
# List network interfaces
ip link show

# Assign static IP address
ip addr add 192.168.1.100/24 dev eth0

# Bring interface up
ip link set eth0 up

# Display routing table
ip route show
```
