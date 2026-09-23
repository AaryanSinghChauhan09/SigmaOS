# SigmaOS Practical Distro-Inspired Master Roadmap

> **Strategic Vision:** Borrow the best ideas from Linux and BSD without becoming a generic clone.
> - **Linux Contributions:** Hardware support, driver ecosystem, rich package availability, modern desktop tooling, Wayland graphics stack.
> - **BSD Contributions:** Simplicity, clean defaults, security discipline (pledge/unveil, pf firewall), predictable service management (`rc.conf`), ZFS/Btrfs boot environments, clear privilege boundaries.
> - **Hybrid Formula:** Linux Kernel + BSD Service/Security Design + Debian Stability + Arch Velocity + Nix Reproducibility.

---

## 🏛️ 1. Base Philosophy
- **Stable by Default, Modern when Needed:** Production-worthy baseline with rolling edge channel option.
- **Minimal, Not Barebones:** Clean defaults out of the box with zero unwanted bloatware.
- **Secure Defaults Out of the Box:** Deny-by-default firewall (`pf`), default umask `0027`, AppArmor/Landlock process sandboxing.
- **Declarative Configuration:** Unified system state governance (`login.conf`, `/etc/environment.d`, `rc.conf`).
- **Transactional Updates:** Atomic upgrades with zero-downtime rollback capabilities.
- **Strict Separation of Concerns:** Core OS (`/usr`), dynamic state (`/var`), system config (`/etc`), user data (`/home`).

---

## 🧩 2. Core System Layers & Architecture
A desktop operating system requires a rock-solid system foundation.

### Package Management Subsystem (`sigpkg`)
- **Cryptographic Trust:** Signed packages only (Dilithium-5 PQC + Ed25519 GPG).
- **Multi-Format Interoperability:** Transpilation and CLI bridging for `apt`, `pacman`, `dnf`, `zypper`, `apk`, `xbps`, `ebuild`, `pkg`, `nix`.
- **Dependency Resolution:** SemVer-aware constraint resolution with automatic conflict detection.
- **Transactional Generations:** O(1) Btrfs/ZFS snapshot rollbacks (`sigpkg rollback <generation>`).
- **Release Channels:** `stable`, `testing`, `unstable`/`rolling`.

### Init & Service Management
- **Supervisor & Service Model:** Declarative service units with dependency tracking, health checks, socket activation, and restart policies.
- **BSD-Inspired Simplicity:** `rc.conf`-style toggle syntax for fast service governance.

### Boot, Filesystem & Updates
- **Standard Hierarchy:** FHS-compliant `/etc`, `/var`, `/srv`, `/home`, `/sovereign/store`.
- **Secure Boot & Fallbacks:** Signed EFI bootloaders with automated fallback entries.
- **Atomic Upgrades:** A/B partition swapping or snapshot-based transactional migration (`sigma-update`).

---

## 🛡️ 3. Security Discipline
Security integrated as a foundational pillar rather than an add-on.

- **Stateful Firewall:** Default `pf`-style packet filtering with sane desktop profiles.
- **Mandatory Access Control:** Landlock v5 + AppArmor/SELinux system call filtering.
- **Process Sandboxing:** OpenBSD `pledge()` and `unveil()` path restriction across userland binaries.
- **Executable Protection:** Zorin Exec Guard path boundary verification to prevent sandboxing bypasses.
- **Immutable Rootfs:** Read-only system partition mount overlays.
- **Supply Chain Integrity:** Automated SBOM generation and CAS checksum verification.

---

## 🖥️ 4. Modern Zenith Desktop Environment
A coherent, Wayland-first desktop OS experience.

- **Wayland-First Compositor:** Zenith compositor with GPU acceleration, dynamic tiling, and floating window management.
- **Integrated Control Center:** System settings app managing displays, sound, network, power, updates, and themes.
- **Polish & Consistency:** Integrated lock screen, session controls, notifications daemon, system tray, and app launcher.
- **XDG Compliance:** Full XDG desktop portals, MIME type associations, and Wayland clipboard management.

---

## 📦 5. Software Ecosystem & Package Categories
- **Curated Base Repository:** Core OS utilities, security tools, and system libraries.
- **Containerized Apps:** Flatpak and AppImage integration out-of-the-box.
- **Developer Toolchains:** Rust, Go, Python, Node.js, C/C++, Java, and Zig toolchains with version manager isolation (`mise`/`asdf`).
- **Categorized Software Store:** GUI Apps, Dev Tools, System Utilities, Media/Productivity, Gaming & Creative.

---

## ⚙️ 6. System Administration & Ergonomics
- **Dual GUI/CLI Parity:** Every GUI setting backed by a clean CLI equivalent tool (`sigma-ctl`, `sigpkg`, `timedatectl`).
- **Unified Settings Manager:** User permissions, network profiles, storage mounts, power profiles, and backups.
- **Comprehensive Documentation:** Manpages, interactive CLI guides (`sigma-help`), and offline documentation.

---

## 💾 7. Resilient Storage & Recovery
- **Snapshot Infrastructure:** Btrfs / ZFS boot environments (`bectl` / Snapper parity).
- **Disk Telemetry:** NVMe/SATA SMART disk health monitoring and thermal warnings.
- **Backup & Recovery Suite:** Native snapshot restore and home directory backup utility (`sigma-backup`).

---

## 🌐 8. Predictable Networking & Connectivity
- **Unified Network Manager:** Wi-Fi, Ethernet, WireGuard/Tailscale VPN, and DNS governance.
- **Virtualization & Containers:** Native lightweight pod lifecycle (`Podman`/`Docker` parity) and MicroVM support (`Firecracker`).

---

## 🛠️ 9. Developer & Power-User Workflows
- **Preinstalled Dev Stacks:** One-command environment bootstrap (`sigma-setup dev`).
- **Neovim Omakase IDE:** Native preconfigured IDE (`sigma-nvim`) with LSP support.
- **System Tracing & Debugging:** eBPF event tracing and system diagnostics (`htop`, `lsof`, `strace` parity).

---

## 🚀 10. Release Discipline & Maintenance
- **Release Tracks:**
  - `v1.0 Stable`: LTS base with conservative updates.
  - `Testing`: Staged evaluation for upcoming point releases.
  - `Nightly/Rolling`: Rolling edge updates for developers.
- **Versioned ISO Distributions:** Bootable ISO media verified with Rufus/Ventoy checksums.

---

## 🗺️ 11. Implementation Phases & Milestones

### Phase 1: Foundation Baseline
- Universal Package Manager (`sigpkg`) with multi-distro format support.
- Signed repository metadata and GPG keyring trust pipeline.
- Service supervisor and atomic snapshot rollback engine.

### Phase 2: Desktop Polish
- Zenith Wayland compositor features and display configuration.
- Unified Settings app and file manager (`sigma-fm`).
- Input methods, power management, and desktop accessibility.

### Phase 3: Security & Isolation
- Default `pf` firewall rules and AppArmor/Landlock profiles.
- OpenBSD pledge/unveil sandboxing for app execution.
- Exec Guard path boundary verification.

### Phase 4: Ecosystem & Tooling
- Software Manager GUI (`sigpkg-gui`) and AppStream catalog integration.
- Language version managers and developer environment presets.
- Container and VM orchestration.

### Phase 5: Production Release
- Stable ISO build pipeline and automated release criteria checks.
- Comprehensive user and developer documentation.
