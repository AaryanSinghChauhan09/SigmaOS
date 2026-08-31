# Debian/Ubuntu APT Compatibility in SigmaOS

## Overview

SigmaOS includes a zero-dependency, clean-room subsystem providing comprehensive compatibility with **Debian Linux** and **Ubuntu** and their **APT** package manager. This subsystem allows Debian/Ubuntu packages and DEB files to be parsed, resolved, and managed natively on SigmaOS.

---

## Key Modules

- [`src/sigpkg/debian_apt_engine.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/sigpkg/debian_apt_engine.rs): APT package manager, DEB control file parser, and dpkg compatibility
- [`src/sigpkg/mod.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/sigpkg/mod.rs): Unified package orchestrator integrating native `.spkg`, Debian `.deb`, and other formats

---

## Features

| Feature | SigmaOS Implementation | Notes |
|---------|------------------------|-------|
| **DEB Control Parsing** | Native pure-Rust parser | Extracts package metadata without dpkg tools |
| **APT Repository Support** | Sources.list format compatibility | Supports main, universe, multiverse repositories |
| **Dependency Resolution** | Recursive dependency graph | Handles complex dependency trees with Recommends/Suggests |
| **dpkg Database** | Native package database emulation | Compatible with dpkg status database format |
| **Package Priorities** | Essential/Required/Important/Standard/Optional | Debian priority system support |

---

## Architecture Flow

```
Debian/Ubuntu Repository
       │ (Download Packages)
       ▼
[DEB Control Parser] ──> Validates package metadata
       │
       ▼
[Dependency Graph Resolver] ───> Resolves Depends/Pre-Depends/Recommends
       │
       ▼
[dpkg Database] ────────────> Maintains package status
       │
       ▼
[SigmaPkg Native Index] ───────> Integrates with native package system
```

---

## CLI Usage

```bash
# Search Debian packages
sigma-pkg search nginx

# Install from Debian repository
sigma-pkg apt-install nginx

# Update Debian repositories
sigma-pkg apt-update

# Query package information
sigma-pkg apt-show nginx

# List installed packages
sigma-pkg apt-list
```

---

## Implementation Details

### DEB Package Structure

```rust
pub struct DebPackage {
    pub package: String,
    pub version: String,
    pub architecture: String,
    pub maintainer: String,
    pub description: String,
    pub depends: Vec<String>,
    pub pre_depends: Vec<String>,
    pub recommends: Vec<String>,
    pub suggests: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
    pub installed_size: u64,
    pub section: String,
    pub priority: String,
}
```

### APT Repository Configuration

```rust
pub struct AptRepository {
    pub name: String,
    pub url: String,
    pub distribution: String,
    pub components: Vec<String>,
    pub enabled: bool,
}
```

### Dependency Fields

- **Depends**: Required dependencies
- **Pre-Depends**: Required before installation
- **Recommends**: Recommended but not required
- **Suggests: Optional suggestions
- **Conflicts**: Conflicting packages
- **Provides: Virtual package provides

---

## Repository Support

- **Debian**: Main, contrib, non-free repositories
- **Ubuntu**: Main, restricted, universe, multiverse repositories
- **Custom**: User-defined repositories

### Repository Example

```
deb http://archive.ubuntu.com/ubuntu/ focal main restricted
deb http://archive.ubuntu.com/ubuntu/ focal-updates main restricted
deb http://security.ubuntu.com/ubuntu/ focal-security main restricted
```

---

## Integration with SigmaOS

The Debian APT engine integrates seamlessly with:
- **SigmaPkg**: Native package manager
- **init system**: Systemd compatibility (optional)
- **Filesystem**: Standard Linux filesystem hierarchy
- **Service management**: Service compatibility layer

---

## Benefits

1. **Zero-Dependency**: No external APT tools required
2. **Debian Compatibility**: Access to vast Debian/Ubuntu package ecosystem
3. **Dependency Management**: Sophisticated dependency resolution
4. **Priority System**: Debian's package priority system
5. **Flexibility**: Mix Debian/Ubuntu and native SigmaOS packages

---

## Examples

### Web Server Installation

```bash
# Install nginx from Ubuntu repository
sigma-pkg apt-install nginx

# Install Apache with dependencies
sigma-pkg apt-install apache2

# Query package details
sigma-pkg apt-show nginx
```

### Development Tools

```bash
# Install development tools
sigma-pkg apt-install build-essential
sigma-pkg apt-install git
sigma-pkg apt-install cmake
```

### Desktop Environment

```bash
# Install desktop environment
sigma-pkg apt-install ubuntu-desktop
sigma-pkg apt-install gnome-desktop
```

---

## Comparison with Original APT

| Feature | Original APT | SigmaOS Implementation |
|---------|-------------|------------------------|
| **Dependency Resolution** | SAT solver | Recursive graph resolver |
| **Package Database** | /var/lib/dpkg | Native HashMap database |
| **Repository Cache** | /var/cache/apt/archives | Native cache system |
| **Configuration** | /etc/apt/ | Native configuration system |
| **Tool Dependency** | apt-get, apt-cache | Zero external tools |

---

**Generated:** August 24, 2026  
**Repository:** [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)