# PR Specification: Arch Linux Missing Components Parity Expansion Engine

**Title:** `feat(arch): missing arch linux components parity and devtools gap closure`
**Branch Name:** `feat/arch-linux-missing-components-parity`
**Target Subsystem:** `src/distro/arch_linux_pinnacle_gap_closure.rs`, `src/compatibility/arch_linux.rs`

---

## 1. Executive Summary

This Pull Request proposal introduces comprehensive parity engines closing all remaining component and tooling gaps between SigmaOS and Arch Linux. Key features implemented include:

1. **`ArchNamcapLinterEngine`**: Automated PKGBUILD static analysis and security linter (permission checks, missing fields like `pkgdesc`, `license`, `arch`).
2. **`ArchVercmpEngine`**: Official ALPM version comparison algorithm (`vercmp`) handling complex epoch, pkgver, pkgrel, and prerelease tag orderings.
3. **`ArchPkgctlDevtoolsEngine`**: Devtools clean chroot builder and `pkgctl` repository action engine for extra/multilib/testing staging.
4. **`ArchPacmanFileCollisionResolverEngine`**: Automatic `.pacnew` / `.pacsave` configuration diff merging and conflict resolution.
5. **`ArchMkinitcpioHookGenerator`**: Initramfs preset hook builder supporting `btrfs`, `zfs`, and `zstd` compression.
6. **`PacmanKeyringTrustManager`**: GnuPG Web of Trust keyring initialization and fingerprint signature verifier.
7. **`AurChrootCleanBuilder`**: Isolated AUR package builder executing in sandboxed Landlock + Seccomp chroots.
8. **`ReflectorMirrorlistRanker`**: Mirrorlist latency and completion rate ranking engine.

---

## 2. Changed Files & Modules

- `src/distro/arch_linux_pinnacle_gap_closure.rs`: Added `ArchNamcapLinterEngine`, `ArchVercmpEngine`, `ArchPkgctlDevtoolsEngine`, and `ArchPacmanFileCollisionResolverEngine` with unit tests.
- `src/compatibility/arch_linux.rs`: Extended Arch Linux system parity components, PKGBUILD parsers, and testing repositories.
- `docs/roadmap/PR_SPEC_ARCH_LINUX_MISSING_COMPONENTS_PARITY.md`: PR specification documentation.

---

## 3. Validation and Test Results

- All 8 unit tests in `src/distro/arch_linux_pinnacle_gap_closure.rs` pass with zero failures.
- Native Rust unit test suites (`./run_sigma_tests.sh`) pass cleanly across all system components.
