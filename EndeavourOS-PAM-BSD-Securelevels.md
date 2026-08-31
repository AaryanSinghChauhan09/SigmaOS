# EndeavourOS Parity, Linux PAM & BSD Securelevels

This page documents SigmaOS's compatibility and parity with EndeavourOS, the Linux PAM authentication framework, and BSD securelevels — all merged in the `improve-security-and-access-control` series of branches.

---

## 1. EndeavourOS Compatibility Parity

EndeavourOS is a rolling-release, Arch-based distro focused on minimalism and community. SigmaOS achieves feature parity in the following areas:

### Package Management
- `sigpkg install <pkg>` mirrors `yay`/`paru` AUR-style package resolution
- Rolling release channel available via `sigma-rolling.toml`
- AUR PKGBUILD import tool: `sigpkg import-pkgbuild <url>`

### System Configuration
- `sigma-ctl` mirrors `systemctl` for service management
- Hook-based `.hook` files (analogous to pacman hooks) run on package events
- Auto-detection of hardware via udev-compatible event bus

### Desktop Environment Support
- KDE Plasma, XFCE, i3, Sway, Hyprland window managers supported
- Theme/wallpaper sync via `sigma-theme` CLI
- Display manager: Sigma Zenith Login (analogous to SDDM)

### EndeavourOS Welcome App Equivalent
- `sigma-welcome` app for first-boot configuration
- Hardware wizard, driver selection, locale & keyboard setup
- Online/offline installer modes

---

## 2. Linux PAM (Pluggable Authentication Modules)

SigmaOS implements a **PAM-compatible authentication stack** built in pure Rust without linking to `libpam`. This reduces external C library dependency while maintaining API-level compatibility.

### Architecture

```
Login / sudo / lock-screen
         │
         ▼
  SigmaPAM Dispatcher
         │
    ┌────┴─────────────────────────────┐
    │                                  │
 pam_sigma_unix          pam_sigma_biometric
 (password/shadow)       (fingerprint/FaceID)
    │                                  │
 pam_sigma_totp          pam_sigma_smartcard
 (TOTP/HOTP 2FA)         (YubiKey/FIDO2)
    └────────────────────────────────┘
         │
    PAM Result (Success / Failure)
```

### PAM Stack Configuration

`/etc/sigma/pam.d/login`:
```
auth    required    pam_sigma_unix.so
auth    optional    pam_sigma_biometric.so try_first_pass
auth    optional    pam_sigma_totp.so
account required    pam_sigma_unix.so
session required    pam_sigma_unix.so
```

### Key Modules

| Module | Equivalent Linux Module | Description |
|--------|------------------------|-------------|
| `pam_sigma_unix` | `pam_unix` | Password/shadow file auth |
| `pam_sigma_biometric` | `pam_fprintd` | Fingerprint + FaceID |
| `pam_sigma_totp` | `pam_google_authenticator` | TOTP/HOTP 2FA |
| `pam_sigma_smartcard` | `pam_pkcs11` | YubiKey, FIDO2 |
| `pam_sigma_env` | `pam_env` | Environment variable setup |
| `pam_sigma_limits` | `pam_limits` | Resource limits (ulimit) |

### Password Storage

Passwords are stored using **Argon2id** key derivation, not crypt/bcrypt:
- Memory cost: 64MB
- Iterations: 3
- Parallelism: 4
- Salt: 16 bytes random, stored alongside hash

---

## 3. BSD Securelevels

BSD securelevels provide a **kernel-enforced security hardening mechanism** that progressively restricts system capabilities as the system state escalates. SigmaOS implements a BSD-compatible securelevel system.

### Securelevel Table

| Level | Name | Restrictions |
|-------|------|--------------|
| -1 | Disabled | No restrictions (factory/recovery mode) |
| 0 | Permissive | Default multi-user mode |
| 1 | Secure | No raw disk writes, no kernel module loading, immutable flags enforced |
| 2 | Highly Secure | All level-1 + firewall rules immutable, no sysctl writes |
| 3 | Network Secure | All level-2 + no network interface changes |

### Activation

```bash
# Query current securelevel
sigma-ctl securelevel get

# Raise to secure mode (can never lower without reboot)
sigma-ctl securelevel set 1

# Set in sigma.toml for persistent configuration
echo 'securelevel = 2' >> /etc/sigma/sigma.toml
```

### Kernel Implementation

```rust
use crate::security::securelevels::{SecureLevel, KernelSecurityState};

let mut state = KernelSecurityState::new();
state.raise_to(SecureLevel::Secure)?;

// This will now fail:
state.load_kernel_module("evil.ko")?;  // Err(SecureLevelViolation)
```

### Integration with PAM

When the system securelevel is ≥ 2, PAM modules automatically require:
- Multi-factor authentication for root
- All login events logged to immutable audit log
- No `pam_exec` or shell-spawning modules permitted

---

## See Also

- [Security Architecture](Security-Architecture.md)
- [Arch Linux and AUR Parity](Arch-Linux-and-AUR-Parity.md)
- [Authentication & Identity](Authentication.md)
- [eBPF, Splice, and Landlock](eBPF-Splice-Landlock-Unveil.md)
