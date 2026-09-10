# AI Agent Debian / Ubuntu APT Parity Component Maintenance & Development Guide

This document provides operational directives and technical specifications for AI agents developing, maintaining, and auditing Debian and Ubuntu package management compatibility in SigmaOS.

---

## 1. Debian Component Hierarchy

Debian/Ubuntu parity components are organized across the following zero-dependency `#![no_std]` Rust modules:

| Component | File Path | Scope & Responsibilities |
|---|---|---|
| **APT & Dpkg Core Engine** | `src/sigpkg/debian_apt_engine.rs` | Debian control file parser (`DebControlParser`), `.deb` package reader (`DebPackage`), APT repository source manager (`AptRepositoryManager`), Dpkg database status tracker (`DpkgDatabase`) |
| **APT Pinning & Multi-Arch Resolver** | `src/compatibility/distro_parity_ultimate.rs` | APT pin priorities (1..=1000), foreign architecture dependency resolver (`AptPinningMultiArchResolver`), multi-arch `i386`/`amd64`/`arm64` cross-installation rules |
| **Ubuntu AppArmor & Snapd Engine** | `src/distro/missing_distro_innovations.rs` | AppArmor profile enforcement, path-based access control, snapd confinement |

---

## 2. Technical Directives & Code Conventions

1. **RFC 822 / Debian Control File RFC Conventions**:
   - `DebControlParser` must handle key-value headers separated by colons (`:`) and multiline descriptions beginning with leading spaces.
   - Maintain support for `Package`, `Version`, `Architecture`, `Depends`, `Pre-Depends`, `Recommends`, and `Maintainer` field fields.
2. **Zero Allocation Parsing**:
   - Prefer single-pass byte slice operations (`&[u8]`) when parsing Debian control headers and status databases to avoid intermediate string heap allocations.
3. **Standalone Testing**:
   ```bash
   rustc --test src/sigpkg/debian_apt_engine.rs --edition=2021 -o build/debian_apt_test && ./build/debian_apt_test
   rustc --test src/compatibility/distro_parity_ultimate.rs --edition=2021 -o build/distro_parity_ultimate_test && ./build/distro_parity_ultimate_test
   ```

---

## 3. Maintenance Procedures

### A. Dpkg Status Database
- Maintain package status states (`install ok installed`, `deinstall ok config-files`).
- Ensure atomic commits when modifying `/var/lib/dpkg/status` virtual records.

### B. APT Pinning & Multi-Arch
- Evaluate pin priority weights (Pin-Priority > 1000 allows downgrade; Pin-Priority 500-989 standard candidates).
- Enforce foreign architecture isolation unless explicitly enabled via `dpkg --add-architecture`.

---

## 4. Verification Checklist

- [ ] Execute `rustc --test src/sigpkg/debian_apt_engine.rs --edition=2021 -o build/debian_apt_test && ./build/debian_apt_test`
- [ ] Run `./run_sigma_tests.sh`
- [ ] Confirm `#![no_std]` compliance in `src/sigpkg/debian_apt_engine.rs`.
