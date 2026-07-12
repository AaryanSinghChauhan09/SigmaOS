# SigmaOS Installer Specification

## Overview

The SigmaOS Installer (`siginstall`) provides a polished graphical and command-line interface based on Calamares-style module flows. To ensure robust security from initial setup, the installer mandates UEFI Secure Boot configuration and home directory encryption (via LUKS2 and Argon2id) by default.

### Key Features

- **Polished UI**: Calamares-style graphical installer with modern design
- **Security First**: Mandatory Secure Boot and full-disk encryption
- **Flexible Profiles**: Multiple installation profiles for different use cases
- **Accessibility**: Screen reader support and keyboard navigation
- **Indic Languages**: Full support for Indian languages
- **Automated Installation**: Unattended installation support
- **Rollback Capability**: Installation rollback on failure

## Installation Architecture

### Installation Flow

```
 [Welcome Page] ──► [Select Profile] ──► [Partition Drive]
                                                 │
                                                 ▼
 [Install System Files] ◄── [Set LUKS Key] ◄── [Secure Boot Setup]
         │
         ▼
 [Reboot into Clean System]
```

### Installation Modules

1. **Welcome Module**: Introduction and language selection
2. **Profile Module**: Installation profile selection
3. **Partition Module**: Disk partitioning and filesystem setup
4. **Encryption Module**: LUKS2 encryption configuration
5. **Secure Boot Module**: UEFI Secure Boot setup
6. **Installation Module**: System files installation
7. **Configuration Module**: User and system configuration
8. **Reboot Module**: Installation completion and reboot

## System Requirements

### Minimum Requirements

- **CPU**: x86_64 processor, 2 cores
- **RAM**: 4 GB minimum, 8 GB recommended
- **Storage**: 20 GB minimum, 50 GB recommended
- **Boot**: UEFI firmware with Secure Boot support
- **Network**: Internet connection for package download

### Recommended Requirements

- **CPU**: x86_64 processor, 4 cores or more
- **RAM**: 16 GB or more
- **Storage**: 100 GB or more SSD
- **Boot**: UEFI 2.3.1 or later
- **Network**: High-speed internet connection

## Installation Profiles

### Minimal Profile

**Target**: Developers, servers, minimal installations

**Components**:
- Base system (kernel, sigmad, sigpkg)
- Terminal utilities
- Network tools
- SSH server
- Basic development tools

**Disk Space**: ~5 GB

### Desktop Profile

**Target**: General users, office work

**Components**:
- Base system
- Zenith Desktop
- Office suite (LibreOffice)
- Web browser
- Media player
- System utilities

**Disk Space**: ~15 GB

### Development Profile

**Target**: Software developers

**Components**:
- Base system
- Development tools (GCC, Rust, Python)
- IDE support
- Version control (Git)
- Documentation
- Debugging tools

**Disk Space**: ~20 GB

### Education Profile

**Target**: Students, educators

**Components**:
- Base system
- Zenith Desktop
- Education tools (GeoGebra, Scilab, Octave)
- Office suite
- Learning management system
- Indian language support

**Disk Space**: ~25 GB

### Security Profile

**Target**: Security professionals

**Components**:
- Base system
- SigmaSec suite
- Security tools (Kali tools)
- Forensic tools
- Network analysis tools
- Documentation

**Disk Space**: ~20 GB

## Security Configuration

### Secure Boot

**Requirements**:
- UEFI firmware with Secure Boot support
- SigmaOS signing key enrollment
- Kernel and driver signature verification

**Process**:
1. Detect Secure Boot status
2. Generate Machine Owner Key (MOK)
3. Enroll MOK in firmware
4. Sign kernel and initramfs
5. Configure bootloader

**Implementation**:
```c
// installer/secure_boot.c
#include <efi.h>

EFI_STATUS enroll_mok(EFI_HANDLE image_handle) {
    EFI_GUID mok_protocol_guid = MOK_PROTOCOL_GUID;
    MOK_PROTOCOL *mok_protocol;
    EFI_STATUS status;

    // Locate MOK protocol
    status = gBS->LocateProtocol(&mok_protocol_guid, NULL, (void**)&mok_protocol);
    if (EFI_ERROR(status)) {
        return status;
    }

    // Generate MOK
    status = mok_protocol->GenerateKey();
    if (EFI_ERROR(status)) {
        return status;
    }

    // Enroll MOK
    status = mok_protocol->EnrollKey();
    if (EFI_ERROR(status)) {
        return status;
    }

    return EFI_SUCCESS;
}
```

### Disk Encryption

**LUKS2 Configuration**:
- Algorithm: AES-XTS
- Key size: 512 bits
- PBKDF: Argon2id
- Memory cost: 1 GB
- Time cost: 5 iterations
- Parallelism: 4 threads

**Implementation**:
```c
// installer/encryption.c
#include <libcryptsetup.h>

int encrypt_device(const char *device, const char *passphrase) {
    struct crypt_device *cd;
    struct crypt_params_luks2 params = {
        .type = CRYPT_LUKS2,
        .hash = "sha256",
        .data_alignment = 0,
        .data_device = NULL,
    };

    int r = crypt_init(&cd, device);
    if (r < 0) return r;

    // Format as LUKS2
    r = crypt_format(cd, CRYPT_LUKS2, "aes", "xts-plain64",
                     "sha256", NULL, 512, &params);
    if (r < 0) {
        crypt_free(cd);
        return r;
    }

    // Configure Argon2id
    struct crypt_pbkdf_type pbkdf = {
        .type = CRYPT_PBKDF_ARGON2ID,
        .hash = "sha256",
        .time_ms = 5000,
        .max_memory_kb = 1024 * 1024,
        .parallel_threads = 4,
    };

    r = crypt_set_pbkdf_type(cd, &pbkdf);
    if (r < 0) {
        crypt_free(cd);
        return r;
    }

    // Add keyslot
    r = crypt_keyslot_add_by_passphrase(cd, CRYPT_ANY_KEYSLOT, NULL, 0,
                                       passphrase, strlen(passphrase));
    crypt_free(cd);
    return r;
}
```

### TPM2 Integration

**TPM2 Binding**:
- Bind encryption key to TPM2
- Automatic unlock on boot
- Fallback to passphrase

**Implementation**:
```c
// installer/tpm2.c
#include <tpm2-tss.h>

int bind_to_tpm2(const char *device, const char *passphrase) {
    TPM2_RC rc;
    TPM2_HANDLE handle;

    // Initialize TPM2
    rc = Tss2_Sys_Initialize();
    if (rc != TPM2_RC_SUCCESS) {
        return -1;
    }

    // Create primary key
    rc = Tss2_Sys_CreatePrimary(..., &handle);
    if (rc != TPM2_RC_SUCCESS) {
        return -1;
    }

    // Seal passphrase to TPM2
    rc = Tss2_Sys_Create(..., handle, passphrase, ...);
    if (rc != TPM2_RC_SUCCESS) {
        return -1;
    }

    return 0;
}
```

## Partitioning

### Default Partition Layout

```
/dev/sda1  EFI System Partition  512 MB  FAT32
/dev/sda2  Boot Partition       1 GB   ext4
/dev/sda3  System Partition     50 GB  Btrfs (encrypted)
/dev/sda4  Swap Partition       8 GB   swap
/dev/sda5  Data Partition       Remain Btrfs (encrypted)
```

### Filesystem Options

**Btrfs**:
- Compression: zstd
- Subvolumes: root, home, var, tmp
- Snapshots: Enabled
- RAID: Optional

**ZFS**:
- Compression: lz4
- Dataset: root, home, var, tmp
- Snapshots: Enabled
- ZIL: Separate device

## User Interface

### Graphical Installer

**Features**:
- Modern, responsive design
- Zenith toolkit widgets
- Dark mode support
- Accessibility features
- Multi-language support

**Screens**:
1. Welcome screen with language selection
2. Profile selection
3. Disk partitioning
4. Encryption setup
5. Secure Boot configuration
6. User creation
7. Installation progress
8. Completion

### Command-Line Installer

**Features**:
- Scripted installation
- Configuration file support
- Automated deployment
- Remote installation

**Usage**:
```bash
# Interactive installation
siginstall

# Automated installation
siginstall --config install.conf

# Remote installation
siginstall --ssh user@remote --config install.conf
```

**Configuration File**:
```toml
[installer]
profile = "desktop"
language = "en_US"
keyboard = "us"
timezone = "Asia/Kolkata"

[disk]
device = "/dev/sda"
partitioning = "auto"
filesystem = "btrfs"
encryption = true
passphrase = "secure123"

[user]
name = "sigmaos"
password = "password123"
fullname = "SigmaOS User"
autologin = false

[network]
hostname = "sigmaos-desktop"
dhcp = true
```

## Accessibility

### Screen Reader Support

**Features**:
- Orca integration
- Text-to-speech
- Braille display support
- Keyboard navigation

### Keyboard Navigation

**Features**:
- Full keyboard support
- Keyboard shortcuts
- Focus indicators
- High contrast mode

### Indic Language Support

**Supported Languages**:
- Hindi
- Bengali
- Tamil
- Telugu
- Marathi
- Gujarati
- Kannada
- Malayalam
- Punjabi
- Urdu

## Installation Process

### Pre-Installation Checks

1. **Hardware Compatibility**: Verify hardware compatibility
2. **Memory Check**: Verify sufficient RAM
3. **Disk Check**: Verify disk space
4. **Network Check**: Verify network connectivity
5. **Secure Boot Check**: Verify Secure Boot status

### Installation Steps

1. **Boot Installer**: Boot from USB/ISO
2. **Welcome Screen**: Select language and keyboard
3. **Profile Selection**: Choose installation profile
4. **Disk Setup**: Partition and format disk
5. **Encryption Setup**: Configure LUKS2 encryption
6. **Secure Boot Setup**: Configure Secure Boot
7. **User Setup**: Create user account
8. **Installation**: Install system files
9. **Configuration**: Configure system settings
10. **Reboot**: Reboot into installed system

### Post-Installation

1. **First Boot**: Boot into installed system
2. **Initial Setup**: Complete initial configuration
3. **Update System**: Install security updates
4. **Install Packages**: Install additional packages
5. **Configure Desktop**: Customize desktop environment

## Troubleshooting

### Installation Failures

**Common Issues**:
1. **Secure Boot Errors**: Disable Secure Boot temporarily
2. **Disk Errors**: Check disk health with SMART
3. **Memory Errors**: Run memory test
4. **Network Errors**: Verify network configuration

### Boot Issues

**Common Issues**:
1. **Boot Loop**: Check bootloader configuration
2. **Encryption Errors**: Verify passphrase
3. **Driver Issues**: Boot in safe mode

## Best Practices

### Development

1. **Modular Design**: Keep installer modules independent
2. **Error Handling**: Comprehensive error handling
3. **Logging**: Detailed logging for debugging
4. **Testing**: Test on various hardware configurations

### Security

1. **Secure Defaults**: Enable security by default
2. **Verification**: Verify all downloads and signatures
3. **Encryption**: Encrypt sensitive data
4. **Audit**: Regular security audits

### User Experience

1. **Clear Feedback**: Provide clear progress feedback
2. **Recovery**: Support installation rollback
3. **Accessibility**: Ensure accessibility features
4. **Localization**: Full localization support

## Roadmap & Milestones

### Phase 1 (Months 0-3)
- Console-based interactive install script
- ext4/btrfs filesystem support
- Basic partitioning
- User creation

### Phase 2 (Months 3-6)
- LUKS2 volume configuration
- Argon2id keyslots
- TPM2 binding
- Secure Boot detection

### Phase 3 (Months 6-9)
- GUI installer front-end
- Zenith toolkit widgets
- Accessibility features
- Indic language support

### Phase 4 (Months 9-12)
- MOK Secure Boot configuration
- Automated installation
- Remote installation
- Installation rollback

## References

- [Calamares Installer](https://calamares.io/)
- [LUKS2 Documentation](https://gitlab.com/cryptsetup/cryptsetup/-/wikis/LUKS2)
- [TPM2 Specification](https://trustedcomputinggroup.org/work-groups/tcg-tpm-working-group/)
- [UEFI Secure Boot](https://uefi.org/specifications)
