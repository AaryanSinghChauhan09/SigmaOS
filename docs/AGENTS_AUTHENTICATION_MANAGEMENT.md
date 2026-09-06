# AI Agent Authentication Management Specification for SigmaOS

This document provides specifications and guidelines for AI agents developing, managing, and auditing user authentication, identity records, and access control across **SigmaOS**.

---

## 1. Authentication Architecture & PAM Stack Integration

SigmaOS implements a zero-dependency, modular Linux PAM (Pluggable Authentication Modules) authentication engine in `src/compatibility/linux_distro_parity.rs` (`LinuxPamAuthenticationEngine`):

- **PAM Service Chains**: Supports `auth`, `account`, `session`, and `password` management modules.
- **Control Flags**: Evaluates `required`, `requisite`, `sufficient`, and `optional` rules in service configuration chains (e.g. `/etc/pam.d/login`, `/etc/pam.d/sudo`).

---

## 2. Portable Encrypted Home Directories (`systemd-homed` Parity)

Systemd-homed parity is implemented in `src/auth/systemd_homed.rs` (`SovereignSystemdHomedEngine`):

1. **Storage Backends**:
   - `LuksLoop`: Portable LUKS-encrypted loopback file (`/home/$USER.home`).
   - `Fscrypt`: Ext4/Btrfs native file system encryption.
   - `Directory` / `BtrfsSubvolume` / `Cifs`.

2. **JSON Identity Records (`~/.identity`)**:
   - Self-contained, cryptographically signed user records (`HomedUserRecord`).
   - Supports auto-mounting on login, auto-locking on suspend, and disk space quota resizing.

---

## 3. Biometric & Post-Quantum Authentication

1. **Biometric Authentication (`SigmaBio`)**:
   - Implemented in `src/futuristic_modules.rs` (`SigmaBio`).
   - Authenticates users using multi-modal biometric signals (heartbeat, gait analysis, and neural data streams).

2. **Dual-Layer PQC Verification (`GpgPqcVerifierAdapter`)**:
   - Implemented in `src/sigpkg/universal_oop_system.rs`.
   - Combines classical GPG signatures with post-quantum Dilithium-5 signatures for package maintainer and identity verification.

---

## 4. Testing & Verification Commands

```bash
# Run systemd-homed standalone test
rustc --test --edition=2021 src/auth/systemd_homed.rs -o build/test_homed && ./build/test_homed

# Run full test runner
./run_sigma_tests.sh
```
