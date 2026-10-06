# Universal Package Manager Pull Request Transpilation Roadmap

## PR Proposal Header
- **Title:** `[SIGPKG-PR] Universal Multi-Distro Package Transpilation & Automated PR Workflow`
- **Type:** Feature Specification & Deployment Roadmap
- **Status:** Accepted & Enforced
- **Target Engine:** `SovereignUniversalPrExecutionMasterSuite` (`src/package/universal_pr_execution_engine.rs`)

---

## 1. Overview & Architecture Goal

The SigmaOS Universal Package Subsystem allows contributors and automated CI bots to submit software packages in **any foreign package manager format** (Debian `.deb`, Arch `PKGBUILD`, Fedora `.rpm`, Alpine `.apk`, Void `.xbps`, FreeBSD Ports, OpenBSD Ports, openSUSE Zypper, Gentoo Ebuilds, Nix Flakes, Solus Moss, Haiku `.hpkg`, and Flatpak/AppImage bundles) via Pull Requests (PRs).

The `SovereignUniversalPrExecutionMasterSuite` automatically parses, validates, and transpiles foreign manifests into native, sandboxed, Post-Quantum Cryptography (PQC) signed `.sigpkg` packages and stages them across multi-channel repositories (`Stable`, `Testing`, `Rolling`).

---

## 2. Multi-Distro PR Format Transpilation Matrix

| Foreign Distro / Format | Source Manifest Payload | Transpiled Dependency Mapping | Assigned Sandbox Profile | Default Target Channel |
| :--- | :--- | :--- | :--- | :--- |
| **Debian / Ubuntu (`.deb`)** | `control` file, `debian/rules` | `sigma-compat-<dep>` | `apparmor-strict` | `Testing` |
| **Arch Linux (`PKGBUILD`)** | `PKGBUILD`, `.SRCINFO`, AUR | `sigma-arch-<dep>` | `seccomp-landlock` | `Rolling` |
| **Fedora / RHEL (`.rpm`)** | `.spec` file, RPM headers | `sigma-rpm-<dep>` | `selinux-confined` | `Testing` |
| **Alpine Linux (`.apk`)** | `APKBUILD`, `APKINDEX` | `sigma-apk-<dep>` | `musl-chroot` | `Stable` |
| **Void Linux (`.xbps`)** | `template` file | `sigma-xbps-<dep>` | `runit-isolated` | `Rolling` |
| **FreeBSD / OpenBSD Ports** | `Makefile`, `pkg-descr` | `sigma-bsd-<dep>` | `pledge-capsicum-jail` | `Stable` |
| **openSUSE (`Zypper`)** | `.spec` file, Delta RPM | `sigma-zypper-<dep>` | `snapper-cow-sandbox` | `Testing` |
| **Gentoo (`Portage`)** | `.ebuild` file, USE flags | `sigma-ebuild-<dep>` | `portage-sandbox` | `Rolling` |
| **NixOS / GNU Guix** | `flake.nix`, `.scm` scheme | `sigma-cas-<dep>` | `hermetic-store-sandbox` | `Stable` |
| **Flatpak / Snap / OCI** | `manifest.json`, `snapcraft.yaml` | `sigma-bundle-<dep>` | `oci-rootless-container` | `Stable` |

---

## 3. Automated PR Execution Pipeline

When a contributor submits a PR containing a foreign package specification:

```
[ Contributor GitHub PR ]
       │ (Contains PKGBUILD, control, .spec, Makefile, ebuild, etc.)
       ▼
[ Automated PR CI Pipeline ]
       │
       ├── 1. Format Detection & Parser Dispatch (`PullRequestPackageFormat`)
       ├── 2. Syntax & Security Policy Validation (check_unveil, check_landlock)
       ├── 3. Transpilation Engine (`AptDebianPrTranspiler`, `PacmanArchPrTranspiler`, etc.)
       ├── 4. Dependency Graph Resolution (`sigma-compat-*`)
       ├── 5. Post-Quantum Cryptography (PQC) Dilithium-5 Package Signing
       └── 6. Multi-Channel Repository Staging (`Stable`, `Testing`, `Rolling`)
       ▼
[ Native `.sigpkg` Asset Generation & Auto-Merge ]
```

---

## 4. Architectural Specs & Execution Rules

1. **Zero Runtime Transpilation Overhead:** Transpilation occurs strictly during PR validation at CI build time; the runtime package manager (`sigpkg`) installs pre-transpiled native `.sigpkg` archives with zero transpilation latency.
2. **Deterministic Dependency Translation:** All foreign dependencies (e.g., `libc6`, `glibc`, `musl`, `ncurses`) are deterministically mapped to native `sigma-compat-*` virtual capabilities.
3. **Sandbox Isolation Enforcement:** Each transpiled package payload is tagged with an immutable security sandbox profile (`seccomp-landlock`, `pledge-capsicum-jail`, `selinux-confined`) enforced at launch time by the kernel.
4. **PQC Dilithium-5 Signature Verification:** Every transpiled archive must pass PQC signature verification before promotion to the `Stable` channel.

---

## 5. Implementation Status & Verification

- [x] Transpiler module implementation for Debian, Arch, Fedora, Alpine, Void, BSD Ports, Nix, Zypper, Gentoo, and Flatpak (`src/package/universal_pr_execution_engine.rs`).
- [x] Multi-format PR gateway and auto-merger (`src/package/sovereign_pr_package_gateway.rs`).
- [x] Universal PM PR bridge engine (`src/package/sovereign_universal_pm_pr_bridge.rs`).
- [x] Standalone unit tests verified (`build/universal_pr_execution_test`).
