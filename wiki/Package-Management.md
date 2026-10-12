# Package Management & Distribution Adapters

The **SigmaOS Package Management Subsystem** (`sigpkg`) provides a sovereign, unified universal package management architecture. Rather than requiring users to re-package the world or suffer ecosystem isolation, SigmaOS implements native parsing, resolution, and transaction engines for the major Linux and BSD distribution packaging formats.

---

## 1. Subsystem Architecture

### 1.1 Sovereign Universal Package Format (`.sigpkg`)
- **Location**: `src/package/sovereign_universal_package_format_master.rs`, `src/sigpkg/`
- **Specification**: Content-addressable, cryptographically signed package container. Includes deterministic metadata, dependency DAG manifests, zero-copy payload streaming, and transaction journals for instantaneous atomic commit or rollback.

### 1.2 Arch Linux ALPM & `PKGBUILD` Parity
- **Location**: `src/sigpkg/pacman.rs`, `src/sigpkg/arch_compat.rs`
- **Specification**: Full compatibility with Arch Linux ALPM database format (`/var/lib/pacman/local/`), `PKGBUILD` variable and function stanza parsing (`pkgname`, `pkgver`, `pkgrel`, `arch`, `depends`, `makedepends`, `checkdepends`), and pre/post transaction hook execution.

### 1.3 Debian & Ubuntu Parity (`dpkg` / `APT`)
- **Location**: `src/sigpkg/debian_dpkg_engine.rs`, `src/sigpkg/multi_distro.rs`
- **Features**:
  - `dpkg-divert`: File redirection engine allowing conflicting packages to install alternative binary paths without overwriting existing system files.
  - `dpkg-trigger`: Deferred event notification framework for post-transaction execution (e.g., ldconfig cache rebuild, font caches, desktop database updates).
  - `debconf`: Interactive and preseeded configuration answer engine.
  - `deb-control`: RFC 822 formatted control stanza tokenizer and parser.

### 1.4 Fedora & RHEL Parity (`RPM` / `OSTree`)
- **Location**: `src/sigpkg/fedora_rpm_engine.rs`, `src/remote/fedora_remote.rs`
- **Features**:
  - RPM `.spec` stanza parser and payload extractor.
  - RPM-OSTree immutable deployment state engine with atomic rollback trees.
  - Active Directory & FreeIPA realm join client.

### 1.5 Void, Alpine, Gentoo & NixOS Multi-Distro Engines
- **Location**: `src/package/sovereign_distro_package_advancements_v7.rs`, `src/package/sovereign_distro_package_advancements_v8.rs`
- **Void Linux `xbps`**: Fast transaction graph resolution and dictionary-based package index parsing.
- **Alpine Linux `apk`**: Lightweight cryptographic signature and repository catalog verification.
- **Gentoo `ebuild`**: Source-based dependency DAG solver with dynamic USE-flag conditional resolution.
- **NixOS Flakes**: Declarative dependency locking and reproducible generation derivations.

---

## 2. Parity Comparison Matrix

| Distribution System | Format | Native Engine in SigmaOS | Transaction Semantics |
|---|---|---|---|
| **SigmaOS** | `.sigpkg` | `SovereignUniversalPackageMaster` | Full ACID Journaled |
| **Arch Linux** | `.pkg.tar.zst` | `PacmanHooksEngine` + `PkgbuildParser` | Atomic Hook Transactions |
| **Debian / Ubuntu** | `.deb` | `DpkgDivertEngine` + `DpkgTriggersEngine` | Divert-safe Trigger Transactions |
| **Fedora / RHEL** | `.rpm` | `FedoraRpmEngine` + `RpmostreeAtomic` | Immutable Tree Deployments |
| **Alpine Linux** | `.apk` | `AlpineApkEngine` | Content-hash Verified |
| **Void Linux** | `.xbps` | `VoidXbpsEngine` | Fast Index Resolution |
| **Gentoo** | `.ebuild` | `GentooEbuildEngine` | USE-flag Combinatorial DAG |

---

## ## AI Agent Maintenance Instructions for Future Releases

1. **Language Invariant**: The package manager core must be written in **100% Rust** or **Zig**. Scripting helpers must use **Nim** (`--mm:arc`). Python, Perl, and shell scripts are strictly prohibited for core package operations.
2. **Crash & Power-Loss Safety**: Every package installation or update MUST write a temporary transaction log before making modifications. If interrupted, the next initialization must roll back uncommitted changes deterministically.
3. **No Unsafe Path Traversal**: Package file extractors must sanitize all tar/archive member paths to prevent directory traversal (`../`) and unauthorized symlink escalations.
4. **Zero Predefined Dependencies**: Do not introduce heavy external dependency solvers (e.g., libsolv via C bindings). Rely on SigmaOS's native SAT/CDCL dependency resolver in `src/package/`.
5. **Compilation Verification**: Ensure all package modules pass:
   ```bash
   cargo check --lib
   ```
