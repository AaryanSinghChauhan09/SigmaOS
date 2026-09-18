# 🎛️ System Administration & Service Supervision

SigmaOS provides a unified, highly resilient system administration framework combining declarative service supervision (`systemd`, `OpenRC`, and `Runit` parity), capability-based privilege delegation (`doas`), eBPF structured logging (`journalctl`), and native networking.

---

## ⚙️ Service Supervision Engine

SigmaOS service supervision (`src/distro/systemd_parity.rs`, `src/distro/void_runit.rs`) manages system services using simple, validated TOML unit manifests (`SovereignServiceManifest`).

### 1. Service Manifest Example (`/etc/services/nginx.toml`):
```toml
[unit]
description = "Nginx Web Server"
after = ["network.target"]
requires = ["network.target"]

[service]
exec_start = "/usr/bin/nginx -g 'daemon off;'"
restart = "always"
restart_sec = 2
user = "www-data"
sandbox = "pledge_unveil"

[sandboxing]
pledge = "stdio rpath wpath cpath inet"
unveil = ["/etc/nginx:r", "/var/www:r", "/var/log/nginx:rwc"]
```

### 2. Service Management Commands:
```bash
# Start a service
sigctl start nginx

# Enable service at boot
sigctl enable nginx

# Query service status
sigctl status nginx

# List running services
sigctl list-units
```

---

## 🔑 Capability-Based Privilege Delegation (`doas`)

SigmaOS replaces traditional `sudo` with an enhanced implementation of OpenBSD `doas` (`SovereignOpenBsdDoas`).

### Configuration (`/etc/doas.conf`):
```conf
# Allow wheel group members to run commands as root with password prompt
permit keepenv :wheel as root

# Allow sovereign user to execute package manager without password
permit nopass sovereign as root cmd sigpkg
```

### Usage:
```bash
# Execute command with root capabilities
doas sigpkg update

# Execute command as another user
doas -u www-data touch /var/www/index.html
```

---

## 🌐 Network Configuration & WireGuard

Networking in SigmaOS (`src/net/`) is controlled via `sigma-sh net` or the Zenith Wayland network tray applet (`LxqtNetworkManagerTray`).

### 1. Static & DHCP IP Binding:
```bash
# List network interfaces
sigma-sh net list

# Assign static IPv4 address
sigma-sh net set eth0 address 192.168.1.150/24 gateway 192.168.1.1

# Enable DHCP autoconfiguration
sigma-sh net dhcp eth0 enable
```

### 2. WireGuard VPN Tunnels:
SigmaOS includes a native, in-kernel WireGuard protocol implementation (`src/net/wireguard.rs`):
```bash
# Load WireGuard configuration tunnel
sigma-sh wg up /etc/wireguard/wg0.conf

# Query active WireGuard peer status
sigma-sh wg show
```

---

## 📜 Structured Logging & Journalctl (`journalctl`)

SigmaOS logs system events into a zero-allocation binary ring buffer using `SovereignJournalLogger`.

### Querying Logs with `journalctl`:
```bash
# View live tailing system logs
journalctl -f

# Filter logs by service unit
journalctl -u nginx

# Filter logs by priority level (err, warn, info)
journalctl -p err

# Export logs in JSON format
journalctl -u nginx --output=json
```
