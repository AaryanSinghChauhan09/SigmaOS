# Packaging

SigmaPkg is SigmaOS's universal package manager supporting 29+ Linux/BSD package formats.

## Package Management

### Basic Operations

Update package database:

```bash
sigpkg update
```

Upgrade installed packages:

```bash
sigpkg upgrade
```

Search for packages:

```bash
sigpkg search package-name
```

Install packages:

```bash
sigpkg install package-name
```

Remove packages:

```bash
sigpkg remove package-name
```

List installed packages:

```bash
sigpkg list
```

### Package Information

Show package details:

```bash
sigpkg info package-name
```

Show package dependencies:

```bash
sigpkg deps package-name
```

Show package files:

```bash
sigpkg files package-name
```

## Package Formats

### Supported Formats

SigmaPkg supports multiple package formats:

- `.deb` (Debian/Ubuntu)
- `.rpm` (Fedora/RHEL)
- `.apk` (Alpine)
- `.pkg.tar.xz` (Arch)
- `.ebuild` (Gentoo)
- `.xbps` (Void)
- `.txz` (Slackware)
- `.ipk` (OpenWrt)
- `.sigpkg` (SigmaOS native)

### Cross-Distro Packages

Install packages from other distributions:

```bash
# Install Debian package
sigpkg install package.deb

# Install RPM package
sigpkg install package.rpm

# Install Arch package
sigpkg install package.pkg.tar.xz
```

## Repository Configuration

### Add Repository

Add package repository:

```toml
# /etc/sigmaos/sigpkg.toml
[repositories]
official = "https://packages.sigmaos.org"
community = "https://community.sigmaos.org"
testing = "https://testing.sigmaos.org"
```

### Enable Repository

Enable repository:

```bash
sigpkg repo enable repository-name
```

## Package Development

### Creating Packages

Create SigmaPkg packages:

```bash
# Create package skeleton
sigpkg create package-name

# Build package
sigpkg build package-name

# Install local package
sigpkg install package-name.sigpkg
```

### Package Metadata

Package metadata in `SIGPKG.toml`:

```toml
[package]
name = "example-package"
version = "1.0.0"
description = "Example package description"
maintainer = "maintainer@example.com"

[dependencies]
required = ["dependency1", "dependency2"]
optional = ["optional-dependency"]

[files]
# List of files to include
```

## Package Signing

### Sign Packages

Sign packages for verification:

```bash
# Generate signing key
sigpkg keygen

# Sign package
sigpkg sign package.sigpkg

# Verify signature
sigpkg verify package.sigpkg
```

## Package Updates

### Auto Updates

Configure automatic updates:

```toml
# /etc/sigmaos/sigpkg.toml
[package]
auto_update = true
auto_cleanup = true
keep_old_versions = 3
```

### Manual Updates

Update specific packages:

```bash
# Update package
sigpkg update package-name

# Update all packages
sigpkg upgrade --all
```

## Package Cleanup

### Remove Unused Packages

Remove unused dependencies:

```bash
# Remove orphan packages
sigpkg autoremove

# Clean package cache
sigpkg clean
```

### Old Versions

Remove old package versions:

```bash
# Remove old versions
sigpkg remove --old-versions
```

## Distro Compatibility Models

`src/distro/sovereign_linux_bsd_ecosystem_leap_suite.rs` contains in-memory
models for Gentoo dependency reachability, Snapper transaction metadata,
FreeBSD boot-environment state, runit-style log retention, APKBUILD metadata,
and debconf answers. These models do not invoke host package tools, ZFS,
chroot, or filesystem rollback. Package builds and SHA-512 digests return
provider-unavailable errors; log input and retained history are bounded. Do
not present model state as an installed package, verified digest, isolated
build, or host system change.

Future changes to these models should preserve bounded inputs and resource use,
validate path components before constructing dataset or log paths, and fail
closed where package verification or host integration is unavailable. Tests
must distinguish state-model behavior from real host operations.

## Next Steps

- [Development](10-Development.md) - Development tools and building
- [Configuration](03-Configuration.md) - System configuration
- [Security](07-Security.md) - Package security and signing
