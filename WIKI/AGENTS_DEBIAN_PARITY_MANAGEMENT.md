# AI Agent Debian Parity Management Specification for SigmaOS

This document provides guidelines and architectural specifications for AI agents maintaining and developing Debian Linux compatibility components within **SigmaOS**.

---

## 1. Overview & Debian Parity Subsystem

SigmaOS implements a clean-room, zero-external-dependency Debian parity subsystem across `src/compatibility/debian.rs`, `src/distro/debian_parity.rs`, `src/package/debian.rs`, and `src/sigpkg/debian_apt_engine.rs`.

Key components managed by AI agents:

1. **APT Repository Synchronization & Keyring Verification (`AptRepositorySync`)**:
   - Manages channel sources (`Stable`, `Testing`, `UnstableSid`), mirror URLs, and GPG release keyring verification (`verify_release_keyring`).
2. **Debconf Preconfiguration & Preseed Answer Engine (`DebconfEngine`)**:
   - Parses debconf template definitions and automated preseed configuration files (`parse_preseed_file`) with question types (`String`, `Select`, `Multiselect`, `Boolean`, `Password`, `Note`, `Error`).
3. **dpkg-statoverride File Ownership & Mode Overrides (`DpkgStatOverrideEngine`)**:
   - Parses `/var/lib/dpkg/statoverride` lines (`user group mode path`) to enforce custom file permissions during package deployment (`add_override`, `get_override`).
4. **APT Pinning & Release Preferences Engine (`AptPinningEngine`)**:
   - Evaluates `/etc/apt/preferences` pin priority rules (`Package`, `Pin: release`, `Pin-Priority`) to resolve candidate package version selection (`get_pin_priority`).
5. **APT Build-Dep Resolver Engine (`AptBuildDepResolver`)**:
   - Parses Debian source package `.dsc` and `debian/control` `Build-Depends:` fields to extract build-time dependencies.
6. **SysVinit Service Management (`SysVInitEngine`)**:
   - Simulates SysV runlevel transitions (Runlevels 0–6: `Halt`, `SingleUser`, `MultiUserConsole`, `MultiUserDefault`, `MultiUserX11`, `MultiUserFull`, `Reboot`).
7. **Debian Alternatives Link Management (`DebianAlternativesSystem`)**:
   - Implements `update-alternatives` parity with priority-based automatic resolution and manual target selection.

---

## 2. Rules for AI Agents Developing Debian Parity Modules

1. **Zero External Dependencies**:
   - All parsers, engines, and data models must use standard Rust or `klib` primitives. Do not add external crates to `Cargo.toml`.
2. **Strict Preseed & Statoverride Parsing**:
   - Always handle missing or malformed lines gracefully without panicking. Ignore comment lines starting with `#`.
3. **Pin Priority Semantics**:
   - Default APT pin priority is `500`. Higher pin priorities (e.g. `900` for testing, `1001` for target overrides) supersede default channel selections.
4. **Testing Matrix**:
   - Every new Debian parity structure or method must include a corresponding unit test in `src/compatibility/debian.rs`.

---

## 3. Verification Commands

AI agents must verify Debian compatibility changes using:

```bash
# Run standalone Debian unit test suite
rustc --test src/compatibility/debian.rs --edition=2021 -o build/test_debian && ./build/test_debian
```
