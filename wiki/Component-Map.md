# SigmaOS Component Map

> Full registry of all 149 component repositories in the SigmaOS multi-repo architecture.
>
> **Status Key:** 🔲 Planned | 🔨 Scaffolded | 🚧 In Progress | ✅ Stable | ⚠️ Experimental

---

## Layer Index

- [Firmware & Boot (8)](#firmware--boot)
- [Kernel (14)](#kernel)
- [System Services (22)](#system-services)
- [Desktop Environment (18)](#desktop-environment)
- [Core Applications (20)](#core-applications)
- [AI & Intelligence (8)](#ai--intelligence)
- [Security (10)](#security)
- [Package & Update (8)](#package--update)
- [Hardware Abstraction (12)](#hardware-abstraction)
- [Build & Toolchain (10)](#build--toolchain)
- [Developer Tools (9)](#developer-tools)
- [Common Libraries (10)](#common-libraries)

Total: **149 components**

---

## Firmware & Boot

| Component Name | Repo | Description | Language | Layer | Status | Upstream Inspiration |
|----------------|------|-------------|----------|-------|--------|---------------------|
| sigma-bootloader | sigma-bootloader | GRUB2 configuration, patches, and scripts for SigmaOS boot entries | Shell, Python | Boot | 🔨 Scaffolded | GRUB2, Linux Mint grub config |
| sigma-shim | sigma-shim | UEFI shim layer for Secure Boot chain trust | C | Boot | 🔲 Planned | shim (rhboot/shim) |
| sigma-uefi | sigma-uefi | UEFI application helpers and boot variable management | Rust | Boot | 🔲 Planned | efibootmgr, systemd-boot |
| sigma-initrd | sigma-initrd | Initial RAM disk build system and early userspace | Rust, Shell | Boot | 🔨 Scaffolded | dracut, mkinitcpio |
| sigma-acpi | sigma-acpi | ACPI table patching and override management | C, Python | Boot | 🔲 Planned | linux-firmware, acpica |
| sigma-microcode | sigma-microcode | CPU microcode update packaging for Intel and AMD | Shell | Boot | 🔲 Planned | intel-microcode, amd-ucode |
| sigma-grub-theme | sigma-grub-theme | Visual theme for GRUB2 boot menu | Shell, Python | Boot | 🔲 Planned | Vimix GRUB theme |
| sigma-boot-config | sigma-boot-config | Boot parameters, kernel cmdline management, boot profiles | Rust, TOML | Boot | 🔨 Scaffolded | systemd-boot loader.conf |

---

## Kernel

| Component Name | Repo | Description | Language | Layer | Status | Upstream Inspiration |
|----------------|------|-------------|----------|-------|--------|---------------------|
| sigma-kernel | sigma-kernel | Kernel config, patches, build system, and release scripts | C, Rust, Shell | Kernel | 🚧 In Progress | Linux 6.x LTS, Liquorix patches |
| sigma-kernel-config | sigma-kernel-config | Platform-specific kernel Kconfig files (x86_64, ARM64) | KConfig | Kernel | 🚧 In Progress | Debian kernel config, Arch linux config |
| sigma-mm | sigma-mm | Memory management policies, huge pages, ZRAM config | C, Rust | Kernel | 🔲 Planned | Linux mm subsystem |
| sigma-sched | sigma-sched | Scheduler config, EEVDF tuning, RT patches | C | Kernel | 🔲 Planned | EEVDF scheduler, CFS |
| sigma-ipc-core | sigma-ipc-core | Core IPC primitives — pipes, sockets, shared memory | C | Kernel | 🔲 Planned | Linux IPC |
| sigma-sec | sigma-sec | LSM hooks, seccomp profiles, Landlock policies, kernel hardening | C, Rust | Kernel | 🔲 Planned | Landlock, SELinux, AppArmor |
| sigma-drivers | sigma-drivers | Out-of-tree Rust kernel drivers and driver patches | C, Rust | Kernel | 🔨 Scaffolded | Linux driver model |
| sigma-fs | sigma-fs | Filesystem config — btrfs primary, ext4/xfs secondary | C | Kernel | 🔲 Planned | btrfs, ext4 |
| sigma-net-core | sigma-net-core | Networking stack config, eBPF programs, TC rules | C | Kernel | 🔲 Planned | Linux networking |
| sigma-crypto | sigma-crypto | Kernel crypto API, keyring management, TPM2 integration | C, Rust | Kernel | 🔲 Planned | Linux keyring, tpm2-tools |
| sigma-dm-verity | sigma-dm-verity | dm-verity setup for root filesystem integrity | C, Shell | Kernel | 🔲 Planned | dm-verity, Verified Boot |
| sigma-kmod | sigma-kmod | Kernel module management, signing, and autoload rules | Rust, Shell | Kernel | 🔲 Planned | kmod, modprobe |
| sigma-ebpf | sigma-ebpf | eBPF programs for performance monitoring and security | C (eBPF), Rust | Kernel | 🔲 Planned | bpftrace, Cilium |
| sigma-cgroup | sigma-cgroup | cgroup v2 hierarchy configuration and resource policies | Rust, Shell | Kernel | 🔲 Planned | systemd cgroup, cgroups v2 |

---

## System Services

| Component Name | Repo | Description | Language | Layer | Status | Upstream Inspiration |
|----------------|------|-------------|----------|-------|--------|---------------------|
| sigma-init | sigma-init | PID 1 init system (systemd integration + custom units) | Rust, Shell | System | 🚧 In Progress | systemd, OpenRC |
| sigma-pkg | sigma-pkg | Package manager with Rust frontend (APT/pacman backend) | Rust | System | 🔨 Scaffolded | APT, pacman, nix |
| sigma-update | sigma-update | Secure, atomic, signed system update daemon | Rust | System | 🔨 Scaffolded | ostree, swupd |
| sigma-net | sigma-net | Network management daemon + DNS-over-HTTPS client | Rust | System | 🔲 Planned | NetworkManager |
| sigma-audio | sigma-audio | Audio session manager + PipeWire configuration | Rust | System | 🔲 Planned | PipeWire, PulseAudio |
| sigma-input | sigma-input | Input device management, keymapping, libinput config | Rust | System | 🔲 Planned | libinput, evdev |
| sigma-display | sigma-display | Display server abstraction (Wayland/X11) | Rust | System | 🔨 Scaffolded | wlroots, libdrm |
| sigma-power | sigma-power | Power management, ACPI events, suspend/resume hooks | Rust | System | 🔲 Planned | UPower, TLP |
| sigma-udev | sigma-udev | udev rules, device event routing, hotplug | Rust, Shell | System | 🔲 Planned | udev, eudev |
| sigma-dbus | sigma-dbus | D-Bus session and system bus policy + daemons | Rust | System | 🔲 Planned | dbus-daemon, dbus-broker |
| sigma-keyring | sigma-keyring | Secret service, credential store, keyring daemon | Rust | System | 🔲 Planned | gnome-keyring, KWallet |
| sigma-journal | sigma-journal | Structured logging aggregator and query interface | Rust | System | 🔨 Scaffolded | systemd-journal, journald |
| sigma-time | sigma-time | NTP/PTP time synchronization daemon | Rust | System | 🔲 Planned | chrony, systemd-timesyncd |
| sigma-locale | sigma-locale | Locale and internationalization management | Rust | System | 🔲 Planned | glibc locale, locale-gen |
| sigma-user | sigma-user | User account management, PAM configuration | Rust | System | 🔲 Planned | shadow-utils, PAM |
| sigma-mount | sigma-mount | Automatic mounting, udisks2 integration | Rust | System | 🔲 Planned | udisks2, udiskie |
| sigma-print | sigma-print | Print subsystem integration (CUPS) | Rust | System | 🔲 Planned | CUPS |
| sigma-bluetooth | sigma-bluetooth | Bluetooth stack management | Rust | System | 🔲 Planned | BlueZ |
| sigma-geolocation | sigma-geolocation | Geolocation service (for time zone, night mode) | Rust | System | 🔲 Planned | geoclue |
| sigma-accessibility | sigma-accessibility | Accessibility services (a11y dbus services) | Rust | System | 🔲 Planned | AT-SPI2 |
| sigma-flatpak | sigma-flatpak | Flatpak runtime management and portal integration | Rust | System | 🔲 Planned | Flatpak |
| sigma-container | sigma-container | OCI container runtime for development use | Rust | System | 🔲 Planned | Podman, Bubblewrap |

---

## Desktop Environment

| Component Name | Repo | Description | Language | Layer | Status | Upstream Inspiration |
|----------------|------|-------------|----------|-------|--------|---------------------|
| sigma-shell | sigma-shell | Wayland compositor and shell (wlroots-based) | Rust | Desktop | 🔨 Scaffolded | Sway, Wayfire, wlroots |
| sigma-wm | sigma-wm | Window management policies, tiling/floating logic | Rust | Desktop | 🔨 Scaffolded | i3, Hyprland |
| sigma-panel | sigma-panel | Top/bottom panel with modular applets | Rust | Desktop | 🔲 Planned | Cinnamon panel, waybar |
| sigma-dm | sigma-dm | Display manager and session greeter | Rust | Desktop | 🔲 Planned | greetd, LightDM |
| sigma-theme | sigma-theme | GTK4 + Qt6 theme, icon pack, cursor theme | Rust, CSS | Desktop | 🔲 Planned | Adwaita, Papirus |
| sigma-notifications | sigma-notifications | Notification daemon (freedesktop spec) | Rust | Desktop | 🔲 Planned | mako, dunst |
| sigma-search | sigma-search | Global search and app launcher | Rust | Desktop | 🔲 Planned | rofi, wofi, GNOME search |
| sigma-session | sigma-session | Session manager, autostart, XDG compliance | Rust | Desktop | 🔲 Planned | gnome-session, xfce4-session |
| sigma-polkit | sigma-polkit | Graphical privilege escalation agent | Rust | Desktop | 🔲 Planned | polkit-gnome, lxqt-policykit |
| sigma-portal | sigma-portal | XDG desktop portals (file chooser, screenshot, etc.) | Rust | Desktop | 🔲 Planned | xdg-desktop-portal |
| sigma-clipboard | sigma-clipboard | Clipboard manager with history | Rust | Desktop | 🔲 Planned | wl-clipboard, clipman |
| sigma-screen | sigma-screen | Screenshot and screen recording | Rust | Desktop | 🔲 Planned | grim, slurp, wf-recorder |
| sigma-wallpaper | sigma-wallpaper | Wallpaper engine with dynamic/AI-generated backgrounds | Rust | Desktop | 🔲 Planned | swaybg, Variety |
| sigma-workspace | sigma-workspace | Workspace/virtual desktop management | Rust | Desktop | 🔲 Planned | i3 workspaces |
| sigma-input-method | sigma-input-method | Input method framework (IME) for CJK and other scripts | Rust | Desktop | 🔲 Planned | IBus, Fcitx5 |
| sigma-accessibility-desktop | sigma-accessibility-desktop | Desktop accessibility: screen reader, magnifier, high contrast | Rust | Desktop | 🔲 Planned | Orca, GNOME a11y |
| sigma-color-management | sigma-color-management | Color profile management and ICC support | Rust | Desktop | 🔲 Planned | colord |
| sigma-night-mode | sigma-night-mode | Night light / blue light filter | Rust | Desktop | 🔲 Planned | gammastep, redshift |

---

## Core Applications

| Component Name | Repo | Description | Language | Layer | Status | Upstream Inspiration |
|----------------|------|-------------|----------|-------|--------|---------------------|
| sigma-browser | sigma-browser | Privacy-first web browser (Firefox ESR-based) | Rust, C++ | App | 🔲 Planned | Firefox, LibreWolf |
| sigma-files | sigma-files | File manager with previews and tags | Rust | App | 🔲 Planned | Nautilus, Nemo |
| sigma-terminal | sigma-terminal | GPU-accelerated terminal emulator | Rust | App | 🔲 Planned | Alacritty, Kitty |
| sigma-text-editor | sigma-text-editor | Plain text and markdown editor | Rust | App | 🔲 Planned | gedit, Kate |
| sigma-code-editor | sigma-code-editor | Full-featured code editor with LSP support | Rust | App | 🔲 Planned | Zed, VS Code |
| sigma-settings | sigma-settings | Unified system settings UI | Rust | App | 🔲 Planned | GNOME Settings, System Settings |
| sigma-store | sigma-store | App store (Flatpak + native packages) | Rust | App | 🔲 Planned | GNOME Software, Discover |
| sigma-media | sigma-media | Video and music player | Rust | App | 🔲 Planned | Celluloid, Rhythmbox |
| sigma-image-viewer | sigma-image-viewer | Image viewer with basic editing | Rust | App | 🔲 Planned | eog, Shotwell |
| sigma-archive-manager | sigma-archive-manager | Archive compression/decompression GUI | Rust | App | 🔲 Planned | file-roller |
| sigma-calendar | sigma-calendar | Calendar and event manager | Rust | App | 🔲 Planned | GNOME Calendar |
| sigma-contacts | sigma-contacts | Contacts manager | Rust | App | 🔲 Planned | GNOME Contacts |
| sigma-mail | sigma-mail | Email client | Rust | App | 🔲 Planned | Thunderbird, Geary |
| sigma-notes | sigma-notes | Note-taking app with markdown and sync | Rust | App | 🔲 Planned | GNOME Notes, Obsidian |
| sigma-maps | sigma-maps | Offline-capable maps (OpenStreetMap) | Rust | App | 🔲 Planned | GNOME Maps |
| sigma-weather | sigma-weather | Weather app | Rust | App | 🔲 Planned | GNOME Weather |
| sigma-calculator | sigma-calculator | Scientific calculator | Rust | App | 🔲 Planned | GNOME Calculator |
| sigma-monitor | sigma-monitor | System monitor (CPU, RAM, disk, network) | Rust | App | 🔲 Planned | GNOME System Monitor |
| sigma-disk-manager | sigma-disk-manager | Disk partitioning and management GUI | Rust | App | 🔲 Planned | GNOME Disks, GParted |
| sigma-fonts | sigma-fonts | Font manager and viewer | Rust | App | 🔲 Planned | GNOME Font Viewer |

---

## AI & Intelligence

| Component Name | Repo | Description | Language | Layer | Status | Upstream Inspiration |
|----------------|------|-------------|----------|-------|--------|---------------------|
| sigma-ai-assistant | sigma-ai-assistant | AI assistant with local (Ollama) and cloud backends | Rust, Python | App | 🔲 Planned | GNOME AI, Copilot |
| sigma-ai-core | sigma-ai-core | Core AI runtime: model management, inference API | Rust, Python | System | 🔲 Planned | Ollama, llama.cpp |
| sigma-ai-search | sigma-ai-search | AI-powered semantic search over files and apps | Rust | Desktop | 🔲 Planned | Semantic search, Meilisearch |
| sigma-ai-shell | sigma-ai-shell | Natural language shell command suggestions | Rust | Desktop | 🔲 Planned | GitHub Copilot CLI, fish AI |
| sigma-ai-log-analyzer | sigma-ai-log-analyzer | Anomaly detection and log summarization | Rust, Python | System | 🔲 Planned | Logstash AI, Graylog |
| sigma-ai-pkg | sigma-ai-pkg | AI-assisted package recommendations | Rust | System | 🔲 Planned | apt-suggest, Nix AI |
| sigma-ai-wallpaper | sigma-ai-wallpaper | AI-generated wallpapers using local Stable Diffusion | Rust, Python | Desktop | 🔲 Planned | Stable Diffusion, wallpaper AI |
| sigma-ai-privacy | sigma-ai-privacy | Privacy guardian: monitors AI data access and consent | Rust | System | 🔲 Planned | Privacy sandbox |

---

## Security

| Component Name | Repo | Description | Language | Layer | Status | Upstream Inspiration |
|----------------|------|-------------|----------|-------|--------|---------------------|
| sigma-secboot | sigma-secboot | Secure Boot key management and shim enrollment | Rust, Shell | Boot | 🔲 Planned | sbctl, shim-signed |
| sigma-apparmor | sigma-apparmor | AppArmor profiles for all SigmaOS components | Shell | System | 🔲 Planned | AppArmor, Ubuntu profiles |
| sigma-seccomp | sigma-seccomp | seccomp-bpf filter profiles per process class | C, Rust | System | 🔲 Planned | systemd seccomp, Flatpak |
| sigma-landlock | sigma-landlock | Landlock LSM policy management daemon | Rust | System | 🔲 Planned | Landlock, sandbox-exec |
| sigma-firewall | sigma-firewall | Firewall configuration (nftables-based) | Rust | System | 🔲 Planned | firewalld, ufw |
| sigma-vpn | sigma-vpn | VPN client integration (WireGuard + OpenVPN) | Rust | System | 🔲 Planned | NetworkManager VPN, Mullvad |
| sigma-audit | sigma-audit | Kernel audit subsystem integration and log analysis | Rust | System | 🔲 Planned | auditd |
| sigma-tpm | sigma-tpm | TPM2 integration for measured boot and key storage | Rust | System | 🔲 Planned | tpm2-tools, systemd-cryptenroll |
| sigma-fde | sigma-fde | Full disk encryption setup and management (LUKS2) | Rust | System | 🔲 Planned | cryptsetup, LUKS2 |
| sigma-integrity | sigma-integrity | IMA/EVM integrity measurement architecture | C, Rust | Kernel | 🔲 Planned | Linux IMA |

---

## Package & Update

| Component Name | Repo | Description | Language | Layer | Status | Upstream Inspiration |
|----------------|------|-------------|----------|-------|--------|---------------------|
| sigma-repo | sigma-repo | Package repository server and metadata management | Rust | System | 🔲 Planned | reprepro, Arch repo |
| sigma-pkg-build | sigma-pkg-build | Package build system and spec format | Rust, TOML | Build | 🔨 Scaffolded | PKGBUILD, deb packaging |
| sigma-pkg-sign | sigma-pkg-sign | GPG/minisign package signing infrastructure | Rust | System | 🔲 Planned | dpkg-sig, pacman signing |
| sigma-delta-update | sigma-delta-update | Binary delta update generation and application | Rust | System | 🔲 Planned | casync, zsync |
| sigma-rollback | sigma-rollback | Btrfs snapshot-based system rollback | Rust | System | 🔲 Planned | snapper, timeshift |
| sigma-recovery | sigma-recovery | Recovery mode environment and repair tools | Rust, Shell | System | 🔲 Planned | Linux Mint recovery |
| sigma-flatpak-mgr | sigma-flatpak-mgr | Flatpak repository management and update orchestration | Rust | System | 🔲 Planned | Flathub |
| sigma-distrobox | sigma-distrobox | Container-based compatibility layer for other distro packages | Rust | System | 🔲 Planned | Distrobox |

---

## Hardware Abstraction

| Component Name | Repo | Description | Language | Layer | Status | Upstream Inspiration |
|----------------|------|-------------|----------|-------|--------|---------------------|
| sigma-gpu | sigma-gpu | GPU driver management, Vulkan config, PRIME support | Rust | System | 🔲 Planned | PRIME, switcheroo, GPU manager |
| sigma-wifi | sigma-wifi | Wi-Fi driver management, firmware, iwlwifi config | Rust, Shell | System | 🔲 Planned | iwd, NetworkManager Wi-Fi |
| sigma-bluetooth-hw | sigma-bluetooth-hw | Bluetooth hardware firmware and kernel config | Shell | System | 🔲 Planned | BlueZ, linux-firmware |
| sigma-audio-hw | sigma-audio-hw | Audio hardware profiles (SOF firmware, ALSA UCM) | C, Shell | System | 🔲 Planned | SOF firmware, ALSA UCM |
| sigma-touchpad | sigma-touchpad | Touchpad gesture configuration and tuning | Rust | System | 🔲 Planned | libinput, mtrack |
| sigma-display-hw | sigma-display-hw | Display hardware: HDR, VRR, multi-monitor hotplug | Rust | System | 🔲 Planned | KMS/DRM, wlr-randr |
| sigma-camera | sigma-camera | Camera device support and V4L2 configuration | Rust | System | 🔲 Planned | V4L2, libcamera |
| sigma-battery | sigma-battery | Battery management, charge thresholds, health monitoring | Rust | System | 🔲 Planned | TLP, powertop, upower |
| sigma-thermal | sigma-thermal | Thermal management, fan control, throttling policies | Rust | System | 🔲 Planned | thermald, fan2go |
| sigma-firmware | sigma-firmware | Firmware update integration (fwupd) | Rust | System | 🔲 Planned | fwupd, LVFS |
| sigma-hardware-detect | sigma-hardware-detect | Hardware detection and driver recommendation | Rust | System | 🔲 Planned | hwdetect, mhwd (Manjaro) |
| sigma-usb | sigma-usb | USB device management, thunderbolt, USB4 | Rust | System | 🔲 Planned | usbguard, bolt |

---

## Build & Toolchain

| Component Name | Repo | Description | Language | Layer | Status | Upstream Inspiration |
|----------------|------|-------------|----------|-------|--------|---------------------|
| sigma-build | sigma-build | Meta build system orchestrating all 149 components | Python, Rust | Build | 🚧 In Progress | Yocto, Buildroot |
| sigma-ci | sigma-ci | CI/CD pipeline definitions for all components | YAML, Shell | Build | 🔨 Scaffolded | GitHub Actions, GitLab CI |
| sigma-cross-compile | sigma-cross-compile | Cross-compilation toolchain (x86_64 → ARM64) | Shell, Rust | Build | 🔲 Planned | crosstool-ng, LLVM cross |
| sigma-iso | sigma-iso | ISO image build system and live environment | Shell, Python | Build | 🔨 Scaffolded | live-build, mkarchiso |
| sigma-installer-build | sigma-installer-build | Installer image build automation | Shell, Python | Build | 🔲 Planned | Calamares, Ubiquity |
| sigma-signing | sigma-signing | Release signing infrastructure (GPG, Sigstore) | Rust, Shell | Build | 🔲 Planned | sigstore/cosign |
| sigma-sbom | sigma-sbom | Software Bill of Materials generation | Rust, Python | Build | 🔲 Planned | cyclonedx, SPDX |
| sigma-reproducible | sigma-reproducible | Reproducible build verification tooling | Rust, Shell | Build | 🔲 Planned | reproducible-builds.org |
| sigma-release | sigma-release | Release engineering scripts and automation | Python, Shell | Build | 🔲 Planned | Linux Mint release tooling |
| sigma-mirror | sigma-mirror | Repository mirror management and CDN configuration | Python | Build | 🔲 Planned | rsync mirrors, Linux Mint mirrors |

---

## Developer Tools

| Component Name | Repo | Description | Language | Layer | Status | Upstream Inspiration |
|----------------|------|-------------|----------|-------|--------|---------------------|
| sigma-ctl | sigma-ctl | System control CLI for all sigma daemons | Rust | System | 🔨 Scaffolded | systemctl, nmcli |
| sigma-debug | sigma-debug | Debug tools, coredump manager, crash reporter | Rust | System | 🔲 Planned | coredumpctl, apport |
| sigma-perf | sigma-perf | Performance profiling integration (perf, eBPF) | Rust | System | 🔲 Planned | perf, flamegraph |
| sigma-test-runner | sigma-test-runner | Integration test runner for SigmaOS components | Rust | Build | 🔨 Scaffolded | pytest, cargo test |
| sigma-vm | sigma-vm | SigmaOS VM image for development/testing | Shell | Build | 🔲 Planned | QEMU, virt-manager |
| sigma-dev-container | sigma-dev-container | Development container with full SigmaOS toolchain | Shell | Build | 🔲 Planned | dev containers |
| sigma-docs | sigma-docs | Documentation build system (mdBook) | Rust, Markdown | Build | 🔨 Scaffolded | mdBook, Sphinx |
| sigma-changelog | sigma-changelog | Automated changelog generation from commit history | Rust | Build | 🔲 Planned | git-cliff, conventional commits |
| sigma-lint | sigma-lint | Linting and formatting enforcement for all languages | Shell, Rust | Build | 🔨 Scaffolded | clippy, pylint, shellcheck |

---

## Common Libraries

| Component Name | Repo | Description | Language | Layer | Status | Upstream Inspiration |
|----------------|------|-------------|----------|-------|--------|---------------------|
| sigma-common | sigma-common | Shared utilities: logging, config parsing, error types | Rust | Library | 🚧 In Progress | — |
| sigma-types | sigma-types | Shared type definitions across all components | Rust | Library | 🚧 In Progress | — |
| sigma-error | sigma-error | Error type hierarchy and reporting | Rust | Library | 🔨 Scaffolded | thiserror, anyhow |
| sigma-async | sigma-async | Async runtime helpers and utilities | Rust | Library | 🔲 Planned | Tokio, async-std |
| sigma-dbus-client | sigma-dbus-client | D-Bus client library for sigma services | Rust | Library | 🔲 Planned | zbus |
| sigma-config | sigma-config | Configuration management library (TOML + env) | Rust | Library | 🔨 Scaffolded | config-rs, figment |
| sigma-crypto-lib | sigma-crypto-lib | Cryptographic primitives (hashing, signing, TLS) | Rust | Library | 🔲 Planned | ring, rustls |
| sigma-ipc-lib | sigma-ipc-lib | IPC client/server library for Unix sockets | Rust | Library | 🔲 Planned | tarpc |
| sigma-test-utils | sigma-test-utils | Test utilities, mocks, and fixtures for sigma components | Rust | Library | 🔨 Scaffolded | mockall, proptest |
| sigma-ui-toolkit | sigma-ui-toolkit | Shared UI components and widget library (GTK4) | Rust | Library | 🔲 Planned | gtk4-rs, libadwaita |

---

## Summary Statistics

| Layer | Count | Stable | In Progress | Planned |
|-------|-------|--------|-------------|---------|
| Firmware & Boot | 8 | 0 | 2 | 6 |
| Kernel | 14 | 0 | 1 | 13 |
| System Services | 22 | 0 | 3 | 19 |
| Desktop Environment | 18 | 0 | 2 | 16 |
| Core Applications | 20 | 0 | 0 | 20 |
| AI & Intelligence | 8 | 0 | 0 | 8 |
| Security | 10 | 0 | 0 | 10 |
| Package & Update | 8 | 0 | 1 | 7 |
| Hardware Abstraction | 12 | 0 | 0 | 12 |
| Build & Toolchain | 10 | 0 | 3 | 7 |
| Developer Tools | 9 | 0 | 4 | 5 |
| Common Libraries | 10 | 0 | 4 | 6 |
| **Total** | **149** | **0** | **20** | **129** |

*Status reflects v0.1 Alpha state (November 2026)*
