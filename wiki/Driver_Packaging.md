# SigmaOS Driver Packaging Guide

## Overview

This guide explains how to convert driver packages from various Linux distributions into the SigmaOS sigpkg format, ensuring security, reproducibility, and proper integration with the SigmaOS package ecosystem.

## sigpkg Format

### Package Structure

```
driver-name-version.sigpkg/
├── SIGPKG_MANIFEST.json    # Package metadata
├── SIGPKG_SIGNATURE.sig    # GPG signature
├── kernel-module.ko        # Compiled kernel module
├── firmware/               # Firmware files
│   ├── firmware.bin
│   └── firmware.lic
├── config/                 # Configuration files
│   └── driver.conf
└── docs/                   # Documentation
    ├── README.md
    └── LICENSE
```

### SIGPKG_MANIFEST.json

```json
{
  "name": "nvidia-driver",
  "version": "535.154.05",
  "arch": "x86_64",
  "type": "kernel-driver",
  "kernel_version": "6.1.0",
  "dependencies": [
    "kernel >= 6.1.0",
    "nvidia-firmware"
  ],
  "provides": [
    "nvidia",
    "nvidia-current"
  ],
  "conflicts": [
    "nouveau"
  ],
  "firmware_required": true,
  "secure_boot": true,
  "sbom": "sbom.json",
  "build_info": {
    "builder": "sigma-build-farm-v1",
    "build_time": "2024-01-15T10:30:00Z",
    "git_commit": "abc123...",
    "reproducible": true
  }
}
```

## Converting from RPM (Fedora)

### Analyzing RPM Spec

1. **Extract RPM spec file**:
   ```bash
   rpm -qsp nvidia-driver-535.154.05-1.fc39.src.rpm
   ```

2. **Key sections to convert**:
   - `%description` → SIGPKG_MANIFEST description
   - `Requires` → dependencies array
   - `Provides` → provides array
   - `Conflicts` → conflicts array
   - `%install` → installation instructions
   - `%post` → post-install scripts

### Conversion Example

**Fedora RPM Spec**:
```spec
Name: nvidia-driver
Version: 535.154.05
Release: 1%{?dist}
Requires: kernel >= 6.1.0
Requires: nvidia-firmware
Provides: nvidia
Conflicts: nouveau
```

**SigmaOS sigpkg Manifest**:
```json
{
  "name": "nvidia-driver",
  "version": "535.154.05",
  "dependencies": [
    "kernel >= 6.1.0",
    "nvidia-firmware"
  ],
  "provides": ["nvidia"],
  "conflicts": ["nouveau"]
}
```

## Converting from PKGBUILD (Arch)

### Analyzing PKGBUILD

1. **Extract PKGBUILD**:
   ```bash
   wget https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h=nvidia
   ```

2. **Key fields to convert**:
   - `pkgname` → name
   - `pkgver` → version
   - `depends` → dependencies
   - `provides` → provides
   - `conflicts` → conflicts
   - `makedepends` → build dependencies

### Conversion Example

**Arch PKGBUILD**:
```bash
pkgname=nvidia
pkgver=535.154.05
depends=('linux>=6.1.0' 'nvidia-utils')
provides=('nvidia')
conflicts=('nouveau')
```

**SigmaOS sigpkg Manifest**:
```json
{
  "name": "nvidia-driver",
  "version": "535.154.05",
  "dependencies": [
    "kernel >= 6.1.0",
    "nvidia-utils"
  ],
  "provides": ["nvidia"],
  "conflicts": ["nouveau"]
}
```

## Converting from Debian (deb)

### Analyzing debian/control

1. **Extract control file**:
   ```bash
   dpkg -I nvidia-driver_535.154.05_amd64.deb
   ```

2. **Key fields to convert**:
   - `Package` → name
   - `Version` → version
   - `Depends` → dependencies
   - `Provides` → provides
   - `Conflicts` → conflicts

### Conversion Example

**Debian Control**:
```
Package: nvidia-driver
Version: 535.154.05-1
Depends: linux-image-6.1.0, nvidia-firmware
Provides: nvidia
Conflicts: nouveau
```

**SigmaOS sigpkg Manifest**:
```json
{
  "name": "nvidia-driver",
  "version": "535.154.05",
  "dependencies": [
    "kernel >= 6.1.0",
    "nvidia-firmware"
  ],
  "provides": ["nvidia"],
  "conflicts": ["nouveau"]
}
```

## Building Drivers

### Build Environment

1. **Set up build container**:
   ```bash
   sigma-build create-driver-env --kernel 6.1.0
   ```

2. **Install build dependencies**:
   ```bash
   sigma-build install-deps nvidia-driver
   ```

3. **Compile driver**:
   ```bash
   sigma-build compile nvidia-driver
   ```

4. **Package driver**:
   ```bash
   sigma-build package nvidia-driver
   ```

### Build Script Template

```bash
#!/bin/bash
# build-driver.sh

DRIVER_NAME=$1
KERNEL_VERSION=$2

# Set up build environment
export SIGMA_BUILD_ROOT=/tmp/sigma-build
mkdir -p $SIGMA_BUILD_ROOT

# Download source
sigma-build download $DRIVER_NAME

# Compile
make -C /lib/modules/$KERNEL_VERSION/build M=$SIGMA_BUILD_ROOT/$DRIVER_NAME modules

# Sign module
sigmod-sign $SIGMA_BUILD_ROOT/$DRIVER_NAME/*.ko

# Create sigpkg
sigma-pkg create \
  --name $DRIVER_NAME \
  --kernel $KERNEL_VERSION \
  --output $DRIVER_NAME.sigpkg
```

## Signing Packages

### GPG Key Setup

1. **Generate signing key**:
   ```bash
   gpg --full-generate-key --key-type RSA --key-length 4096
   ```

2. **Export public key**:
   ```bash
   gpg --export --armor > sigmaos-keyring.asc
   ```

3. **Sign package**:
   ```bash
   sigma-pkg sign nvidia-driver.sigpkg --key sigmaos-keyring
   ```

### Secure Boot Integration

1. **Sign kernel module**:
   ```bash
   sigmod-sign nvidia.ko --key-db /var/lib/shim-signed/mok
   ```

2. **Generate signature**:
   ```bash
   sigmod-sign --output nvidia.ko.sig nvidia.ko
   ```

3. **Verify signature**:
   ```bash
   sigmod-verify nvidia.ko nvidia.ko.sig
   ```

## Firmware Packaging

### Firmware Structure

```
firmware.sigpkg/
├── SIGPKG_MANIFEST.json
├── SIGPKG_SIGNATURE.sig
└── firmware/
    ├── nvidia/
    │   ├── gp107/gp107.bin
    │   └── gp107/gp107.lic
    └── LICENSE
```

### Firmware Manifest

```json
{
  "name": "nvidia-firmware",
  "version": "535.154.05",
  "type": "firmware",
  "license": "proprietary",
  "firmware_files": [
    "nvidia/gp107/gp107.bin",
    "nvidia/gp107/gp107.lic"
  ],
  "license_files": [
    "LICENSE.nvidia"
  ]
}
```

## Configuration Files

### Driver Configuration

```
config/driver.conf
```

```ini
[Driver]
Name=nvidia
Version=535.154.05
KernelVersion=6.1.0

[Options]
CoolBits=28
RegistryDwords=0x00000001

[PowerManagement]
PowerMizerLevel=1
```

### Installation Script

```bash
#!/bin/bash
# post-install.sh

# Load module
modprobe nvidia

# Create device nodes
nvidia-smi

# Update initramfs
update-initramfs -u
```

## Testing Packages

### Unit Tests

```bash
# Test package structure
sigma-pkg validate nvidia-driver.sigpkg

# Test signature
sigma-pkg verify nvidia-driver.sigpkg

# Test installation
sigma-pkg install --test nvidia-driver.sigpkg
```

### Integration Tests

```bash
# Install package
sigma-pkg install nvidia-driver.sigpkg

# Load module
modprobe nvidia

# Test functionality
nvidia-smi

# Clean up
modprobe -r nvidia
sigma-pkg remove nvidia-driver
```

## Repository Management

### Adding to Repository

```bash
# Add package to repository
sigma-repo add nvidia-driver.sigpkg

# Update repository index
sigma-repo update

# Sync repository
sigma-repo sync
```

### Repository Structure

```
sigmaos-repo/
├── x86_64/
│   ├── nvidia-driver-535.154.05.sigpkg
│   ├── nvidia-firmware-535.154.05.sigpkg
│   └── repo-index.json
├── keyring/
│   └── sigmaos-keyring.asc
└── metadata/
    └── repo-metadata.json
```

## Best Practices

### Versioning

- Follow upstream versioning
- Use semantic versioning for SigmaOS-specific changes
- Include kernel version in package version if required

### Dependencies

- Specify minimum kernel version
- List firmware dependencies
- Avoid circular dependencies

### Security

- Sign all packages with GPG
- Sign kernel modules for Secure Boot
- Verify signatures before installation

### Documentation

- Include README with usage instructions
- Document configuration options
- Provide troubleshooting guide

## Troubleshooting

### Build Failures

- Check kernel headers compatibility
- Verify build dependencies
- Review build logs

### Installation Issues

- Verify package signature
- Check kernel version compatibility
- Review conflicts with other packages

### Runtime Issues

- Check dmesg for driver errors
- Verify firmware loading
- Review configuration files

## References

- [sigpkg Specification](../specs/sigpkg-spec.md)
- [Driver Reproducibility Guide](Driver_Reproducibility.md)
- [SigmaOS Build System](../BUILD.md)
- [GPG Documentation](https://gnupg.org/documentation/)
