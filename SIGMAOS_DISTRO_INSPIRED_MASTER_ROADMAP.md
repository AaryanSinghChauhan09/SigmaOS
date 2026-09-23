# 🐧⚓ SIGMAOS LINUX & BSD DISTRO-INSPIRED HYBRID MASTER ROADMAP

## EXECUTIVE SUMMARY & HYBRID FORMULA

SigmaOS is an absolute, self-sufficient, sovereign operating system designed to combine the best architectural paradigms of **Linux** and **BSD**:
- **From Linux**: Broad hardware compatibility, vast package ecosystem, modern Wayland desktop tooling, eBPF performance tracing, and container virtualization.
- **From BSD**: Clean system design, conservative & readable defaults, security discipline (`rc.conf`, `pf.conf`, `pledge`/`unveil`, `Capsicum`), and well-defined privilege boundaries.

### The SigmaOS Hybrid Formula:
- **Kernel**: Hybrid Linux LTS microkernel layer for broad hardware/NVMe/GPU support and BSD-inspired subsystem isolation.
- **Service Model**: Async init supervisor with `rc.conf`-style simple declarative service configuration.
- **Package Ecosystem**: Native `sigpkg` curated repo + Flatpak + AppImage + Multi-Distro PR ingestion.
- **Security**: AppArmor/Landlock + `pf`-style default firewall + mandatory PQC update signing.
- **Filesystem**: Btrfs/ZFS Copy-on-Write with sub-second snapshot rollback and dual A/B root partition swapping.
- **Desktop**: Zenith Wayland-first custom DE with consistent theming, settings app, and accessibility defaults.
- **Administration**: Universal CLI/GUI parity across network, storage, packages, services, and users.
- **Philosophy**: Debian-like stability + Arch-like pace + BSD-like simplicity + Nix-like reproducibility.

---

## SECTION 1: BASE PHILOSOPHY & CORE PRINCIPLES

1. **Stable by Default, Modern When Needed**:
   - Core OS system binaries follow Long-Term Support (LTS) stability guidelines.
   - Desktop application layers use rolling channels, Flatpaks, and container isolates for bleeding-edge updates.

2. **Minimal but Not Barebones**:
   - Out-of-the-box installation provides a complete working desktop environment, media codecs, network managers, and basic utilities without bloat.

3. **Secure Defaults Out of the Box**:
   - Default-deny firewall policy (`pf.conf`), strict umask `0027`, read-only `/system` partitions, and capability sandboxing (`ZorinExecGuardPolicyEngine`).

4. **Declarative Configuration**:
   - Single-file state reconciliation (`/system/profile.toml` or `rc.conf` semantics) allowing atomic system generation tracking and instant generation rollbacks (`ConfigManager`).

5. **Fast Atomic Updates Without System Breakage**:
   - Dual-root partition swapping (`Root_A` / `Root_B`) with a 60-second hardware watchdog timer. If boot fails, zero-copy snapshot rollback triggers automatically.

6. **Clear Separation of Core OS, Apps, and User Config**:
   - `/system` (Immutable OS base)
   - `/apps` (Userland applications and sandboxed packages)
   - `/user` (User home directories, configurations, and personal data)

---

## SECTION 2: MISSING CORE SYSTEM LAYERS

### 1. Package Manager (`sigpkg`)
- **Mandatory Signing**: All package payloads signed with post-quantum Dilithium-5 / SHA3 keys.
- **Repo Metadata**: SQLite/BTree fast indexed repository mirrors with differential delta synchronization.
- **Dependency Resolution**: $O(N \log N)$ conflict scanner and constraint solver.
- **Multi-Channel Feeds**: `stable`, `testing`, `unstable/rolling` channels.
- **Rollback Support**: Transactional database logging allowing sub-second package transaction reversal (`SovereignPackageRollbackEngine`).

### 2. Init & Service Management
- **Declarative Service Definitions**: Readable unit manifests with explicit dependency graphs (`after = ["network", "dbus"]`).
- **Health Monitoring & Auto-Restart**: Automatic supervisor restart policies with exponential backoff on crash.
- **Log Aggregation**: Structured binary journal logging (`journald`/`syslog-ng` hybrid) with CRLF injection sanitization.

### 3. Boot & Filesystem Layout
- **Standard Hierarchy**: Clean POSIX file layout (`/etc`, `/var`, `/srv`, `/home`, `/system`).
- **Secure Boot**: Measured boot TPM 2.0 PCR register sealing (`PCR_4`).
- **Bootloader Configuration**: Limine / systemd-boot / GRUB fallback boot entries with automated snapshot selection.
- **Snapshot Storage**: Native Btrfs subvolumes (`@root`, `@home`, `@swap`) or OpenZFS dataset pools.

---

## SECTION 3: SECURITY AS A CORE FEATURE

1. **Default Firewall (`pf`-Style Rules)**:
   - Inbound traffic blocked by default; outbound stateful tracking enabled (`pass out all keep state`).
   - Integrated OpenBSD `pf` rule engine and Linux `nftables`/eBPF XDP filter.

2. **Privilege Boundaries & Sandboxing**:
   - Process privilege reduction via OpenBSD `pledge()` and path gating via `unveil()`.
   - Fine-grained file descriptor rights via FreeBSD Capsicum (`CapsicumRight`).
   - Binary execution capability gating via Zorin Exec Guard (`ZorinExecGuardPolicyEngine`).

3. **Audit & Integrity Verification**:
   - Automatic security updates for critical vulnerabilities.
   - Continuous system health monitoring and audit logging (`SecurityAdvisoryTracker`).
   - Secure memory zeroing on process exit (`Parrot OS` RAM scrubber).

---

## SECTION 4: MODERN ZENITH DESKTOP ENVIRONMENT

1. **Wayland-First Stack**:
   - Direct Wayland compositor integration with hardware DMA-BUF surface sharing and low-latency input pipelines.
   - Consistent theming across GTK, Qt, and native Canvas components (`Tokyo Night`, `Catppuccin`, `Gruvbox`, `Ayu`).

2. **Integrated System Controls**:
   - Comprehensive Settings App controlling Display, Scaling, Power Profiles, Wi-Fi/Bluetooth, Users, Audio (PipeWire), and Theme overlays.
   - Lock screen with PAM/BSD authentication memory zeroing safeguards.
   - System tray, workspace management, and accessibility shortcuts (WCAG 2.1 AA compliant).

---

## SECTION 5: SYSTEM ADMINISTRATION & ERGONOMICS

1. **GUI & CLI Parity**:
   - Every graphical setting in the Zenith Control Center has an exact 1:1 command-line interface counterpart (e.g., `omarchy powerprofiles`, `sigpkg install`, `sigma-config rollback`).

2. **Storage & Snapshot Management**:
   - Automated disk health monitoring (S.M.A.R.T. alerts).
   - Snapshot creation and restore UI with graphical time-machine browser.
   - Offline system recovery environment with graphical rescue wizard.

---

## SECTION 6: FIVE-PHASE CHRONOLOGICAL ROADMAP

```
========================================================================================
Phase 1: Foundation (Months 1-3)
- Package manager (`sigpkg`), Dilithium-5 repository signing, and dependency resolver
- Bootable base system with Limine/systemd-boot fallback entries
- Btrfs/ZFS Copy-on-Write snapshot engine and dual-root A/B update swapper

Phase 2: Zenith Desktop (Months 4-6)
- Wayland compositor, window manager, and app launcher
- Integrated Control Center (Settings app for displays, audio, power, network)
- File manager, text editor (`Omawrite`), floating calculator (`Omacalc`), and accessibility

Phase 3: Security & Sandboxing (Months 7-9)
- Default-deny pf-style firewall and eBPF XDP filter
- Landlock v5 + Pledge + Unveil + Capsicum sandboxing boundaries
- Read-only `/system` base partition and Zorin Exec Guard capability policies

Phase 4: Ecosystem & Developer Tools (Months 10-12)
- App store expansion (Flatpak, AppImage, multi-distro PR package ingestion)
- Developer toolchain bundles (Rust, Go, Python, C/C++, LLM AVX-512 inference)
- Container and MicroVM runtime integration (Firecracker, Podman)

Phase 5: Polish & Release (Months 13-15)
- Release channels: `stable`, `testing`, `rolling`
- Comprehensive ISO image creation, mirror ranking scripts, and installer setup wizard
- Full documentation, troubleshooting guides, and performance tuning
========================================================================================
```

---
*End of Master Roadmap Specification.*
