# 📦 SIGMAOS UNIVERSAL PACKAGE MANAGER PULL REQUEST WORKFLOW SPECIFICATION

> **Target Repository:** [https://github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
> **Document Version:** 1.0.0
> **Subsystem Focus:** `src/package/sovereign_universal_pm_pr_bridge.rs` & `src/package/sovereign_pr_package_gateway.rs`
> **Status:** Active Master Specification & Strategic Execution Roadmap

---

## 🛠️ EXECUTIVE SUMMARY

SigmaOS introduces a **Universal Package Manager Pull Request Workflow Architecture** inspired by the premier package management systems of Linux and BSD distributions:
* **Arch Linux / AUR & ALPM:** `PKGBUILD` format, ALPM hooks, pacman sync DBs.
* **Debian / Ubuntu APT:** `.deb` control files, dpkg triggers, debconf configuration databases.
* **Fedora / RHEL DNF:** `.rpm` specs, libdnf5 transaction solver, rpm-ostree atomic commits.
* **Alpine Linux APK:** `APKBUILD`, apk-tools v3, volatile overlay mounts.
* **Void Linux XBPS:** Source templates, `xbps-src` chroot builds, runit service triggers.
* **Gentoo Linux Portage:** `.ebuild` scripts, `USE` flags, slot operator dependency matching (`:=`).
* **FreeBSD / OpenBSD / NetBSD:** FreeBSD `pkg` (+ VNET Jails), OpenBSD `pkg_add` (+ pledge/unveil sandboxing), NetBSD `pkgsrc` bulk builds.
* **NixOS / GNU Guix:** Hermetic functional package expressions, reproducible build closures, Content-Addressable Store (CAS).
* **Flatpak / Snap / AppImage:** Desktop app sandboxing portals, cgroup containment, squashfs image mounting.

Under this architecture, **every foreign package format is converted and submitted as a standardized Pull Request manifest** into the SigmaOS package registry, enabling automated CI testing, post-quantum signature verification, SAT dependency resolution, and atomic Btrfs/ZFS system deployment.

---

## 🏛️ ARCHITECTURE OVERVIEW

```
                  +-------------------------------------------------+
                  |      Foreign Package Formats (Linux & BSD)      |
                  +-------------------------------------------------+
                  | APT, Pacman, DNF, APK, XBPS, Portage, FreeBSD,   |
                  | OpenBSD, Pkgsrc, Nix, Guix, Flatpak, Snap, etc. |
                  +-------------------------------------------------+
                                           |
                                           v
                  +-------------------------------------------------+
                  |  Sovereign Universal Package Format Converter   |
                  |  (`src/package/sovereign_universal_pm_pr...`)   |
                  +-------------------------------------------------+
                                           |
                                           v
                  +-------------------------------------------------+
                  |      Automated Pull Request Workflow Engine     |
                  |    - Standardized PR Manifest Generation        |
                  |    - Boolean SAT Dependency Resolution          |
                  |    - Dilithium-5 Post-Quantum Signature Check |
                  |    - Hermetic Chroot Sandbox Build Verification  |
                  +-------------------------------------------------+
                                           |
                                           v
                  +-------------------------------------------------+
                  |    SigmaOS Package Registry (`sigma-pkg`)      |
                  |    - Atomic Btrfs/ZFS Snapshot Rollback        |
                  |    - Sub-50ms Package Installation              |
                  +-------------------------------------------------+
```

---

## 🌐 1. SUPPORTED PACKAGE FORMATS & CONVERSION MATRIX

| Distro / Ecosystem | Native Format | Extraction / Conversion Parser | SigmaOS Target Subsystem |
| :--- | :--- | :--- | :--- |
| **Arch Linux** | `.pkg.tar.zst` / `PKGBUILD` | `ArchPacmanEngine` & ALPM hook parser | `src/sigpkg/arch_pacman_engine.rs` |
| **Debian / Ubuntu** | `.deb` / Control files | `DebianAptEngine` & dpkg-deb extractor | `src/sigpkg/debian_apt_engine.rs` |
| **Fedora / RHEL** | `.rpm` / Spec files | `FedoraRpmEngine` & cpio archive reader | `src/sigpkg/fedora_rpm_engine.rs` |
| **Alpine Linux** | `.apk` / `APKBUILD` | `AlpineApkEngine` & tar.gz reader | `src/sigpkg/alpine_apk_engine.rs` |
| **Void Linux** | `.xbps` / Templates | `PoudriereXbpsEngine` & xbps header parser | `src/sigpkg/poudriere_xbps.rs` |
| **Gentoo Linux** | `.ebuild` / `USE` flags | `GentooUseFlagsEngine` & bash ebuild runner | `src/sigpkg/gentoo_use_flags.rs` |
| **FreeBSD** | `.pkg` / Manifest JSON | `FreeBsdPkgAdapter` & zstd extractor | `src/package/universal.rs` |
| **OpenBSD** | `.tgz` / `+CONTENTS` | `OpenBsdPkgAdapter` + Pledge/Unveil sandbox | `src/package/universal.rs` |
| **NetBSD** | `pkgsrc` / bmake | `NetBsdPkgsrcAdapter` | `src/package/universal.rs` |
| **NixOS / Guix** | `.nix` / `.scm` | `NixDslEngine` & Guix Scheme evaluator | `src/sigpkg/nix_dsl.rs` |
| **Flatpak / Snap** | `.flatpak` / `.snap` | `FlatpakSandboxBridge` & SquashFS loader | `src/package/universal.rs` |

---

## 🔄 2. PULL REQUEST WORKFLOW SPECIFICATION

### Step 1: Package Ingestion & PR Manifest Submission
A developer or automated bot submits a Pull Request containing a package definition file (`package.pr.toml` or `PKGBUILD`/`Control`/`Spec`).

```toml
# Example Universal Package PR Manifest (package.pr.toml)
[package]
name = "neovim"
version = "0.10.0"
source_format = "PacmanPkg"
upstream_url = "https://github.com/neovim/neovim"
license = "Apache-2.0"

[dependencies]
runtime = ["libuv>=1.48.0", "msgpack-c>=6.0.0", "unibilium>=2.1.1"]
build = ["cmake", "ninja", "gettext", "curl"]

[security]
pq_signature = "dilithium5:8f3a9e..."
sandbox_isolation = "PledgeUnveil"

[build_pipeline]
steps = [
    "cmake -B build -G Ninja -DCMAKE_BUILD_TYPE=Release",
    "cmake --build build",
    "DESTDIR=$SIGMA_BUILD_ROOT cmake --install build"
]
```

### Step 2: Automated CI Validation Pipeline
When a Pull Request is opened or updated, the `sovereign_pr_package_gateway.rs` engine triggers the following validation stages:
1. **Format Parser Verification:** Validates the native syntax of the imported format (`.deb`, `.rpm`, `PKGBUILD`, `.ebuild`, `.nix`).
2. **SAT Constraint Solver:** Runs `dpll_solver.rs` to ensure the package dependencies can be satisfied across all active SigmaOS release channels.
3. **Hermetic Sandbox Build:** Executes the build pipeline in a zero-network, read-only chroot container guarded by OpenBSD Pledge and Unveil permissions.
4. **Binary Diffing & Reproducibility:** Uses `difftastic` AST diffing to compare the generated artifact against reproducible build hashes.
5. **Post-Quantum Signature Verification:** Verifies the cryptographic Dilithium-5 / Falcon-512 signatures accompanying the commit.

### Step 3: Atomic Merge & Btrfs/ZFS Snapshot Rollback
Upon PR merge:
1. A Btrfs/ZFS subvolume snapshot is created (`@pre-package-pr-install`).
2. The package artifacts are merged into the Content-Addressable Store (`/sigma/store/`).
3. System symlinks are atomically updated in under 50 milliseconds.
4. If health checks fail during post-install execution, the system automatically rolls back to the pre-install snapshot.

---

## 🛠️ VERIFICATION & IMPLEMENTATION SUMMARY

The zero-dependency implementation of this architecture is complete and re-exported in `src/package/mod.rs` and `src/sigpkg/mod.rs`:
* `src/package/sovereign_universal_pm_pr_bridge.rs`
* `src/package/sovereign_pr_package_gateway.rs`
* `src/sigpkg/universal_oop_system.rs`

All standalone unit tests pass 100%:
```bash
cargo check --lib
./run_sigma_tests.sh
pytest tests/
```

*End of Specification.*
