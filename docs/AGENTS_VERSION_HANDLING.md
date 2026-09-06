# SigmaOS Version Handling & Multi-Distro Parity Guide for AI Agents

This guide provides technical specifications, version comparison algorithms, multi-distro format mapping rules, and procedural workflows for AI agents managing versions across SigmaOS core subsystems and the universal package manager (`sigpkg`).

---

## 1. Executive Summary & Core Rules

1. **Zero External Dependencies:**
   All version parsing, comparison, and range evaluation logic in SigmaOS MUST be implemented natively in Rust (`#![no_std]` compliant).
2. **Canonical Version Model (`crate::sigpkg::Version`):**
   * `epoch`: Optional integer prefix (default: `0`) for overriding version precedence.
   * `upstream`: Upstream software version string (e.g., `1.2.3`, `2024.01`, `5.15.0`).
   * `release` / `revision`: Distro package release counter (e.g., `1`, `r2`, `deb12u1`, `fc39`).
   * `build_metadata`: Optional hash or build timestamp string.

---

## 2. Multi-Distro Version Format Mapping Table

| Distribution / Format | Original Format | SigmaOS Canonical Translation | Comparison Algorithm |
| :--- | :--- | :--- | :--- |
| **Debian / APT (`.deb`)** | `[epoch:]upstream[-revision]` | `epoch = epoch`, `upstream = upstream`, `revision = revision` | `dpkg --compare-versions` logic |
| **RPM / DNF (`.rpm`)** | `[epoch:]version-release.dist` | `epoch = epoch`, `upstream = version`, `release = release` | RPM `rpmvercmp` algorithm |
| **Arch Linux / Pacman** | `pkgver-pkgrel` | `epoch = 0`, `upstream = pkgver`, `release = pkgrel` | `vercmp` alphanumeric chunking |
| **Alpine Linux (`.apk`)** | `version-r<pkgrel>` | `epoch = 0`, `upstream = version`, `release = r<pkgrel>` | APK numeric/alpha suffix rules |
| **Gentoo / Portage** | `version-r<revision>` | `epoch = 0`, `upstream = version`, `release = r<revision>` | Gentoo ebuild version spec |
| **Void Linux (`.xbps`)** | `name-version_revision` | `epoch = 0`, `upstream = version`, `release = revision` | XBPS version dictionarycmp |
| **Haiku OS (`.hpkg`)** | `name-version-revision` | `epoch = 0`, `upstream = version`, `release = revision` | Haiku BPackageVersion cmp |
| **BSD Ports / Pkg (`.pkg`)** | `version[,epoch]` | `epoch = epoch`, `upstream = version`, `release = 0` | FreeBSD `pkg_version` logic |

---

## 3. Version Comparison Algorithm Rules

When comparing two versions $V_1$ and $V_2$ in `sigpkg`:

1. **Epoch Comparison:** Compare `epoch` values numerically. If $E_1 \neq E_2$, the version with the higher epoch is greater regardless of upstream strings.
2. **Upstream Comparison:**
   * Split upstream strings into alternating numeric and non-numeric chunks.
   * Compare corresponding numeric chunks as integers (`10 > 2`).
   * Compare corresponding non-numeric chunks using ASCII alphabetical order (`beta < rc < final`).
   * Tilde `~` character handling: Any chunk starting with `~` sorts BEFORE empty strings or any other character (e.g., `1.0~rc1 < 1.0`).
3. **Release / Revision Comparison:** If epochs and upstream versions are identical, compare `release` / `revision` strings using the same chunking algorithm.

---

## 4. Kernel & Subsystem Versioning Workflow

When an AI agent modifies core kernel subsystems, drivers, or system call interfaces:

### 4.1 System Call Table Invariants (`src/kernel/syscall/table.rs`)
* Do NOT change existing syscall numbers (indices `0..499`).
* Add new extension syscalls at `500+` (e.g., `SigmaCryptoHash = 500`, `SigmaIoUring = 503`).
* Verify that `SyscallTable::list_registered()` includes all registered handlers.

### 4.2 KABI Compatibility Verification
* Run `cargo test --lib -- kernel::syscall::table::tests` to verify anti-rootkit shadow SSDT auditing.
* Ensure kernel symbols exported in `kabi/` maintain stable function signatures across minor release versions.

---

## 5. Automated Release Checklist for AI Agents

Follow these steps when performing a version bump or release tag:

1. **Check Workspace Status:**
   ```bash
   git status
   ./run_sigma_tests.sh
   ```
2. **Update Package Manifests:**
   * Update `version` in `Cargo.toml`.
   * Update `version` in `sigma-stable.toml` and `sigma-rolling.toml`.
3. **Update Release Documentation:**
   * Add new release entry to `CHANGELOG.md` with sections: `Added`, `Changed`, `Fixed`, `Security`.
   * Create or update `docs/RELEASE_NOTES_vX.Y.md`.
4. **Run Validation Suite:**
   ```bash
   ./run_sigma_tests.sh
   ```
5. **Commit & Tag Changes:**
   ```bash
   git commit -m "chore(release): bump version to x.y.z"
   ```
