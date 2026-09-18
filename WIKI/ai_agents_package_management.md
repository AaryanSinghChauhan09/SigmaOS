# AI Agent Developer Guide: Universal Package Management in SigmaOS

This guide provides guidelines, Rust API references, CLI commands, and security guidelines for AI Autonomous Agents interacting with the **SigmaOS Universal Package Management Subsystem (`SigmaPkg`)**.

---

## 1. Overview & Universal Architecture

SigmaOS provides a zero-dependency, multi-distro universal package manager capable of natively parsing, inspecting, resolving dependencies for, and installing packages across **all major Linux, BSD, and Unix package formats**:

* **Linux Desktop & Enterprise Formats:** Debian (`.deb`), Fedora/RHEL (`.rpm`), Arch Linux (`.pkg.tar.zst`), Alpine Linux (`.apk`), Solus (`.moss`), Haiku (`.hpkg`), TinyCore (`.tcz`)
* **Embedded & Mobile Formats:** OpenWrt (`.ipk`), Yocto/OpenEmbedded (`.opkg`), Void Linux (`.xbps`)
* **UNIX & BSD Formats:** FreeBSD (`.txz`), OpenBSD (`.openbsd.tgz`), NetBSD (`.pkgsrc`), Solaris/Illumos (`.p5p`, `.ips`), GNU Guix/Nix (`.nar`)
* **Container & App Bundles:** Flatpak (`.flatpak`), AppImage (`.appimage`), Snap (`.snap`)

AI Agents can perform package discovery, dry-run simulation, format conversion, dependency resolution, and atomic installation without external sub-processes or shell execution dependencies.

---

## 2. Package Format Detection Logic

`SigmaPkg` uses magic header signatures and extension fallback to auto-detect package formats:

| Format Name | File Extension | Magic Header Bytes | Format Enum Variant |
| :--- | :--- | :--- | :--- |
| Debian | `.deb` | `!<arch>\n` | `PackageFormat::Debian` |
| RedHat RPM | `.rpm` | `\xed\xab\xee\xdb` | `PackageFormat::Rpm` |
| Arch Linux | `.pkg.tar.zst` | `\x28\xb5\x2f\xfd` (Zstd) | `PackageFormat::Pacman` |
| Alpine Linux | `.apk` | `\x1f\x8b\x08` (Gzip) | `PackageFormat::Apk` |
| Solus Moss | `.moss` | `MOSS` | `PackageFormat::Moss` |
| Haiku | `.hpkg` | `hpkg` | `PackageFormat::Hpkg` |
| OpenWrt | `.ipk` | `IPK!` | `PackageFormat::Ipk` |
| Yocto | `.opkg` | `OPKG` | `PackageFormat::Opkg` |
| Solaris IPS | `.p5p` / `.ips` | `P5P!` | `PackageFormat::P5p` |
| Nix / Guix | `.nar` | `NARS` | `PackageFormat::Nar` |

---

## 3. Rust API Reference for AI Agents

AI Agents embedded within SigmaOS or calling the system via FFI can use the following high-level Rust structs:

```rust
use sigmaos::sigpkg::{
    UniversalPackageManager, UniversalDependencyMapper, UniversalDryRunSimulator,
    UniversalFormatConverter, PackageFormat, StandardPackage
};

// 1. Initialize Universal Package Manager Engine
let mut manager = UniversalPackageManager::new("/var/lib/sigmaos/store");

// 2. Auto-Detect and Parse Package Header
let package_bytes: &[u8] = include_bytes!("../sample.deb");
let format = manager.detect_format_by_header(package_bytes)?;

// 3. Perform Dry-Run Simulation (Check Filesystem & Dependency Conflicts)
let simulator = UniversalDryRunSimulator::new();
let simulation = simulator.simulate_install(package_bytes, format)?;
assert!(simulation.success);

// 4. Resolve Cross-Distro Dependency (e.g. mapping 'so:libc.so.6' or 'python3-dev')
let mapper = UniversalDependencyMapper::new();
let canonical_dep = mapper.to_canonical_name("python3-dev"); // Returns 'python'

// 5. Atomic Install with Automatic Rollback Snapshot
manager.install_package_bytes(package_bytes, format)?;
```

---

## 4. CLI Commands for AI Agents

AI Agents invoking shell commands should use structured `sigma-pkg` CLI flags with JSON output formatting:

### Query Package Metadata (JSON)
```bash
sigma-pkg query --file /tmp/package.rpm --json
```

### Perform Dry-Run Install Simulation
```bash
sigma-pkg dry-run --file /tmp/package.pkg.tar.zst --json
```

### Cross-Distro Format Conversion
```bash
sigma-pkg convert --input /tmp/app.deb --target-format rpm --output /tmp/app.rpm
```

### Universal Package Installation
```bash
sigma-pkg install /tmp/package.moss --sandbox --json
```

---

## 5. Security, Sandboxing & Verification Constraints for AI Agents

When executing package operations, AI Agents MUST adhere to the following safety rules:

1. **ED25519 & Dilithium-5 Signature Verification:** Never bypass package signature verification unless `--allow-unsigned` is explicitly supplied in an isolated test environment.
2. **Mandatory Sandboxing:** All scriptlets (`pre-install`, `post-install`, `triggers`) run inside OpenBSD `pledge`/`unveil` and Linux `Landlock`/`AppArmor` isolated containers.
3. **Atomic CoW Snapshot Rollback:** Before modifying `/usr` or `/var/lib/sigmaos/store`, `SigmaPkg` automatically creates a Btrfs/ZFS Merkle CoW snapshot. If an installation step fails, the agent must trigger `sigma-pkg rollback`.
4. **No Hardcoded Secrets or Paths:** All package paths must resolve through `/var/lib/sigmaos/store` or user-specific sandbox directories.

---
*Maintained by the SigmaOS AI Agent Framework & Governance Board.*
