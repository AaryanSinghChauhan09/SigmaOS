# Software Store and Command Palette

The **Software Store and Command Palette** subsystems ([`src/desktop/mint_software_store.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/desktop/mint_software_store.rs) and [`src/desktop/launcher.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/desktop/launcher.rs)) provide centralized application management, fine-grained sandbox auditing, zstd delta download optimization, and a universal keyboard-driven command palette.

---

## 1. Overview & Comparative Advantages

| Functional Area | Linux Mint (`mintinstall` / GMenu) | Omarchy (Arch / Walker) | **SigmaOS Store & Palette** |
| :--- | :--- | :--- | :--- |
| **Package Origins** | APT deb + Flathub Flatpak | Arch pacman + AUR | **Unified Native `sigpkg` + Flatpak OCI + AppImage** |
| **Sandbox Security Auditing** | Basic Flatpak permission tags | No integrated store audit | **Automated permission scoring (0–100) & auto-hardening** |
| **Download Optimization** | Full package re-download | Full tarball re-download | **Zstandard delta chunk estimation (~75% bandwidth reduction)** |
| **Command Palette Actions** | App launching only | App + window switcher | **Apps + Windows + Math + Clipboard + Kernel Performance Modes** |

---

## 2. Component Specifications

### A. Linux Mint-Inspired Software Store Engine (`MintSoftwareStoreEngine`)
*Source: [`src/desktop/mint_software_store.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/desktop/mint_software_store.rs)*

* **Security Permission Auditing (`audit_app_security`)**:
  * Evaluates requested permissions against category expectations.
  * Deducts points for `full_filesystem_access` (-35), unneeded `camera_microphone_access` (-20), `home_directory_access` (-10), and unverified developer signatures (-15).
* **Automated Sandbox Hardening (`harden_permissions`)**:
  * Automatically strips root and unrestricted filesystem access for untrusted applications.
* **Delta Package Compression (`estimate_delta_download_bytes`)**:
  * Leverages zstd dictionary diffs to download only changed binary pages, cutting update payload sizes by $\sim 75\%$.
* **Atomic Batch Installation (`batch_install`)**:
  * Pre-validates all dependencies and package availability before executing multi-app deployments without partial transaction failures.

### B. Universal Command Palette & Launcher (`CommandPalette`)
*Source: [`src/desktop/launcher.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/desktop/launcher.rs)*

* **Multi-Mode Execution**:
  * `LauncherMode::Application`: Search indexed apps with launch count popularity weighting.
  * `LauncherMode::WindowSwitcher`: Instantly jump to active windows across all Wayland workspaces.
  * `LauncherMode::Calculator`: Evaluates inline math expressions (`= 50 * 2` $\rightarrow$ `= 100`) and copies results.
  * `LauncherMode::Clipboard`: Clipboard history snippets (`cb <query>`).
  * `LauncherMode::SystemAction`: Kernel state triggers.
* **Sovereign System Action Triggers**:
  * `:gaming`: Activates CPU `performance` governor and increases GPU TDP ceiling.
  * `:powersave`: Switches CPU to energy-saving frequency governor.
  * `:snapshot`: Creates an instantaneous Btrfs/SigmaFS Timeshift snapshot.
  * `:backup`: Exports portable software selection manifests and user data backups.
  * `:matrix`: Runs the 12-pillar launch superiority verification suite.

---

## 3. Source Code Reference

* Software Store: [`src/desktop/mint_software_store.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/desktop/mint_software_store.rs)
* Command Palette: [`src/desktop/launcher.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/desktop/launcher.rs)

---

## 4. AI Agent Maintenance Instructions

> **For AI Agents Maintaining This Page:**
> - Source: `src/desktop/mint_software_store.rs`, `src/desktop/launcher.rs`
> - When new system action shortcuts are added to `CommandPalette`, register them in the system action table.
> - Ensure sandbox permission audit weights remain aligned with ISO 27001 and CIS Linux benchmarks.
> - Cross-reference with [Packaging](09-Packaging.md) and [Compositor](Compositor.md).
