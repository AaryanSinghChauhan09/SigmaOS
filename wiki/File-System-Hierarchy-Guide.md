# File System Hierarchy Guide: Directory Layout & Path Resolution in SigmaOS

## Introduction

SigmaOS combines the usability and standardization of Linux with the clean isolation and security models of BSD distributions. This guide explains how directories are organized in SigmaOS, how paths are resolved, and how developers can structure application data and configurations.

## Directory Layout Overview

```
 /                           # Sovereign VFS Root
 ├── bin -> usr/bin          # UsrMerge binary path
 ├── dev/                    # Hardware device nodes
 ├── etc/                    # System configurations
 ├── home/                   # User directories
 ├── proc/                   # Process information
 ├── sys/                    # Kernel object information
 ├── run/                    # Volatile runtime state
 ├── sigma/                  # Sovereign Store & Generations
 ├── usr/                    # Immutable System Applications
 │   └── local/              # Custom / Ported Third-Party Apps
 └── var/                    # Variable state (logs, caches)
```

## Key Architectural Concepts

### 1. Unified UsrMerge
In SigmaOS, all system executables reside in `/usr/bin`. Traditional Linux paths (`/bin`, `/sbin`, `/usr/sbin`) are symlinks pointing directly to `/usr/bin`. This ensures that scripts referencing `#!/bin/bash` or `#!/bin/sh` execute seamlessly without modification.

### 2. Base System vs. Third-Party Packages
Inspired by FreeBSD, SigmaOS enforces a clear boundary between base OS software and user-installed software:
- **Base System Software**: Stored in `/usr/bin`, `/usr/lib`, and `/etc/`.
- **Third-Party / Ported Software**: Installed in `/usr/local/bin`, `/usr/local/lib`, and `/usr/local/etc/`.

### 3. Merkle Package Store (`/sigma/store/`)
Inspired by NixOS, applications installed via `.sigpkg` can be stored in `/sigma/store/<hash>-<pkgname>/`. This allows multiple versions of the same library to coexist without collision. Symlinks in `/usr/bin` point to active generation binaries.

## Application Developer Guidelines

When building or packaging applications for SigmaOS:

1. **User Configurations**: Always follow XDG base directory specifications:
   - User Configs: `~/.config/<appname>/`
   - User Data: `~/.local/share/<appname>/`
   - User Cache: `~/.cache/<appname>/`
2. **System Configurations**: Place global default configuration templates in `/etc/<appname>/` or `/usr/local/etc/<appname>/`.
3. **Temporary Files**: Use `/run/user/<uid>/` for user runtime sockets and ephemeral files.
4. **Log Storage**: Direct daemon log output to stdout/stderr or use `journalctl` structured logging API instead of writing unindexed raw text files.
