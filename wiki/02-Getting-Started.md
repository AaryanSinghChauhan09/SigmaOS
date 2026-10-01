# Getting Started

This guide covers first-time configuration and basic usage of SigmaOS.

## First Boot

On first boot, you will be greeted by the Zenith desktop environment with the onboarding wizard.

### Onboarding Wizard

The onboarding wizard will guide you through:

1. **Language and Region Selection**
2. **Network Configuration**
3. **User Account Setup**
4. **Desktop Theme Selection**
5. **Privacy Settings**

## Basic Configuration

### System Settings

Access system settings via the Zenith Control Center:

```bash
# Launch control center
sigma-control-center
```

### Terminal Access

Open the Sigma Shell:

```bash
# Terminal shortcut: Super+T
sigma-shell
```

### Package Management

Update the system:

```bash
sigpkg update
sigpkg upgrade
```

Install packages:

```bash
sigpkg install package-name
```

Remove packages:

```bash
sigpkg remove package-name
```

## Desktop Environment

### Zenith Compositor

Zenith is SigmaOS's native compositor providing:

- Direct hardware GPU rendering
- Zero-copy framebuffers
- Low-latency input handling
- Multi-monitor support

### Shortcuts

- **Super**: Open application launcher
- **Super+T**: Open terminal
- **Super+D**: Show desktop
- **Super+Shift+S**: Screenshot
- **Super+L**: Lock screen

## System Services

View running services:

```bash
sigma-systemctl list
```

Enable/disable services:

```bash
sigma-systemctl enable service-name
sigma-systemctl disable service-name
```

## Filesystem

SigmaOS uses a custom filesystem layout compatible with Linux FHS:

- `/` - Root filesystem
- `/home` - User home directories
- `/usr` - System software
- `/etc` - Configuration files
- `/var` - Variable data
- `/tmp` - Temporary files

## Security

### Pledge/Unveil Sandbox

SigmaOS uses pledge/unveil for process sandboxing:

```bash
# Run application with sandbox
pledge unveil /path/to/app
```

### Capability-based Security

Process capabilities restrict access to resources:

```bash
# Grant specific capabilities
sigcaps grant process-name read:/etc/config
```

## Getting Help

- **Manual Pages**: `man command-name`
- **Wiki**: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki
- **Issue Tracker**: https://github.com/AaryanSinghChauhan09/SigmaOS/issues

## Next Steps

- [Configuration](03-Configuration.md) - Advanced system configuration
- [Kernel](04-Kernel.md) - Kernel subsystems and modules
- [Filesystems](05-Filesystems.md) - Storage and filesystem options

## AI Agent Maintenance Instructions

- **Bolt ⚡**: Ensure documentation of any new zero-allocation optimizations or performance improvements are added concisely without marketing fluff.
- **Palette 🎨**: Maintain Arch Linux wiki style: clear, factual, one page per topic, using appropriate markdown formatting and tables where necessary.
- **Sentinel 🛡️**: Verify that no hardcoded credentials or unvetted cryptographic algorithms are documented as production-ready. Ensure security limitations are accurately stated.
- **General**: Keep pages up-to-date with current repository capabilities. Remove redundant files when consolidating information.
