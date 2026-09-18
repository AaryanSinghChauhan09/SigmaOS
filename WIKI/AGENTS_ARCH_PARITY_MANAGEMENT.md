# AI Agent Arch Parity Management Specification for SigmaOS

This document provides guidelines and architectural specifications for AI agents maintaining and developing Arch Linux compatibility components within **SigmaOS**.

---

## 1. Overview & Arch Parity Subsystem

SigmaOS implements a clean-room, zero-external-dependency Arch Linux parity subsystem across `src/distro/arch.rs`, `src/distro/arch_parity.rs`, `src/distro/arch_aur.rs`, and `src/sigpkg/arch_pacman_engine.rs`.

Key components managed by AI agents:

1. **ALPM Pacman Hook Transaction Management (`ArchPacmanHookManager`)**:
   - Parses `.hook` definitions in `/etc/pacman.d/hooks/` with `[Trigger]` (`Operation`, `Type`, `Target`) and `[Action]` (`Description`, `When`, `Exec`, `NeedsTargets`) sections.
2. **Archinstall Automated Declarative Provisioner (`ArchinstallEngine`)**:
   - Parses JSON/TOML configuration specifications for disk partitioning (Btrfs, Ext4), bootloader selection (systemd-boot, GRUB), mirror ranking, and desktop installation.
3. **Reflector Mirrorlist Ranking Engine (`ArchReflectorEngine`)**:
   - Evaluates speed, latency, completion rate, and sync age of Arch Linux package mirrors, outputting ranked `/etc/pacman.d/mirrorlist` files.
4. **Pacman WKD Keyring Signature Verification (`ArchKeyringEngine`)**:
   - Manages Web Key Directory (WKD) PGP public keys and verifies package tarball signatures (`.sig`) prior to ALPM transaction commitment.
5. **AUR Package Helper & Build Sandbox (`ArchAurHelper`)**:
   - Downloads `PKGBUILD` specifications from AUR, executes dependency graph resolution, and builds packages inside unprivileged user capability zones (`chroot` / `unveil`).

---

## 2. Rules for AI Agents Developing Arch Parity Modules

1. **Zero External Dependencies**:
   - All parsers, ALPM hook engines, and keyring checkers must use safe Rust or `klib` primitives.
2. **PKGBUILD & Hook Compliance**:
   - PKGBUILD variables (`pkgname`, `pkgver`, `pkgrel`, `depends`, `makedepends`, `arch`) and ALPM hook triggers must be handled cleanly.
3. **Rolling Release Consistency**:
   - Pacman transactions must maintain database consistency (`/var/lib/pacman/local/`) and respect explicit package ignores (`IgnorePkg`, `IgnoreGroup`).

---

## 3. Verification Commands

AI agents must verify Arch Linux compatibility changes using:

```bash
rustc --edition=2021 --crate-type=lib src/lib.rs -o libsigmaos.rlib
```
