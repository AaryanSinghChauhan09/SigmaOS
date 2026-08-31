# Fedora/RPM Compatibility in SigmaOS

## Overview

SigmaOS includes a zero-dependency, clean-room subsystem providing comprehensive compatibility with **Fedora Linux**, **RHEL**, and other **RPM-based distributions**. This subsystem allows RPM packages and spec files to be parsed, resolved, and managed natively on SigmaOS.

---

## Key Modules

- [`src/sigpkg/fedora_rpm_engine.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/sigpkg/fedora_rpm_engine.rs): RPM package manager, spec file parser, and DNF/YUM compatibility
- [`src/sigpkg/mod.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/sigpkg/mod.rs): Unified package orchestrator integrating native `.spkg`, RPM, and other formats

---

## Features

| Feature | SigmaOS Implementation | Notes |
|---------|------------------------|-------|
| **RPM Spec Parsing** | Native pure-Rust parser | Extracts package metadata without rpmbuild |
| **DNF/YUM Compatibility** | Repository format support | Supports DNF repository metadata |
| **Dependency Resolution** | RPM dependency graph | Handles Requires/Provides/Conflicts/Obsoletes |
| **RPM Database** | Native package database emulation | Compatible with rpmdb format |
| **Repository Management** | .repo file format | Supports multiple repositories |

---

## Architecture Flow

```
Fedora/RHEL Repository
       │ (Download RPMs)
       ▼
[RPM Spec Parser] ──────> Validates package metadata
       │
       ▼
[Dependency Resolver] ───> Resolves Requires/Provides/Conflicts
       │
       ▼
[RPM Database] ───────────> Maintains package status
       │
       ▼
[SigmaPkg Native Index] ───────> Integrates with native package system
```

---

## CLI Usage

```bash
# Search Fedora packages
sigma-pkg search firefox

# Install from Fedora repository
sigma-pkg dnf-install firefox

# Update Fedora repositories
sigma-pkg dnf-update

# Query package information
sigma-pkg rpm-show firefox

# List installed packages
sigma-pkg rpm-list
```

---

## Implementation Details

### RPM Package Structure

```rust
pub struct RpmPackage {
    pub name: String,
    pub version: String,
    pub release: String,
    pub epoch: u32,
    pub architecture: String,
    pub summary: String,
    pub description: String,
    pub license: String,
    pub url: String,
    pub vendor: String,
    pub build_time: u64,
    pub size: u64,
    pub requires: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub obsoletes: Vec<String>,
}
```

### DNF Repository Configuration

```rust
pub struct DnfRepository {
    pub id: String,
    pub name: String,
    pub baseurl: String,
    pub enabled: bool,
    pub gpgcheck: bool,
    pub metadata_expire: u64,
}
```

### RPM Spec File Format

The spec file parser handles standard sections:
- **%description**: Package description
- **%files**: File list and attributes
- **%install**: Installation scripts
- **%clean**: Cleanup scripts
- **%changelog: Version changelog

---

## Repository Support

- **Fedora**: Main, updates, updates-testing repositories
- **RHEL**: Base, AppStream, Extras repositories
- **CentOS**: Base, Updates, Extras repositories
- **Custom**: User-defined repositories

### Repository Example

```
[fedora]
name=Fedora Repository
baseurl=https://download.fedoraproject.org/pub/fedora/linux/releases/39/Everything/x86_64/os/
enabled=1
gpgcheck=1
```

---

## Dependency Resolution

### Dependency Fields

- **Requires**: Required dependencies
- **Provides**: Virtual packages provided
- **Conflicts**: Conflicting packages
- **Obsoletes: Obsolete packages

### Dependency Resolution Process

1. Resolve direct dependencies
2. Check for conflicts
3. Handle obsoletes
4. Apply provides/requires relationships
5. Calculate final dependency tree

---

## Integration with SigmaOS

The Fedora/RPM engine integrates seamlessly with:
- **SigmaPkg**: Native package manager
- **Systemd**: Systemd service compatibility
- **SELinux**: Security-enhanced Linux integration
- **Filesystem**: RPM-standard filesystem layout

---

## Benefits

1. **Zero-Dependency**: No external RPM tools required
2. **Fedora Ecosystem**: Access to latest Fedora packages
3 **Enterprise Support**: RHEL/CentOS compatibility
4. **Sophisticated Resolution**: Complex dependency handling
5. **Enterprise Features**: GPG checking, repository management

---

## Examples

### Desktop Environment

```bash
# Install Fedora Workstation edition packages
sigma-pkg dnf-install @workstation-product-environment

# Install GNOME desktop
sigma-pkg dnf-install gnome-desktop
```

### Development Tools

```bash
# Install development tools
sigma-pkg dnf-install @development-tools
sigma-pkg dnf-install rust cargo
sigma-pkg dnf-install go
```

### Server Components

```bash
# Install web server
sigma-pkg dnf-install httpd

# Install database server
sigma-pkg dnf-install mariadb-server
```

---

## RPM Spec File Example

```spec
Name: sigmaos-example
Version: 1.0.0
Release: 1%{?dist}
Summary: Example package for SigmaOS RPM compatibility
License: MIT
URL: https://example.com

%description
This is an example package demonstrating SigmaOS RPM compatibility.
It provides essential functionality for the system.

%files
/usr/bin/sigmaos-example
/usr/share/doc/sigmaos-example/README
```

---

## Comparison with Original DNF/YUM

| Feature | Original DNF/YUM | SigmaOS Implementation |
|---------|----------------|------------------------|
| **Dependency Solver** | DNF's libsolv | Native recursive resolver |
| **Package Database** | /var/lib/rpm | Native HashMap database |
| **Repository Cache** | /var/cache/dnf | Native cache system |
| **Configuration** | /etc/dnf/ | Native configuration system |
| **Tool Dependency** | dnf, yum | Zero external tools |

---

## SELinux Integration

SigmaOS provides SELinux integration for RPM packages:
- **Security Contexts**: File and process security contexts
- **Policy Management**: SELinux policy enforcement
- **Compatibility**: Fedora/RHEL SELinux policies

---

## GPG Key Management

RPM packages can be verified with GPG keys:
- **Repository Keys**: Repository GPG keys
- **Package Signatures**: Package signature verification
- **Key Import**: GPG key import and management

---

**Generated:** August 24, 2026  
**Repository:** [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)