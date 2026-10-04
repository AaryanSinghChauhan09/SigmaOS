# System Management and Initialization

SigmaOS provides modern system management inspired by systemd, OpenRC, and BSD init systems.

## SystemD-Compatible Init System
**Location:** `src/init/systemd.rs`

Modern initialization and service management daemon.

### Core Architecture

```
Kernel Boot
    ↓
PID 1: systemd
    ↓
├─ System Initialization Targets
│  ├─ sysinit.target (early boot)
│  ├─ basic.target (basic system)
│  └─ multi-user.target (full system)
├─ Service Management
│  ├─ Start/Stop/Restart services
│  ├─ Dependency resolution
│  └─ Parallel startup
├─ Device Management (udev integration)
├─ Mount Management (fstab, automount)
├─ Socket Activation
├─ Timer Units (cron replacement)
├─ Resource Control (cgroups v2)
└─ Logging (journald)
```

---

## Service Management

### Unit Files

Services defined in INI-style unit files:

**Example: `/etc/systemd/system/nginx.service`**
```ini
[Unit]
Description=Nginx HTTP Server
After=network.target
Wants=network-online.target

[Service]
Type=forking
PIDFile=/run/nginx.pid
ExecStartPre=/usr/sbin/nginx -t
ExecStart=/usr/sbin/nginx
ExecReload=/bin/kill -s HUP $MAINPID
ExecStop=/bin/kill -s QUIT $MAINPID
Restart=on-failure
RestartSec=5s

[Install]
WantedBy=multi-user.target
```

### Unit Types

1. **Service Units (`.service`):**
   - Daemons and background processes
   - Most common unit type

2. **Socket Units (`.socket`):**
   - Socket-activated services
   - Service starts on first connection
   - Reduces memory usage

3. **Target Units (`.target`):**
   - Synchronization points (like runlevels)
   - Group units together
   - Examples: multi-user.target, graphical.target

4. **Timer Units (`.timer`):**
   - Time-based activation (cron replacement)
   - Monotonic and calendar timers
   - Example: backup.timer

5. **Mount Units (`.mount`):**
   - Filesystem mount points
   - Auto-generated from /etc/fstab

6. **Path Units (`.path`):**
   - Path-based activation
   - Start service when file/directory changes

7. **Device Units (`.device`):**
   - Device-based activation
   - Generated from udev rules

### Service Types

- **simple:** Main process runs in foreground (default)
- **forking:** Service forks daemon process (traditional Unix)
- **oneshot:** Process exits after completing task
- **dbus:** Service registers on D-Bus
- **notify:** Service sends readiness notification (sd_notify)
- **idle:** Delayed start until system idle

---

## Dependency Management

### Ordering Dependencies

- **After=:** Start after specified units
- **Before=:** Start before specified units

**Example:**
```ini
[Unit]
After=network.target
Before=nginx.service
```

### Requirement Dependencies

- **Requires=:** Hard dependency (fails if dependency fails)
- **Wants=:** Soft dependency (continues if dependency fails)
- **Requisite=:** Dependency must already be active
- **BindsTo=:** Service stops if dependency stops
- **PartOf=:** Service stops/restarts with dependency

### Conflict Dependencies

- **Conflicts=:** Cannot run simultaneously
- **Example:** `rescue.service` conflicts with all normal services

---

## Service Control

### Systemctl Commands

```bash
# Service management
systemctl start nginx.service      # Start service
systemctl stop nginx.service       # Stop service
systemctl restart nginx.service    # Restart service
systemctl reload nginx.service     # Reload config (SIGHUP)
systemctl status nginx.service     # Check status

# Enable/disable on boot
systemctl enable nginx.service     # Auto-start on boot
systemctl disable nginx.service    # Don't auto-start
systemctl is-enabled nginx.service # Check if enabled

# System state
systemctl list-units               # All loaded units
systemctl list-units --type=service --state=running  # Running services
systemctl list-dependencies nginx.service  # Show dependencies

# System targets
systemctl isolate multi-user.target  # Change target
systemctl get-default              # Show default target
systemctl set-default graphical.target  # Set default target

# System control
systemctl reboot                   # Reboot system
systemctl poweroff                 # Shutdown
systemctl suspend                  # Suspend to RAM
systemctl hibernate                # Suspend to disk
```

---

## Socket Activation

Services start on-demand when socket receives connection.

**Example: SSH socket activation**

**`/etc/systemd/system/sshd.socket`:**
```ini
[Unit]
Description=SSH Socket
Before=ssh.service

[Socket]
ListenStream=22
Accept=yes

[Install]
WantedBy=sockets.target
```

**`/etc/systemd/system/sshd@.service`:**
```ini
[Unit]
Description=SSH Per-Connection Server

[Service]
ExecStart=/usr/sbin/sshd -i
StandardInput=socket
```

**Benefits:**
- Faster boot (services not started until needed)
- Lower memory usage (idle services unloaded)
- Automatic restart on crash (next connection spawns service)

---

## Timer Units (Cron Replacement)

Time-based service activation.

**Example: Backup timer**

**`/etc/systemd/system/backup.timer`:**
```ini
[Unit]
Description=Daily Backup

[Timer]
OnCalendar=daily
OnCalendar=Mon *-*-* 02:00:00  # Every Monday 2am
Persistent=true  # Run missed timers on boot

[Install]
WantedBy=timers.target
```

**`/etc/systemd/system/backup.service`:**
```ini
[Unit]
Description=Backup Service

[Service]
Type=oneshot
ExecStart=/usr/local/bin/backup.sh
```

**Timer Specifications:**
- `OnCalendar=`: Calendar-based (like cron)
- `OnBootSec=`: Time after boot
- `OnStartupSec=`: Time after systemd startup
- `OnUnitActiveSec=`: Relative to last activation (periodic)

**Advantages over cron:**
- Persistent timers (run missed jobs)
- Logging via journald
- Resource control (cgroups)
- Dependency management

---

## Resource Control (Cgroups v2)

Limit CPU, memory, I/O for services.

**Example: Limit service resources**
```ini
[Service]
CPUQuota=50%           # Max 50% CPU
MemoryMax=1G           # Max 1GB RAM
MemoryHigh=800M        # Soft limit (throttle, not kill)
IOWeight=200           # I/O priority (100-10000)
TasksMax=100           # Max processes/threads
```

**Slice Hierarchy:**
```
- (root cgroup)
  ├─ system.slice (system services)
  │  ├─ nginx.service
  │  └─ sshd.service
  ├─ user.slice (user sessions)
  │  └─ user-1000.slice
  └─ machine.slice (containers/VMs)
```

**Accounting:**
```bash
systemd-cgtop  # Top-like cgroup monitor
systemctl status nginx.service  # Shows CPU/memory usage
```

---

## Journal Logging (Journald)

Structured, indexed logging system.

**Features:**
- Binary log format (efficient, indexed)
- Automatic log rotation
- Integration with syslog
- Metadata (timestamp, PID, service, priority)
- Persistent/volatile storage

### Journalctl Commands

```bash
# View logs
journalctl                          # All logs
journalctl -u nginx.service         # Service logs
journalctl -f                       # Follow (tail -f)
journalctl -k                       # Kernel messages (dmesg)
journalctl -b                       # Current boot
journalctl -b -1                    # Previous boot

# Time filtering
journalctl --since "2027-01-01"
journalctl --since "1 hour ago"
journalctl --until "2027-01-02 12:00"

# Priority filtering
journalctl -p err                   # Errors only
journalctl -p warning..emerg        # Warning and above

# Output formats
journalctl -o json                  # JSON format
journalctl -o json-pretty           # Pretty JSON
journalctl -o cat                   # Message only (no metadata)

# Log management
journalctl --disk-usage             # Show disk usage
journalctl --vacuum-size=1G         # Keep only 1GB
journalctl --vacuum-time=30d        # Keep only 30 days
```

**Persistent Storage:**
```bash
mkdir -p /var/log/journal
systemctl restart systemd-journald
# Logs now survive reboot
```

---

## Device Management (Udev Integration)

Systemd integrates with udev for hardware events.

**Udev Rules:**
```
# /etc/udev/rules.d/99-usb-webcam.rules
ACTION=="add", SUBSYSTEM=="video4linux", ATTR{name}=="*Webcam*", \
  TAG+="systemd", ENV{SYSTEMD_WANTS}="webcam-handler.service"
```

When webcam plugged in → `webcam-handler.service` starts automatically.

---

## Network Management

### systemd-networkd

Network configuration daemon.

**Example: `/etc/systemd/network/20-wired.network`**
```ini
[Match]
Name=eth0

[Network]
DHCP=yes
DNS=1.1.1.1 8.8.8.8

[DHCP]
UseDomains=yes
```

**Static IP:**
```ini
[Match]
Name=eth0

[Network]
Address=192.168.1.100/24
Gateway=192.168.1.1
DNS=192.168.1.1
```

### systemd-resolved

DNS resolver daemon.

**Features:**
- DNSSEC validation
- DNS over TLS (DoT)
- mDNS (Multicast DNS) for .local domains
- LLMNR (Link-Local Multicast Name Resolution)

**Configuration: `/etc/systemd/resolved.conf`**
```ini
[Resolve]
DNS=1.1.1.1 8.8.8.8
FallbackDNS=9.9.9.9
DNSSEC=allow-downgrade
DNSOverTLS=opportunistic
```

---

## Boot Process

### Boot Stages

1. **Firmware (UEFI/BIOS):**
   - Hardware initialization
   - Load bootloader

2. **Bootloader (GRUB, systemd-boot):**
   - Load kernel and initramfs
   - Pass kernel parameters

3. **Initramfs (Initial RAM Filesystem):**
   - Minimal root filesystem
   - Load drivers for root filesystem
   - Mount real root filesystem

4. **systemd (PID 1):**
   - Parse unit files
   - Resolve dependencies
   - Start services in parallel

5. **System Targets:**
   - `sysinit.target`: Early boot (fsck, mount)
   - `basic.target`: Basic system utilities
   - `multi-user.target`: Full multi-user system
   - `graphical.target`: Graphical login

### Boot Targets (Runlevels)

| Target | Runlevel | Description |
|--------|----------|-------------|
| poweroff.target | 0 | Shutdown |
| rescue.target | 1 | Single-user mode |
| multi-user.target | 3 | Multi-user, no GUI |
| graphical.target | 5 | Multi-user with GUI |
| reboot.target | 6 | Reboot |

**Change default target:**
```bash
systemctl set-default multi-user.target  # Boot to console
systemctl set-default graphical.target   # Boot to GUI
```

---

## User Sessions

### systemd --user

Per-user systemd instance managing user services.

**User unit files:** `~/.config/systemd/user/`

**Example: Auto-start Syncthing**
```ini
# ~/.config/systemd/user/syncthing.service
[Unit]
Description=Syncthing File Sync

[Service]
ExecStart=/usr/bin/syncthing -no-browser

[Install]
WantedBy=default.target
```

```bash
systemctl --user enable syncthing.service
systemctl --user start syncthing.service
```

**Lingering (run services without login):**
```bash
loginctl enable-linger $USER
# User services now start at boot (even without login)
```

---

## Performance Optimization

### Parallel Startup

Systemd starts independent services in parallel:
```
Time: 0s──────1s──────2s──────3s──────4s──────5s
      ├─ network.service ───────┐
      ├─ sshd.socket             ├─ nginx.service
      ├─ postgresql.service ──────┘
      └─ redis.service
```

**Traditional init (serial):**
```
Time: 0s──────2s──────4s──────6s──────8s──────10s
      network → sshd → postgresql → redis → nginx
```

**Boot time improvement:** 50-70% faster

### Lazy Loading

- Socket activation delays service start
- Automount delays filesystem mount
- Path activation delays service until file access

### Optimizing Boot Time

```bash
# Analyze boot time
systemd-analyze                     # Total boot time
systemd-analyze blame               # Services by time
systemd-analyze critical-chain      # Critical path

# Disable unnecessary services
systemctl disable bluetooth.service
systemctl mask bluetooth.service    # Prevent accidental enable
```

---

## Development Roadmap

### Short-term (Q1-Q2 2027)

1. **Systemd Compatibility:**
   - Complete systemd v253 API compatibility
   - Portable services (systemd-portabled)
   - Home directories (systemd-homed)
   - systemd-oomd (out-of-memory killer)

2. **Enhanced Journald:**
   - Remote logging (journal-remote, journal-upload)
   - Forward-secure sealing (cryptographic tamper detection)
   - Compression (zstd, lz4)

3. **Advanced Resource Control:**
   - CPU affinity pinning
   - NUMA awareness
   - I/O latency targets
   - Memory pressure (PSI - Pressure Stall Information)

4. **Container Integration:**
   - systemd-nspawn improvements
   - Podman integration
   - Container registries

### Mid-term (Q3-Q4 2027)

1. **Boot Optimization:**
   - Predictive service loading (AI/ML)
   - Boot-time reduction techniques
   - Lazy service initialization

2. **Security Enhancements:**
   - Service hardening profiles (DynamicUser, ProtectSystem)
   - Credential management (LoadCredential, SetCredential)
   - Encrypted credentials

3. **Monitoring and Observability:**
   - Prometheus exporter for systemd metrics
   - Grafana dashboards
   - Real-time performance profiling

4. **Alternative Init Systems:**
   - OpenRC support (Gentoo-style)
   - runit support (Void Linux-style)
   - s6 support (minimal, suckless-style)

### Long-term (2028+)

1. **Declarative System Configuration:**
   - NixOS-style immutable system
   - Atomic updates and rollbacks
   - Reproducible builds

2. **AI-Powered Management:**
   - Anomaly detection (service crashes, resource spikes)
   - Automatic remediation (restart strategies)
   - Predictive maintenance

3. **Distributed System Management:**
   - Multi-node cluster coordination
   - Distributed service dependencies
   - Cross-host socket activation

4. **Next-Gen Init:**
   - User-space Linux (UML) integration
   - Microkernel-style initialization
   - Capability-based service isolation

---

## Performance Benchmarks

### Boot Time
- **Traditional init (sysvinit):** 30-45 seconds
- **SystemD (parallel):** 5-15 seconds
- **SigmaOS systemd:** 8-12 seconds (target: <5s)

### Resource Usage
- **Memory:** 10-15MB (systemd daemon + journald)
- **CPU:** <1% idle, 5-10% during boot

### Service Management
- **Start/Stop Latency:** <50ms for simple services
- **Dependency Resolution:** O(n log n) complexity
- **Journal Write Throughput:** 10,000+ messages/second

---

## Testing Strategy

### Testing Services

```bash
# Syntax check
systemd-analyze verify nginx.service

# Dry-run
systemctl --dry-run start nginx.service

# Test dependencies
systemctl list-dependencies --reverse nginx.service

# Simulate failures
systemctl kill --signal=SIGKILL nginx.service
# Should auto-restart if Restart=on-failure
```

### Testing Boot Process

```bash
# Test boot sequence
systemctl list-jobs  # During boot

# Test different targets
systemctl isolate rescue.target
systemctl isolate multi-user.target

# Test boot time
systemd-analyze plot > boot.svg  # Generate timeline
```

---

## Migration from Other Init Systems

### From SysVinit

**Old:** `/etc/init.d/nginx start`  
**New:** `systemctl start nginx.service`

**Convert init script:**
```bash
# Automatic conversion (basic)
systemd-sysv-generator /etc/init.d/nginx

# Manual unit file creation (recommended)
# See "Unit Files" section above
```

### From Upstart (Ubuntu)

**Old:** `start nginx`  
**New:** `systemctl start nginx`

**Convert job file:** Similar process to SysVinit

### From OpenRC (Gentoo)

**Old:** `/etc/init.d/nginx start`  
**New:** `systemctl start nginx`

OpenRC services often translate 1:1 to systemd units.

---

## Troubleshooting

### Common Issues

**Service fails to start:**
```bash
systemctl status nginx.service  # Check status
journalctl -xe -u nginx.service  # View logs
systemd-analyze verify nginx.service  # Check syntax
```

**Slow boot:**
```bash
systemd-analyze blame  # Find slow services
systemd-analyze critical-chain  # Find critical path
systemctl disable slow-service.service  # Disable offender
```

**Out of memory:**
```bash
systemctl status  # Check if OOM occurred
journalctl -k | grep -i oom  # Kernel OOM messages
# Adjust MemoryMax= in service file
```

**Permission denied:**
```bash
# Check SELinux denials
ausearch -m avc -ts recent
# Check service user/group
systemctl show nginx.service | grep User
```

---

## References

- systemd Documentation: https://systemd.io/
- systemd for Administrators: https://www.freedesktop.org/wiki/Software/systemd/
- Arch Linux systemd Wiki: https://wiki.archlinux.org/title/Systemd
- systemd GitHub: https://github.com/systemd/systemd

---

**Last Updated:** October 2, 2026  
**Maintainers:** SigmaOS Init Team  
**License:** MPL-2.0 (same as SigmaOS kernel)

## AI Agent Maintenance Instructions
- Language constraints: Strictly Rust (`#![no_std]`), Zig, or Nim only. No C/C++ or Python dependencies.
- Prioritize memory safety, zero-allocation patterns, lock-free primitives, and kernel stability.
- Verify that `cargo check --lib` passes cleanly after any modification.
- Maintain comprehensive unit and property tests.
