# Configuration

SigmaOS uses a declarative configuration system inspired by NixOS.

## Configuration System

### Declarative System Configuration

System configuration is defined in `/etc/sigmaos/configuration.toml`:

```toml
[system]
hostname = "sigmaos-desktop"
timezone = "America/New_York"
locale = "en_US.UTF-8"

[network]
hostname = "sigmaos-desktop"
dhcp = true

[desktop]
compositor = "zenith"
theme = "dark"
animations = true

[security]
sandboxing = true
firewall = true
encryption = true
```

### Apply Configuration

After modifying configuration:

```bash
# Apply configuration changes
sigma-config apply

# Rollback to previous configuration
sigma-config rollback
```

## Kernel Configuration

### Kernel Parameters

Edit `/etc/sigmaos/kernel.toml`:

```toml
[kernel]
log_level = "info"
security_mitigations = true
memory_management = "auto"

[performance]
cpu_governor = "performance"
iopriority = "best-effort"
```

### Kernel Modules

Load kernel modules:

```bash
# Load module
sigmod load module-name

# List loaded modules
sigmod list

# Configure module options
sigmod configure module-name option=value
```

## Desktop Configuration

### Zenith Compositor

Configure Zenith in `/etc/sigmaos/zenith.toml`:

```toml
[compositor]
backend = "drm"
output_scale = "auto"
vsync = true

[input]
keyboard_layout = "us"
mouse_acceleration = "adaptive"

[appearance]
theme = "dark"
font = "system-ui"
icon_theme = "sigma-icons"
```

### Keyboard Shortcuts

Customize keyboard shortcuts in `/etc/sigmaos/shortcuts.toml`:

```toml
[shortcuts]
terminal = "Super+T"
launcher = "Super"
file_manager = "Super+E"
browser = "Super+B"
```

## Network Configuration

### Wired Network

Configure via `/etc/sigmaos/network.toml`:

```toml
[interface.eth0]
type = "wired"
method = "dhcp"
```

Static IP:

```toml
[interface.eth0]
type = "wired"
method = "static"
address = "192.168.1.100/24"
gateway = "192.168.1.1"
dns = ["8.8.8.8", "8.8.4.4"]
```

### Wireless Network

```toml
[interface.wlan0]
type = "wireless"
ssid = "network-name"
security = "wpa2"
password = "your-password"
```

## Package Configuration

### Repository Configuration

Add package repositories in `/etc/sigmaos/sigpkg.toml`:

```toml
[repositories]
official = "https://packages.sigmaos.org"
community = "https://community.sigmaos.org"
```

### Package Preferences

```toml
[package]
auto_update = true
auto_cleanup = true
keep_old_versions = 3
```

## Security Configuration

### Firewall Rules

Configure firewall in `/etc/sigmaos/firewall.toml`:

```toml
[firewall]
default_policy = "deny"

[[firewall.rules]]
service = "ssh"
port = 22
action = "allow"

[[firewall.rules]]
service = "http"
port = 80
action = "allow"
```

### User Permissions

Configure user capabilities:

```bash
# Grant capabilities to user
sigcaps grant username capability-name

# List user capabilities
sigcaps list username
```

## User Configuration

### User Accounts

Create user accounts:

```bash
# Add user
sigma-user add username

# Set password
sigma-user set-password username

# Add to groups
sigma-user add-group username group-name
```

### User Profile

Configure user profile in `/home/username/.config/sigmaos/profile.toml`:

```toml
[user]
shell = "/bin/sigma-shell"
editor = "nano"
browser = "sigma-web"

[preferences]
theme = "dark"
language = "en_US"
```

## Service Configuration

### System Services

Enable/disable services:

```bash
# Enable service
sigma-systemctl enable service-name

# Disable service
sigma-systemctl disable service-name

# Start service
sigma-systemctl start service-name

# Stop service
sigma-systemctl stop service-name
```

### Service Configuration

Configure services in `/etc/sigmaos/services/`:

```toml
[service.sshd]
enabled = true
auto_start = true
port = 22

[service.cups]
enabled = true
auto_start = true
```

## Troubleshooting

### Configuration Not Applying

Check configuration syntax:

```bash
sigma-config validate
```

Check system logs:

```bash
journalctl -xe
```

### Reset Configuration

Reset to default configuration:

```bash
sigma-config reset
```

## Next Steps

- [Kernel](04-Kernel.md) - Kernel configuration and modules
- [Filesystems](05-Filesystems.md) - Storage and filesystem configuration
- [Security](07-Security.md) - Security settings and hardening

## AI Agent Maintenance Instructions

- **Bolt ⚡**: Ensure documentation of any new zero-allocation optimizations or performance improvements are added concisely without marketing fluff.
- **Palette 🎨**: Maintain Arch Linux wiki style: clear, factual, one page per topic, using appropriate markdown formatting and tables where necessary.
- **Sentinel 🛡️**: Verify that no hardcoded credentials or unvetted cryptographic algorithms are documented as production-ready. Ensure security limitations are accurately stated.
- **General**: Keep pages up-to-date with current repository capabilities. Remove redundant files when consolidating information.
