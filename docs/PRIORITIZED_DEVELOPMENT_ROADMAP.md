# SigmaOS Strategic Development Roadmap

Inspired by Linux and BSD distribution best practices, this roadmap maps out 12 key strategic development pillars for SigmaOS.

---

## 🛡️ 1. Security & Sandboxing

* **Per-Tab Capability Model:** Enforce `pledge(2)` / `unveil(2)` / Capsicum rights per process launch.
* **Hardened Toolchain:** Enforce LTO, FORTIFY_SOURCE=3, stack-protector-strong, RELRO, and PIE across all build targets.
* **Extension Sandboxing:** Execute third-party extensions in isolated helper processes with signed manifests.
* **Continuous Fuzzing:** Maintain OSS-Fuzz harnesses for kernel VFS, IPC, and security input validation routines.

---

## 🚀 2. Release Engineering & Update Model

* **Multi-Channel Release Pipeline:** Stable/LTS, Beta, and Nightly release channels with automated cherry-pick backports.
* **Atomic OS Updates:** Transactional OSTree-like staged updates with one-click instant rollback.
* **Reproducible Builds & Sigstore Signing:** Deterministic builds signed with Cosign/Sigstore and SPDX SBOM manifests.

---

## 📦 3. Packaging & Distribution

* **Signed Package Registry:** Built-in `.sigpkg` repository manager supporting dependency resolution and rollback.
* **Curated Meta-Packages:** Bundled ports collections for Privacy, Developer Tools, and Media Workstations.
* **Source Profile USE Flags:** Gentoo-style tunable source compilation profiles.

---

## ⚡ 4. Process & Resource Control

* **cgroups v2 & rctl Quotas:** Per-tab and per-renderer CPU, memory RSS, and I/O rate limits.
* **Containerized Process Isolation:** Launch third-party helpers in lightweight FreeBSD Jail / gVisor sandboxes.

---

## 📊 5. Observability & Diagnostics

* **Low-Overhead eBPF & DTrace Probes:** Linux eBPF and Illumos/FreeBSD DTrace kernel profiling probes.
* **Symbolicated Crash Reports:** Aggregated, privacy-preserving crash report grouping with symbol upload tooling.
* **Session Snapshots:** Reproducible session state snapshots with PII scrubbing for bug reports.

---

## 🛠️ 6. Build & CI Improvements

* **Hermetic CI Images:** Containerized build toolchains with SHA-256 pinned Action dependencies.
* **Multi-OS Build Matrix:** Automated CI coverage across Linux, FreeBSD, NetBSD, DragonFly BSD, and OpenBSD targets.
* **Protected Branch Security Gates:** Required SBOM generation, vulnerability scans, and codeowner reviews.

---

## 🧪 7. Testing & QA

* **Distribution Package Install Tests:** Live ISO installation, upgrade simulation, and default workflow smoke tests.
* **Extension Compatibility Matrix:** Regression testing for third-party extensions against nightly builds.

---

## 🌐 8. System Integration & OS Features

* **Native Installer Images:** Live ISO images, Flatpak/Snap packages, macOS `.pkg`, and FreeBSD `ports` Makefiles.
* **Package Manager Upstream Integration:** Distribution recipes for Debian, Arch, Fedora, FreeBSD, and NetBSD pkgsrc.

---

## 🧑‍💻 9. Developer Experience

* **Reproducible Dev Shells:** Zero-setup Nix/Guix developer shells and Dev Containers.
* **Ports-Style Patch Workflow:** Standardized contribution guidelines and patch templates.

---

## 🔒 10. Privacy & Compliance

* **Privacy-by-Default Profiles:** Pre-configured privacy profiles with extension data access labels.
* **Automated CVE Scans:** Continuous SBOM dependency scanning and security advisory alerts.

---

## 🎨 11. UX & End-User Control

* **Installer Spins:** Curated installer profiles for Minimal, Workstation, Power User, and Privacy setups.
* **System Settings Panel:** Unified OS controls for update channels, telemetry, and resource policies.
