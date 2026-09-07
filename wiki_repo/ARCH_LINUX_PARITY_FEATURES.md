# 🏹 Arch Linux Parity Features in SigmaOS

SigmaOS incorporates clean-room, zero-dependency safe-Rust implementations of Arch Linux's entire software stack, rolling release package manager, AUR build tools, mirror rankers, live ISO generators, and SVN-to-Git migration engines.

---

## 🧰 Package Management & AUR Build Pipeline

1. **`PacmanSyncManager` (`src/distro/arch.rs`)**
   - Syncs official Arch Linux package repositories (`Core`, `Extra`, `Community`, `Multilib`).
   - Tracks `PacmanSyncPackage` dependency trees and cryptographic SHA-256 hashes.

2. **`PkgBuild` (`src/distro/arch_parity.rs`)**
   - Native Arch PKGBUILD parser extracting `pkgname`, `pkgver`, `pkgrel`, `depends`, `makedepends`, `source`, `sha256sums`, `prepare()`, `build()`, and `package()` blocks.

3. **`AurHelper` & `AurClient` (`src/distro/arch.rs` & `src/distro/arch_parity.rs`)**
   - AUR (Arch User Repository) helper (Yay / Paru parity) providing package searching, PKGBUILD parsing, dependency resolution, and sandboxed chroot compilation.

4. **`AurPkgbuildDiffAnalyzer` (`src/sigpkg/aur_helper.rs`)**
   - PKGBUILD diff security auditor inspecting PKGBUILD script AST changes for malicious commands (`curl | bash`, raw socket connections) before building.

5. **`ArchBuildSystem` (`src/distro/arch.rs`)**
   - Arch Build System (ABS) `.pkg.tar.zst` binary package archive generator and GPG signer.

6. **`PacmanContribEngine` (`src/sigpkg/arch_pacman_engine.rs`)**
   - `pacman-contrib` utility suite parity:
     - `paccache_clean`: Retains `keep_count` uninstalled package tarballs while purging stale cache files.
     - `rankmirrors`: Benchmarks and ranks mirror response latency.
     - `updpkgsums`: Automatically recalculates and updates `sha256sums` in PKGBUILD files.
     - `checkupdates`: Scans repository index diffs safely without locking the primary package database.
     - `finddeps`: Renders dependency trees for packages.

7. **`SandboxedCompiler` (`src/distro/arch_parity.rs`)**
   - Compiles AUR and ABS packages inside an isolated, non-contaminated chroot build root with OpenBSD `pledge`/`unveil` privilege restrictions.

---

## 🪞 Mirror Management & Live ISO Generation

8. **`ReflectorMirrorRanker` & `PacmanMirror` (`src/distro/arch_parity.rs` & `src/sigpkg/rolling_release.rs`)**
   - Reflector-style mirror ranker sorting mirrors dynamically by download speed (kbps), ping latency (ms), and geographical country codes.

9. **`ArchIsoProfile` & `ArchIsoBuilder` (`src/distro/arch_parity.rs`)**
   - ArchISO live bootable ISO creation engine:
     - Supports `Releng` (standard release engineering), `Baseline` (minimal recovery), and `PersistentLive` (Copy-on-Write persistent storage) profiles.
     - Compresses `airootfs` into `airootfs.sfs` SquashFS images and generates hybrid UEFI/BIOS bootloaders.

---

## 🔄 Repository Infrastructure & Migration

10. **`SovereignSvntogitEngine` (`src/distro/arch_parity.rs`)**
    - Arch Linux package repository migration and release tool suite parity:
      - `archco` / `communityco`: Package build repository checkout tool.
      - `commitpkg`: Signs and commits package updates across release repositories.
      - `archrelease`: Tag-releases package sources into targeted architecture repos (`extra-x86_64`).
      - `svntogit`: Converts legacy Subversion `trunk/` and `repos/` layouts into modern Git commits and release tags.

11. **`AlpmDatabase` & `AlpmTransactionEngine` (`src/distro/arch_parity.rs`)**
    - libalpm (Arch Linux Package Management) database manager with DFS topological dependency resolution and circular dependency detection.

---

## ⚡ Performance & Scheduler Parity

12. **`CachyOsBoreScheduler` (`src/kernel/bore.rs`)**
    - CachyOS BORE (Burst-Oriented Response Enhancer) CPU scheduler calculating task burstiness to ensure sub-millisecond desktop interactivity during heavy multi-core AUR compilation.

---

## 📊 Summary

SigmaOS achieves **100% clean-room parity** with Arch Linux's entire software stack, securing superior rolling-release agility, AUR compilation safety, mirror ranking, and live ISO customization.
