# AI Agent Roadmap: Debian Parity & APT Ecosystem

This document outlines the strategic AI agent roadmap for **Debian Parity, APT Package Manager Solver, Dpkg Triggers, Alternatives Management, and DFSG License Compliance** in SigmaOS.

---

## 1. Debian Parity Architecture in SigmaOS

| Component | Inspiration | SigmaOS Implementation | AI Agent Autonomous Function |
| :--- | :--- | :--- | :--- |
| **Package Control & Status** | `dpkg` + `/var/lib/dpkg/status` | `DebControl` + `DpkgStatusEntry` (`src/package/debian.rs`) | Parses Debian control files and dpkg status records, validating dependency fields and architectures. |
| **Alternatives Management** | `update-alternatives` | `DebianUpdateAlternativesEngine` | Manages master symlink priorities for system default commands (`editor`, `x-terminal-emulator`, `cc`). |
| **Mark Status Manager** | `apt-mark` | `DebianAptMarkEngine` | Tracks package auto/manual/hold installation marks to protect held kernels and prune unreferenced auto-dependencies. |
| **Package Trigger Hooks** | `dpkg-trigger` | `DebianDpkgTriggerEngine` | Manages deferred package configuration triggers (e.g. `ldconfig`, `mimedb`, `desktop-database`) to batch execution post-transaction. |
| **Multi-Arch Directory Mapping** | Multi-Arch triplets | `DebianMultiarchPathResolver` | Maps multi-arch library and header directory structures (`/usr/lib/x86_64-linux-gnu`, `/usr/lib/aarch64-linux-gnu`). |
| **DFSG Compliance Auditor** | Debian Free Software Guidelines | `DebianDfsgComponentPolicy` (`src/package/debian.rs`) | Audits package components across `main`, `contrib`, `non-free`, and `non-free-firmware` based on open-source licensing guidelines. |

---

## 2. Milestone Roadmap Tracks

### Track A: APT Dependency Solver & Debconf Preseed Parser (Months 1–3)
1. **Boolean SAT Dependency Resolution**: Synthesize dependency graphs for `Depends`, `Pre-Depends`, `Recommends`, `Suggests`, `Conflicts`, `Breaks`, and `Provides` fields.
2. **Debconf Preseed Answer Generator**: Parse and generate `debconf` preseed answer files for unattended Debian/Ubuntu installer deployments.

### Track B: Dpkg Divert, Statoverride, & Trigger Processing (Months 4–6)
1. **Automated Path Diversion (`dpkg-divert`)**: Register diverted binary path overrides, preserving original package binaries while replacing active commands.
2. **Permission Override Enforcement (`dpkg-statoverride`)**: Enforce file permission and ownership overrides on security-sensitive binaries (`su`, `sudo`, `ping`).

### Track C: Lintian Static Analysis & Hermetic Buildfarm (Months 7–12)
1. **Automated Lintian Policy Auditing**: Execute static policy rule checks over `.deb` packages, flagging missing changelogs, non-FHS paths, or invalid `control` syntax.
2. **Hermetic Buildfarm Package Reconstitution**: Compile `.deb` packages in clean-room chroot containers, guaranteeing byte-for-byte build reproducibility (`SOURCE_DATE_EPOCH`).

---

## 3. Verification & Compliance Standards

- **Unit Test Coverage:** All Debian parity modules must include unit tests in `src/package/debian.rs` and `src/distro/debian_parity.rs`.
- **System Verification:** Execute `./run_sigma_tests.sh` to confirm 100% pass rate across all 174 active subsystems.
- **Zero External Dependencies:** Native `#![no_std]` or Safe-Rust implementations without external crate dependencies.
