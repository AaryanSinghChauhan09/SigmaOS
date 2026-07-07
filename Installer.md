# SigmaOS Installer Specification

## Overview
The SigmaOS Installer (`siginstall`) provides a polished graphical and command-line interface based on Calamares-style module flows. To ensure robust security from initial setup, the installer mandates UEFI Secure Boot configuration and home directory encryption (via LUKS2 and Argon2id) by default.

## Installation Architecture
```
 [Welcome Page] ──► [Select Profile] ──► [Partition Drive]
                                                 │
                                                 ▼
 [Install System Files] ◄── [Set LUKS Key] ◄── [Secure Boot Setup]
         │
         ▼
 [Reboot into Clean System]
```

## System Properties & Requirements
- **Secure Boot**: Requires active UEFI Secure Boot. The installer configures local Machine Owner Keys (MOK).
- **Encryption**: Users set a single passphrase that sets up LUKS2 for root and decrypts the home volume using systemd-homed style credentials.
- **Partitioning**: Formats drives using Btrfs or ZFS to support native snapshot workflows.

## Technical Implementation
The partitioning and encryption commands are executed through a memory-safe C module wrapping standard LUKS2 system calls.

```c
// installer/sigma_installer.c
#include <libcryptsetup.h>

int encrypt_device(const char *device, const char *passphrase) {
    struct crypt_device *cd;
    int r = crypt_init(&cd, device);
    if (r < 0) return r;

    r = crypt_format(cd, CRYPT_LUKS2, "aes", "xts-plain64", NULL, NULL, 512, NULL);
    if (r >= 0) {
        r = crypt_keyslot_add_by_passphrase(cd, CRYPT_ANY_KEYSLOT, NULL, 0, passphrase, strlen(passphrase));
    }
    crypt_free(cd);
    return r;
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Console-based interactive install script writing to ext4/btrfs.
- **Phase 2 (Months 3-6)**: LUKS2 volume configuration with Argon2id keyslots and TPM2 binding.
- **Phase 3 (Months 6-9)**: GUI installer front-end using Zenith toolkit widgets.
- **Phase 4 (Months 9-12)**: MOK Secure Boot configuration automation.
