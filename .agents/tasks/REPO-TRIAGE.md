# SigmaOS Repo Triage

> **Phase 1 — Architecture Overhaul**
> Source: `COMPONENTS.toml` (1,591 unique names) + scaffold dirs `phase3-desktop/` and `phase4-components/`.
> All mandatory merge rules from the architecture overhaul plan are applied.

## Classification Key

| Label | Meaning |
|-------|---------|
| **Keep active and production** | ~30 canonical active repos; every release depends on these |
| **Keep active but optional** | Extension, plugin, or app repos; shipped but not on the critical path |
| **Prototype / optional** | Experimental; not yet needed for a release |
| **Merge into canonical repo** | Duplicate; history migrated, repo archived after merge |
| **Archive** | Orphan, redundant, or no active development |

---

## Mandatory Merge Rules (applied throughout the table)

| Source repos | Canonical target |
|---|---|
| SigmaOS-gtk, SigmaOS-GTK-plus | → `SigmaOS-GTK` |
| SigmaOS-theme-registry, SigmaOS-theme-marketplace | → `SigmaOS-theme-registry` |
| SigmaOS-control-center, SigmaOS-settings, SigmaOS-daemon-settings | → `SigmaOS-control-center` |
| SigmaOS-file-manager, SigmaOS-file-manager-system, SigmaOS-file-writer | → `SigmaOS-file-manager-system` |
| SigmaOS-system, SigmaOS-systemd, SigmaOS-rc, SigmaOS-service-manager | → `SigmaOS-system` (others become adapters or archived) |
| SigmaOS-packages, SigmaOS-packages-kit, SigmaOS-packages-tools | → `SigmaOS-package-manager` |

---

## Triage Table

| Repo Name | Classification | Rationale |
|-----------|---------------|-----------|
| **— Core platform (Keep active and production) —** | | |
| SigmaOS-kernel | Keep active and production | Microkernel core; every component depends on it |
| SigmaOS-core | Keep active and production | Base runtime lifecycle, APIs, service discovery |
| SigmaOS-boot | Keep active and production | Boot stages, grub handoff, boot profile |
| SigmaOS-initramfs | Keep active and production | Early-boot ramdisk; required for installer and live session |
| SigmaOS-installer | Keep active and production | Primary OS installer; essential release artifact |
| SigmaOS-filesystem | Keep active and production | VFS layer; depended on by kernel and core |
| SigmaOS-system | Keep active and production | Canonical service orchestration (absorbs systemd, rc, service-manager adapters) |
| SigmaOS-security | Keep active and production | Privilege model, LSM, secure defaults |
| SigmaOS-auth | Keep active and production | Login policy, PAM-compat, session auth |
| SigmaOS-network | Keep active and production | TCP/IP stack, TLS, mDNS |
| SigmaOS-drivers | Keep active and production | Driver model and hardware ABI |
| SigmaOS-hardware | Keep active and production | Hardware abstraction, compatibility policy |
| SigmaOS-display-manager | Keep active and production | Login/session selection, greeter host |
| SigmaOS-session-manager | Keep active and production | Session lifecycle, logout/restart/shutdown |
| SigmaOS-compositor | Keep active and production | Wayland/X compositor (scaffold: phase3-desktop) |
| SigmaOS-window-manager | Keep active and production | Window management, tiling/floating policy (scaffold: phase3-desktop) |
| SigmaOS-desktop | Keep active and production | Desktop shell entry point; coordinates panel, launcher, compositor |
| SigmaOS-panel | Keep active and production | Taskbar/panel; top-level desktop UI surface |
| SigmaOS-launcher | Keep active and production | App launcher and search (phase4-components scaffold) |
| SigmaOS-control-center | Keep active and production | **Canonical** — absorbs SigmaOS-settings and SigmaOS-daemon-settings |
| SigmaOS-terminal | Keep active and production | Default terminal emulator (scaffold: phase3-desktop) |
| SigmaOS-package-manager | Keep active and production | **Canonical** — absorbs SigmaOS-packages, SigmaOS-packages-kit, SigmaOS-packages-tools |
| SigmaOS-repo-manager | Keep active and production | Manages software source repositories |
| SigmaOS-update | Keep active and production | System update orchestration |
| SigmaOS-theme | Keep active and production | Core theming engine entry point |
| SigmaOS-docs | Keep active and production | Canonical developer and user documentation |
| SigmaOS-release | Keep active and production | Release tooling, ISO build, signing |
| SigmaOS-GTK | Keep active and production | **Canonical GTK layer** — absorbs SigmaOS-gtk and SigmaOS-GTK-plus |
| SigmaOS-theme-registry | Keep active and production | **Canonical theme registry** — absorbs SigmaOS-theme-marketplace |
| SigmaOS-file-manager-system | Keep active and production | **Canonical file manager** — absorbs SigmaOS-file-manager and SigmaOS-file-writer |
| **— CachyOS-performance repos (Keep active and production) —** | | |
| SigmaOS-performance | Keep active and production | Performance subsystem entry point |
| SigmaOS-kernel-tuning | Keep active and production | Kernel-level tuning parameters and sysctl profiles |
| SigmaOS-zram | Keep active and production | Compressed RAM swap configuration |
| SigmaOS-scheduler | Keep active and production | CPU scheduler policy (CFS/BORE/etc.) |
| SigmaOS-cpu-profile | Keep active and production | CPU power and frequency profiles |
| SigmaOS-io-scheduler | Keep active and production | Block I/O scheduler selection and tuning |
| SigmaOS-fs-tuning | Keep active and production | Filesystem mount options and buffer tuning |
| SigmaOS-gaming-profile | Keep active and production | Gaming-optimized OS profile (latency, GPU priority) |
| SigmaOS-gpu | Keep active and production | GPU driver integration and selection |
| SigmaOS-vulkan | Keep active and production | Vulkan ICD loader and layer management |
| SigmaOS-swap | Keep active and production | Swap policy and swappiness tuning |
| SigmaOS-power-policy | Keep active and production | Power profiles (performance / balanced / saver) |
| **— Keep active but optional —** | | |
| SigmaOS-accessibility | Keep active but optional | Accessibility layer; important but ships separately |
| SigmaOS-actions | Keep active but optional | Desktop actions/shortcuts framework |
| SigmaOS-app-framework | Keep active but optional | Shared application framework (scaffold: phase3-desktop) |
| SigmaOS-audio | Keep active but optional | Audio subsystem (PipeWire/PulseAudio integration) |
| SigmaOS-audio-tuner | Keep active but optional | Audio tuning presets and EQ profiles |
| SigmaOS-automate | Keep active but optional | Automation and scripting framework |
| SigmaOS-backup-tool | Keep active but optional | User-facing backup utility |
| SigmaOS-bluetooth | Keep active but optional | Bluetooth stack integration |
| SigmaOS-bluetooth-tool | Keep active but optional | Bluetooth pairing and management GUI |
| SigmaOS-bootloader | Keep active but optional | GRUB/systemd-boot configuration tool |
| SigmaOS-brand-logo | Keep active but optional | Official brand assets, logo, and splash artwork |
| SigmaOS-calculator | Keep active but optional | Desktop calculator application |
| SigmaOS-chatroom | Keep active but optional | Chat/messaging client |
| SigmaOS-chromium | Keep active but optional | Chromium browser packaging |
| SigmaOS-clock-global | Keep active but optional | World clock and time-zone widget |
| SigmaOS-common | Keep active but optional | Shared utility library used across apps |
| SigmaOS-configurator | Keep active but optional | Low-level system configurator tool |
| SigmaOS-desktop-portal | Keep active but optional | XDG desktop portal for sandboxed apps |
| SigmaOS-dev-tools | Keep active but optional | Developer toolchain meta-package |
| SigmaOS-disk-utility | Keep active but optional | GUI disk partitioning and format tool |
| SigmaOS-domain-blocker | Keep active but optional | DNS-level ad/tracker blocker |
| SigmaOS-dots-configuration | Keep active but optional | Dotfile management and distribution |
| SigmaOS-draw | Keep active but optional | Vector drawing application |
| SigmaOS-extension-manager | Keep active but optional | Desktop extension install/manage UI |
| SigmaOS-firewall | Keep active but optional | Firewall rules and GUI (nftables/iptables) |
| SigmaOS-fish | Keep active but optional | Fish shell packaging and config |
| SigmaOS-folder-color-switcher | Keep active but optional | Folder color customization utility |
| SigmaOS-font-manager | Keep active but optional | Font installation and preview |
| SigmaOS-gaming | Keep active but optional | Gaming meta-package (Steam, Proton, etc.) |
| SigmaOS-github-action | Keep active but optional | GitHub Actions workflow templates |
| SigmaOS-greeter | Keep active but optional | Login screen greeter UI |
| SigmaOS-grub | Keep active but optional | GRUB2 packaging and default config |
| SigmaOS-grub-themes | Keep active but optional | GRUB theme collection |
| SigmaOS-icon-theme | Keep active but optional | Default icon theme |
| SigmaOS-image-viewer | Keep active but optional | Default image viewer app |
| SigmaOS-init | Keep active but optional | Init system configuration and startup scripts |
| SigmaOS-ISO | Keep active but optional | Live ISO build recipes |
| SigmaOS-JS-interpreter | Keep active but optional | Embedded JS engine for shell scripts/extensions |
| SigmaOS-LAN-fileshare | Keep active but optional | LAN file sharing tool |
| SigmaOS-libappindicator | Keep active but optional | App indicator support library |
| SigmaOS-live-session | Keep active but optional | Live USB/session management |
| SigmaOS-mac-run | Keep active but optional | macOS app compatibility layer |
| SigmaOS-menus | Keep active but optional | Application menu management |
| SigmaOS-mesa3D | Keep active but optional | Mesa/OpenGL packaging |
| SigmaOS-mirror | Keep active but optional | Mirror management and selection tool |
| SigmaOS-mozjs | Keep active but optional | Mozilla SpiderMonkey JS library packaging |
| SigmaOS-network-discovery | Keep active but optional | Network device discovery service |
| SigmaOS-nodejs-binary | Keep active but optional | Bundled Node.js binary for tooling |
| SigmaOS-notification | Keep active but optional | Notification daemon and API |
| SigmaOS-notification-center-plugin | Keep active but optional | Notification center panel plugin |
| SigmaOS-NVIDIA | Keep active but optional | NVIDIA proprietary driver packaging |
| SigmaOS-nvim | Keep active but optional | Neovim packaging and default config |
| SigmaOS-office | Keep active but optional | Office suite meta-package |
| SigmaOS-photo-viewer | Keep active but optional | Photo viewer and gallery app |
| SigmaOS-player | Keep active but optional | Media player application |
| SigmaOS-player-library | Keep active but optional | Shared media playback library |
| SigmaOS-plugin-marketplace | Keep active but optional | Plugin discovery and install UI |
| SigmaOS-plugin-registry | Keep active but optional | Plugin registry and metadata store |
| SigmaOS-plymouth | Keep active but optional | Boot splash screen (Plymouth) |
| SigmaOS-power-manager | Keep active but optional | Power management daemon and tray app |
| SigmaOS-quickshell | Keep active but optional | Quick shell/command bar widget |
| SigmaOS-radio-stream | Keep active but optional | Internet radio streaming app |
| SigmaOS-reel | Keep active but optional | Video reel/slideshow tool |
| SigmaOS-remote-desktop | Keep active but optional | Remote desktop (RDP/VNC) client |
| SigmaOS-renamer-bulk | Keep active but optional | Bulk file rename utility |
| SigmaOS-renamer-file | Keep active but optional | Single-file rename utility |
| SigmaOS-screen-recorder | Keep active but optional | Screen recording tool |
| SigmaOS-screen-saver | Keep active but optional | Screensaver daemon and themes |
| SigmaOS-screen-shot | Keep active but optional | Screenshot tool |
| SigmaOS-setup | Keep active but optional | First-run setup wizard |
| SigmaOS-sign | Keep active but optional | Package signing tooling |
| SigmaOS-simple-scan | Keep active but optional | Scanner frontend |
| SigmaOS-slide-presenter | Keep active but optional | Presentation viewer |
| SigmaOS-software-manager | Keep active but optional | GUI software center |
| SigmaOS-spreadsheet | Keep active but optional | Spreadsheet application |
| SigmaOS-ssh-port-forward-plugin | Keep active but optional | SSH tunnel/port-forward GUI plugin |
| SigmaOS-state-of-components | Keep active but optional | Component health dashboard |
| SigmaOS-stick | Keep active but optional | Sticky note app |
| SigmaOS-sticky-notes | Keep active but optional | Alternative sticky-notes widget |
| SigmaOS-stream | Keep active but optional | Streaming/cast integration |
| SigmaOS-system-admin | Keep active but optional | Admin toolkit meta-package |
| SigmaOS-system-locale | Keep active but optional | Locale and i18n management |
| SigmaOS-term | Keep active but optional | Alternative terminal shorthand package |
| SigmaOS-terminal-text-effects | Keep active but optional | Terminal text-effects library (TTE integration) |
| SigmaOS-text-editor | Keep active but optional | Default text editor (scaffold: phase3-desktop) |
| SigmaOS-theme-engine | Keep active but optional | Theming engine library (scaffold: phase3-desktop) |
| SigmaOS-timeshift | Keep active but optional | System snapshot and restore (Timeshift) |
| SigmaOS-tray-icon | Keep active but optional | System tray icon framework |
| SigmaOS-troubleshooting-tool | Keep active but optional | User-facing diagnostics and log viewer |
| SigmaOS-upload | Keep active but optional | File upload/share tool |
| SigmaOS-upgrade | Keep active but optional | Major-version upgrade orchestration |
| SigmaOS-upgrade-info | Keep active but optional | Release notes and upgrade info UI |
| SigmaOS-vim | Keep active but optional | Vim packaging and default config |
| SigmaOS-vpn-pia | Keep active but optional | PIA VPN client integration |
| SigmaOS-wallpaper-engine | Keep active but optional | Animated/live wallpaper engine |
| SigmaOS-webapp-manager | Keep active but optional | Web app shortcut manager |
| SigmaOS-webcam-recorder | Keep active but optional | Webcam capture tool |
| SigmaOS-welcome-screen | Keep active but optional | First-boot welcome application |
| SigmaOS-wifi | Keep active but optional | Wi-Fi management UI |
| SigmaOS-window-management-library | Keep active but optional | Shared window management library (scaffold: phase4) |
| SigmaOS-wine-installer | Keep active but optional | Wine/Proton installation helper |
| SigmaOS-xapp | Keep active but optional | XApp shared library (Linux Mint-style cross-DE apps) |
| SigmaOS-xapp-python | Keep active but optional | Python bindings for XApp |
| SigmaOS-xapp-thumbnailers | Keep active but optional | XApp file thumbnailer plugins |
| SigmaOS-xorg-server | Keep active but optional | Xorg server packaging (X11 fallback) |
| SigmaOS-xreader | Keep active but optional | Document/PDF reader (XReader fork) |
| SigmaOS-zsh | Keep active but optional | Zsh shell packaging and config |
| SigmaOS-desktop-configuration | Keep active but optional | Desktop profile config files (phase4) |
| SigmaOS-desktop-environment | Keep active but optional | Desktop environment meta-package (phase4) |
| SigmaOS-desktop-library | Keep active but optional | Shared desktop library (phase4) |
| SigmaOS-desktop-session | Keep active but optional | Desktop session startup scripts |
| SigmaOS-asdcontrol-git | Keep active but optional | ASD brightness/backlight control |
| SigmaOS-apturl | Keep active but optional | apt:// URL handler for browser integration |
| SigmaOS-daemon-apt | Keep active but optional | APT daemon helper |
| SigmaOS-display-integration | Keep active but optional | Display output integration (multi-monitor) |
| SigmaOS-herdr-runtime | Keep active but optional | Herdr live-coding agent runtime |
| SigmaOS-indicator-sound | Keep active but optional | Volume/sound indicator panel applet |
| SigmaOS-libappindicator | Keep active but optional | App indicator shared library |
| SigmaOS-nvidia-prime-applet | Keep active but optional | NVIDIA PRIME GPU switch applet |
| SigmaOS-pix | Keep active but optional | Photo organizer (Pix fork) |
| SigmaOS-plymouth | Keep active but optional | Plymouth boot splash |
| SigmaOS-printing | Keep active but optional | CUPS printing integration |
| SigmaOS-pyRenamer | Keep active but optional | Python-based file renamer |
| SigmaOS-quickpanel | Keep active but optional | Quick-settings panel widget |
| SigmaOS-translations | Keep active but optional | Shared translation strings |
| SigmaOS-try | Keep active but optional | Try-before-install sandbox session |
| SigmaOS-try-windows | Keep active but optional | Windows-app trial sandbox |
| SigmaOS-tte-go | Keep active but optional | Go port of terminal text effects |
| SigmaOS-video-cut | Keep active but optional | Video trimming/cutting tool |
| SigmaOS-video-converter | Keep active but optional | Video format conversion tool |
| SigmaOS-wallpaper-package | Keep active but optional | Curated wallpaper collection |
| SigmaOS-wallpaper-themes | Keep active but optional | Themed wallpaper sets |
| SigmaOS-we-can-fix-everything | Keep active but optional | Community repair/fix utility meta-package |
| SigmaOS-wiki-extra | Keep active but optional | Extra wiki content and templates |
| SigmaOS-workstation | Keep active but optional | Workstation profile meta-package |
| SigmaOS-website | Keep active but optional | SigmaOS public website source |
| SigmaOS-forum-themes | Keep active but optional | Discourse/forum theme assets |
| SigmaOS-guide-developer | Keep active but optional | Developer guide documentation |
| SigmaOS-guide-docs | Keep active but optional | Documentation guide |
| SigmaOS-guide-installation | Keep active but optional | Installation guide |
| SigmaOS-guide-translations | Keep active but optional | Translation contributor guide |
| SigmaOS-guide-troubleshooting | Keep active but optional | Troubleshooting guide |
| SigmaOS-guide-user | Keep active but optional | End-user guide |
| **— Merge into canonical repo —** | | |
| SigmaOS-GTK+ | Merge into canonical repo | Variant name for SigmaOS-GTK; merge into `SigmaOS-GTK` |
| SigmaOS-gtk-plus | Merge into canonical repo | Duplicate of SigmaOS-GTK-plus; merge into `SigmaOS-GTK` |
| SigmaOS-GTK-plus | Merge into canonical repo | GTK enhancement layer; merge into `SigmaOS-GTK` per merge rules |
| SigmaOS-theme-marketplace | Merge into canonical repo | Duplicate marketplace; merge into `SigmaOS-theme-registry` |
| SigmaOS-settings | Merge into canonical repo | User settings UI; merge into `SigmaOS-control-center` |
| SigmaOS-daemon-settings | Merge into canonical repo | Settings daemon; merge into `SigmaOS-control-center` |
| SigmaOS-file-manager | Merge into canonical repo | Duplicate FM; merge into `SigmaOS-file-manager-system` (scaffold: phase3-desktop) |
| SigmaOS-file-writer | Merge into canonical repo | File write helper; merge into `SigmaOS-file-manager-system` (phase4) |
| SigmaOS-packages | Merge into canonical repo | Package list; merge into `SigmaOS-package-manager` |
| SigmaOS-packages-kit | Merge into canonical repo | PackageKit adapter; merge into `SigmaOS-package-manager` (phase4) |
| SigmaOS-packages-tools | Merge into canonical repo | Package tooling; merge into `SigmaOS-package-manager` (phase4) |
| SigmaOS-systemd | Merge into canonical repo | systemd compat adapter; becomes adapter module under `SigmaOS-system` |
| SigmaOS-rc | Merge into canonical repo | BSD rc adapter; becomes adapter module under `SigmaOS-system` |
| SigmaOS-service-manager | Merge into canonical repo | Duplicate service manager; merge into `SigmaOS-system` |
| SigmaOS-update-manager | Merge into canonical repo | Redundant wrapper; merge into `SigmaOS-update` |
| SigmaOS-upgrade-legacy | Merge into canonical repo | Legacy upgrade path; merge into `SigmaOS-upgrade` |
| SigmaOS-upgrade-service | Merge into canonical repo | Upgrade service daemon; merge into `SigmaOS-upgrade` |
| SigmaOS-update-service | Merge into canonical repo | Update service daemon; merge into `SigmaOS-update` |
| SigmaOS-software-center | Merge into canonical repo | Duplicate of software-manager; merge into `SigmaOS-software-manager` |
| SigmaOS-session-manager2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-session-manager` |
| SigmaOS-security-core | Merge into canonical repo | Subset of security; merge into `SigmaOS-security` |
| SigmaOS-security-audit | Merge into canonical repo | Audit submodule; merge into `SigmaOS-security` |
| SigmaOS-disk | Merge into canonical repo | Duplicate of disk-utility; merge into `SigmaOS-disk-utility` |
| SigmaOS-driver | Merge into canonical repo | Duplicate of drivers; merge into `SigmaOS-drivers` |
| SigmaOS-driver-manager | Merge into canonical repo | Duplicate driver manager; merge into `SigmaOS-drivers` |
| SigmaOS-network-manager | Merge into canonical repo | Duplicate NM wrapper; merge into `SigmaOS-network` |
| SigmaOS-network-manager-2 | Merge into canonical repo | Second duplicate NM wrapper; merge into `SigmaOS-network` |
| SigmaOS-networking | Merge into canonical repo | Alias for network stack; merge into `SigmaOS-network` |
| SigmaOS-network-stack | Merge into canonical repo | Redundant network-stack name; merge into `SigmaOS-network` |
| SigmaOS-audio-recorder | Merge into canonical repo | Subset of audio; merge into `SigmaOS-audio` |
| SigmaOS-audio-converter | Merge into canonical repo | Subset of audio; merge into `SigmaOS-audio` |
| SigmaOS-audio-effects | Merge into canonical repo | Subset of audio; merge into `SigmaOS-audio` |
| SigmaOS-audio-mixing | Merge into canonical repo | Subset of audio; merge into `SigmaOS-audio` |
| SigmaOS-video-player | Merge into canonical repo | Duplicate of player; merge into `SigmaOS-player` |
| SigmaOS-media-player | Merge into canonical repo | Duplicate of player; merge into `SigmaOS-player` |
| SigmaOS-screen-saver | Merge into canonical repo | Duplicate; merge into `SigmaOS-screen-saver` (screensaver-package alias) |
| SigmaOS-screensaver-package | Merge into canonical repo | Duplicate of screen-saver; merge into `SigmaOS-screen-saver` |
| SigmaOS-screenshot-tool | Merge into canonical repo | Duplicate of screen-shot; merge into `SigmaOS-screen-shot` |
| SigmaOS-draw2 | Merge into canonical repo | Duplicate of draw; merge into `SigmaOS-draw` |
| SigmaOS-drawing | Merge into canonical repo | Duplicate of draw; merge into `SigmaOS-draw` |
| SigmaOS-image-viewer-2 | Merge into canonical repo | Duplicate image viewer; merge into `SigmaOS-image-viewer` |
| SigmaOS-image-editor | Merge into canonical repo | Superset of image viewer; merge into `SigmaOS-image-viewer` |
| SigmaOS-image-resizer | Merge into canonical repo | Subset of image-viewer; merge into `SigmaOS-image-viewer` |
| SigmaOS-pdf | Merge into canonical repo | PDF viewer alias; merge into `SigmaOS-xreader` |
| SigmaOS-reader | Merge into canonical repo | Generic reader alias; merge into `SigmaOS-xreader` |
| SigmaOS-video-editing | Merge into canonical repo | Subset of video-cut; merge into `SigmaOS-video-cut` |
| SigmaOS-vide-cut | Merge into canonical repo | Typo variant of video-cut; merge into `SigmaOS-video-cut` |
| SigmaOS-repo-mirror | Merge into canonical repo | Duplicate mirror management; merge into `SigmaOS-repo-manager` |
| SigmaOS-repolib | Merge into canonical repo | Repository library; merge into `SigmaOS-repo-manager` |
| SigmaOS-grub-configurator | Merge into canonical repo | GRUB config GUI; merge into `SigmaOS-grub` |
| SigmaOS-grub-customizer | Merge into canonical repo | Duplicate GRUB customizer; merge into `SigmaOS-grub` |
| SigmaOS-grub-rescue | Merge into canonical repo | GRUB recovery tool; merge into `SigmaOS-grub` |
| SigmaOS-grub-themes-arch | Merge into canonical repo | Arch-specific GRUB themes; merge into `SigmaOS-grub-themes` |
| SigmaOS-gtk-themes | Merge into canonical repo | GTK theme collection; merge into `SigmaOS-GTK` |
| SigmaOS-icon-themes | Merge into canonical repo | Duplicate icon themes; merge into `SigmaOS-icon-theme` |
| SigmaOS-cursor-theme | Merge into canonical repo | Cursor theme; merge into `SigmaOS-theme` |
| SigmaOS-cursor-themes | Merge into canonical repo | Duplicate cursor themes; merge into `SigmaOS-theme` |
| SigmaOS-theme-color | Merge into canonical repo | Color scheme; merge into `SigmaOS-theme` |
| SigmaOS-theme-cursor | Merge into canonical repo | Cursor subset; merge into `SigmaOS-theme` |
| SigmaOS-theme-desktop-environment | Merge into canonical repo | DE theme; merge into `SigmaOS-theme` |
| SigmaOS-theme-gtk3 | Merge into canonical repo | GTK3 theme; merge into `SigmaOS-GTK` |
| SigmaOS-theme-icon | Merge into canonical repo | Icon theme subset; merge into `SigmaOS-icon-theme` |
| SigmaOS-theme-manager | Merge into canonical repo | Theme manager UI; merge into `SigmaOS-theme-registry` |
| SigmaOS-theme-mdm | Merge into canonical repo | MDM login theme; merge into `SigmaOS-display-manager` |
| SigmaOS-theme-support | Merge into canonical repo | Theme support library; merge into `SigmaOS-theme-engine` |
| SigmaOS-theme-sync | Merge into canonical repo | Theme sync tool; merge into `SigmaOS-theme-registry` |
| SigmaOS-theme-y | Merge into canonical repo | Unnamed theme variant; merge into `SigmaOS-theme` |
| SigmaOS-theming-native | Merge into canonical repo | Native theming layer; merge into `SigmaOS-GTK` |
| SigmaOS-firewall-manager | Merge into canonical repo | Duplicate firewall GUI; merge into `SigmaOS-firewall` |
| SigmaOS-daemon | Merge into canonical repo | Generic daemon; merge into `SigmaOS-system` |
| SigmaOS-daemon-manager | Merge into canonical repo | Daemon manager; merge into `SigmaOS-system` |
| SigmaOS-daemon-service | Merge into canonical repo | Daemon service helper; merge into `SigmaOS-system` |
| SigmaOS-core-daemon | Merge into canonical repo | Core daemon; merge into `SigmaOS-core` |
| SigmaOS-core-daemon-2 | Merge into canonical repo | Duplicate core daemon; merge into `SigmaOS-core` |
| SigmaOS-core-daemon-service | Merge into canonical repo | Core daemon service; merge into `SigmaOS-core` |
| SigmaOS-core-daemon-service-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-core` |
| SigmaOS-core-daemon-service-3 | Merge into canonical repo | Duplicate; merge into `SigmaOS-core` |
| SigmaOS-core-daemons | Merge into canonical repo | Duplicate; merge into `SigmaOS-core` |
| SigmaOS-core-service | Merge into canonical repo | Core service; merge into `SigmaOS-core` |
| SigmaOS-core-service-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-core` |
| SigmaOS-core-service-3 | Merge into canonical repo | Duplicate; merge into `SigmaOS-core` |
| SigmaOS-core-service-4 | Merge into canonical repo | Duplicate; merge into `SigmaOS-core` |
| SigmaOS-core-services | Merge into canonical repo | Plural alias; merge into `SigmaOS-core` |
| SigmaOS-init-scripts | Merge into canonical repo | Init scripts; merge into `SigmaOS-init` |
| SigmaOS-init-service | Merge into canonical repo | Init service helper; merge into `SigmaOS-init` |
| SigmaOS-init-system | Merge into canonical repo | Init system abstraction; merge into `SigmaOS-init` |
| SigmaOS-policy | Merge into canonical repo | Policy definitions; merge into `SigmaOS-security` |
| SigmaOS-polkit | Merge into canonical repo | Polkit integration; merge into `SigmaOS-auth` |
| SigmaOS-install2 | Merge into canonical repo | Duplicate installer; merge into `SigmaOS-installer` |
| SigmaOS-live-installer | Merge into canonical repo | Live-session installer; merge into `SigmaOS-installer` |
| SigmaOS-partition-manager | Merge into canonical repo | Partition tool; merge into `SigmaOS-disk-utility` |
| SigmaOS-efi-manager | Merge into canonical repo | EFI/UEFI management; merge into `SigmaOS-boot` |
| SigmaOS-boot-config | Merge into canonical repo | Boot config alias; merge into `SigmaOS-boot` |
| SigmaOS-boot_config | Merge into canonical repo | Boot config snake-case alias; merge into `SigmaOS-boot` |
| SigmaOS-boot-manager | Merge into canonical repo | Boot manager GUI; merge into `SigmaOS-boot` |
| SigmaOS-boot-service | Merge into canonical repo | Boot service; merge into `SigmaOS-boot` |
| SigmaOS-startup-manager | Merge into canonical repo | Startup app manager; merge into `SigmaOS-session-manager` |
| SigmaOS-desktop-core | Merge into canonical repo | Duplicate desktop core; merge into `SigmaOS-desktop` |
| SigmaOS-desktop-effects | Merge into canonical repo | Desktop effects; merge into `SigmaOS-compositor` |
| SigmaOS-panel-manager | Merge into canonical repo | Panel management; merge into `SigmaOS-panel` |
| SigmaOS-taskbar-manager | Merge into canonical repo | Taskbar variant; merge into `SigmaOS-panel` |
| SigmaOS-dock-manager | Merge into canonical repo | Dock management; merge into `SigmaOS-panel` |
| SigmaOS-desklet-manager | Merge into canonical repo | Desklet manager; merge into `SigmaOS-desktop` |
| SigmaOS-btrfs | Merge into canonical repo | Btrfs-specific tooling; merge into `SigmaOS-filesystem` |
| SigmaOS-fs | Merge into canonical repo | FS alias; merge into `SigmaOS-filesystem` |
| SigmaOS-kernel-module | Merge into canonical repo | Kernel module packaging; merge into `SigmaOS-kernel` |
| SigmaOS-kernel-headers | Merge into canonical repo | Kernel headers; merge into `SigmaOS-kernel` |
| SigmaOS-kernel-optimization | Merge into canonical repo | Kernel optimization; merge into `SigmaOS-kernel-tuning` |
| SigmaOS-kernel-apis | Merge into canonical repo | Kernel API bindings; merge into `SigmaOS-kernel` |
| SigmaOS-pipewire | Merge into canonical repo | PipeWire audio daemon; merge into `SigmaOS-audio` |
| SigmaOS-indicator-sound2 | Merge into canonical repo | Duplicate sound indicator; merge into `SigmaOS-indicator-sound` |
| SigmaOS-clipboard | Merge into canonical repo | Clipboard manager; merge into `SigmaOS-desktop` |
| SigmaOS-color-picker | Merge into canonical repo | Color picker tool; merge into `SigmaOS-draw` |
| SigmaOS-cropper | Merge into canonical repo | Image crop tool; merge into `SigmaOS-image-viewer` |
| SigmaOS-gradient-generator | Merge into canonical repo | Gradient generator; merge into `SigmaOS-draw` |
| SigmaOS-pattern-generator | Merge into canonical repo | Pattern generator; merge into `SigmaOS-draw` |
| SigmaOS-texture-generator | Merge into canonical repo | Texture generator; merge into `SigmaOS-draw` |
| SigmaOS-memory | Merge into canonical repo | Memory management; merge into `SigmaOS-kernel` |
| SigmaOS-memory-manager | Merge into canonical repo | Memory manager daemon; merge into `SigmaOS-kernel` |
| SigmaOS-process | Merge into canonical repo | Process management; merge into `SigmaOS-system` |
| SigmaOS-process-manager | Merge into canonical repo | Process manager; merge into `SigmaOS-system` |
| SigmaOS-process-controller | Merge into canonical repo | Process controller; merge into `SigmaOS-system` |
| SigmaOS-processor | Merge into canonical repo | CPU processor abstraction; merge into `SigmaOS-system` |
| SigmaOS-lightdm-settings | Merge into canonical repo | LightDM settings; merge into `SigmaOS-display-manager` |
| SigmaOS-cinnamon | Merge into canonical repo | Cinnamon DE packaging; merge into `SigmaOS-desktop` |
| SigmaOS-cinnamon-bluetooth | Merge into canonical repo | Cinnamon BT applet; merge into `SigmaOS-bluetooth` |
| SigmaOS-cinnamon-control-center | Merge into canonical repo | Cinnamon control center; merge into `SigmaOS-control-center` |
| SigmaOS-cinnamon-desktop | Merge into canonical repo | Cinnamon desktop base; merge into `SigmaOS-desktop` |
| SigmaOS-cinnamon-menus | Merge into canonical repo | Cinnamon menus; merge into `SigmaOS-menus` |
| SigmaOS-cinnamon-screensaver | Merge into canonical repo | Cinnamon screensaver; merge into `SigmaOS-screen-saver` |
| SigmaOS-cinnamon-session | Merge into canonical repo | Cinnamon session; merge into `SigmaOS-session-manager` |
| SigmaOS-cinnamon-settings-daemon | Merge into canonical repo | Cinnamon settings daemon; merge into `SigmaOS-control-center` |
| SigmaOS-cinnamon-spices-actions | Merge into canonical repo | Cinnamon spices actions; merge into `SigmaOS-actions` |
| SigmaOS-cinnamon-spices-applets | Merge into canonical repo | Cinnamon applets; merge into `SigmaOS-panel` |
| SigmaOS-cinnamon-spices-desklets | Merge into canonical repo | Cinnamon desklets; merge into `SigmaOS-desktop` |
| SigmaOS-cinnamon-spices-extensions | Merge into canonical repo | Cinnamon extensions; merge into `SigmaOS-extension-manager` |
| SigmaOS-cinnamon-themes | Merge into canonical repo | Cinnamon themes; merge into `SigmaOS-theme` |
| SigmaOS-cjs | Merge into canonical repo | Cinnamon JS; merge into `SigmaOS-JS-interpreter` |
| SigmaOS-nemo | Merge into canonical repo | Nemo file manager; merge into `SigmaOS-file-manager-system` |
| SigmaOS-nemo-extensions | Merge into canonical repo | Nemo extensions; merge into `SigmaOS-file-manager-system` |
| SigmaOS-mintcommon | Merge into canonical repo | Mint common library; merge into `SigmaOS-common` |
| SigmaOS-mintinstall | Merge into canonical repo | Mint software manager; merge into `SigmaOS-software-manager` |
| SigmaOS-mintupdate | Merge into canonical repo | Mint update manager; merge into `SigmaOS-update` |
| SigmaOS-mintsources | Merge into canonical repo | Mint sources manager; merge into `SigmaOS-repo-manager` |
| SigmaOS-mintbackup | Merge into canonical repo | Mint backup tool; merge into `SigmaOS-backup-tool` |
| SigmaOS-mintdrivers | Merge into canonical repo | Mint driver manager; merge into `SigmaOS-drivers` |
| SigmaOS-mintnanny | Merge into canonical repo | Mint parental control; merge into `SigmaOS-domain-blocker` |
| SigmaOS-mintdesktop | Merge into canonical repo | Mint desktop settings; merge into `SigmaOS-desktop` |
| SigmaOS-mintlocale | Merge into canonical repo | Mint locale manager; merge into `SigmaOS-system-locale` |
| SigmaOS-mintstick | Merge into canonical repo | Mint USB writer; merge into `SigmaOS-stick` |
| SigmaOS-libreoffice-style-mint | Merge into canonical repo | LibreOffice theme; merge into `SigmaOS-office` |
| SigmaOS-bulky | Merge into canonical repo | Bulky file renamer; merge into `SigmaOS-renamer-bulk` |
| SigmaOS-bulk-renamer | Merge into canonical repo | Duplicate bulk renamer; merge into `SigmaOS-renamer-bulk` |
| SigmaOS-celluloid | Merge into canonical repo | Celluloid video player; merge into `SigmaOS-player` |
| SigmaOS-gdebi | Merge into canonical repo | .deb installer GUI; merge into `SigmaOS-software-manager` |
| SigmaOS-blueman | Merge into canonical repo | Blueman BT manager; merge into `SigmaOS-bluetooth-tool` |
| SigmaOS-blueferry | Merge into canonical repo | Bluetooth file transfer; merge into `SigmaOS-bluetooth-tool` |
| SigmaOS-bluez | Merge into canonical repo | BlueZ library packaging; merge into `SigmaOS-bluetooth` |
| SigmaOS-dunst | Merge into canonical repo | Dunst notification daemon; merge into `SigmaOS-notification` |
| SigmaOS-polybar | Merge into canonical repo | Polybar; merge into `SigmaOS-panel` |
| SigmaOS-bspwm | Merge into canonical repo | BSPWM; merge into `SigmaOS-window-manager` |
| SigmaOS-sway | Merge into canonical repo | Sway compositor; merge into `SigmaOS-compositor` |
| SigmaOS-conky | Merge into canonical repo | Conky monitor; merge into `SigmaOS-monitoring` |
| SigmaOS-fzf | Merge into canonical repo | fzf fuzzy finder; merge into `SigmaOS-dev-tools` |
| SigmaOS-ripgrep | Merge into canonical repo | ripgrep; merge into `SigmaOS-dev-tools` |
| SigmaOS-oh-my-zsh | Merge into canonical repo | Oh-my-zsh; merge into `SigmaOS-zsh` |
| SigmaOS-etcher | Merge into canonical repo | Balena Etcher USB writer; merge into `SigmaOS-stick` |
| SigmaOS-timeshift | Merge into canonical repo | Duplicate; already in optional as primary, remove this secondary |
| SigmaOS-audit | Merge into canonical repo | Audit tooling; merge into `SigmaOS-security` |
| SigmaOS-aur | Merge into canonical repo | AUR helper; merge into `SigmaOS-package-manager` |
| SigmaOS-pacman | Merge into canonical repo | Pacman front-end; merge into `SigmaOS-package-manager` |
| SigmaOS-pacman-alternatives | Merge into canonical repo | Pacman alternatives; merge into `SigmaOS-package-manager` |
| SigmaOS-pacman-color | Merge into canonical repo | Pacman color patch; merge into `SigmaOS-package-manager` |
| SigmaOS-pacman-contrib | Merge into canonical repo | Pacman contrib scripts; merge into `SigmaOS-package-manager` |
| SigmaOS-pacman-db-updater | Merge into canonical repo | Pacman DB updater; merge into `SigmaOS-package-manager` |
| SigmaOS-pacman-frontend | Merge into canonical repo | Pacman GUI; merge into `SigmaOS-software-manager` |
| SigmaOS-pacman-hooks | Merge into canonical repo | Pacman hooks; merge into `SigmaOS-package-manager` |
| SigmaOS-packagekit | Merge into canonical repo | PackageKit; merge into `SigmaOS-package-manager` |
| SigmaOS-aptdaemon | Merge into canonical repo | APT daemon; merge into `SigmaOS-package-manager` |
| SigmaOS-aptkit | Merge into canonical repo | APT kit; merge into `SigmaOS-package-manager` |
| SigmaOS-apt-mint | Merge into canonical repo | Mint APT integration; merge into `SigmaOS-package-manager` |
| SigmaOS-acpi | Merge into canonical repo | ACPI tooling; merge into `SigmaOS-power-manager` |
| SigmaOS-nightlight | Merge into canonical repo | Night light / blue light filter; merge into `SigmaOS-display-manager` |
| SigmaOS-startup-manager | Merge into canonical repo | Startup app manager; merge into `SigmaOS-session-manager` |
| SigmaOS-clipboard | Merge into canonical repo | Clipboard manager; merge into `SigmaOS-desktop` |
| SigmaOS-network-config | Merge into canonical repo | Network config files; merge into `SigmaOS-network` |
| SigmaOS-network-controller | Merge into canonical repo | Network controller; merge into `SigmaOS-network` |
| SigmaOS-network-interface | Merge into canonical repo | Network interface; merge into `SigmaOS-network` |
| SigmaOS-network-adapter | Merge into canonical repo | Network adapter; merge into `SigmaOS-network` |
| SigmaOS-network-protocol | Merge into canonical repo | Network protocol layer; merge into `SigmaOS-network` |
| SigmaOS-network-scripts | Merge into canonical repo | Network scripts; merge into `SigmaOS-network` |
| SigmaOS-network-tools | Merge into canonical repo | Network tools; merge into `SigmaOS-network` |
| SigmaOS-network-utility | Merge into canonical repo | Network utility; merge into `SigmaOS-network` |
| SigmaOS-network-utility-2 | Merge into canonical repo | Duplicate network utility; merge into `SigmaOS-network` |
| SigmaOS-network-integration | Merge into canonical repo | Network integration; merge into `SigmaOS-network` |
| SigmaOS-network-monitor | Merge into canonical repo | Network monitor; merge into `SigmaOS-network` |
| SigmaOS-bandwidth-monitor | Merge into canonical repo | Bandwidth monitor; merge into `SigmaOS-network` |
| SigmaOS-traffic-analyzer | Merge into canonical repo | Traffic analyzer; merge into `SigmaOS-network` |
| SigmaOS-connection | Merge into canonical repo | Connection manager alias; merge into `SigmaOS-network` |
| SigmaOS-connection-manager | Merge into canonical repo | Connection manager; merge into `SigmaOS-network` |
| SigmaOS-connector | Merge into canonical repo | Connector library; merge into `SigmaOS-network` |
| SigmaOS-vpn-pia | Merge into canonical repo | Already optional; if VPN is built in, merge into `SigmaOS-network` |
| SigmaOS-security-integration | Merge into canonical repo | Security integration; merge into `SigmaOS-security` |
| SigmaOS-security-monitor | Merge into canonical repo | Security monitor; merge into `SigmaOS-security` |
| SigmaOS-security-scripts | Merge into canonical repo | Security scripts; merge into `SigmaOS-security` |
| SigmaOS-security-tools | Merge into canonical repo | Security tools; merge into `SigmaOS-security` |
| SigmaOS-security-utility | Merge into canonical repo | Security utility; merge into `SigmaOS-security` |
| SigmaOS-security-utility-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-security` |
| SigmaOS-penetration-testing | Merge into canonical repo | Pentesting tools; merge into `SigmaOS-security` |
| SigmaOS-vulnerability-scanner | Merge into canonical repo | Vulnerability scanner; merge into `SigmaOS-security` |
| SigmaOS-antivirus | Merge into canonical repo | Antivirus integration; merge into `SigmaOS-security` |
| SigmaOS-crypto | Merge into canonical repo | Crypto primitives; merge into `SigmaOS-security` |
| SigmaOS-cryptography-tools | Merge into canonical repo | Crypto tools; merge into `SigmaOS-security` |
| SigmaOS-keyserver | Merge into canonical repo | Key server; merge into `SigmaOS-security` |
| SigmaOS-firmware | Merge into canonical repo | Firmware packaging; merge into `SigmaOS-drivers` |
| SigmaOS-peripheral | Merge into canonical repo | Peripheral driver support; merge into `SigmaOS-drivers` |
| SigmaOS-nvme | Merge into canonical repo | NVMe driver/tooling; merge into `SigmaOS-drivers` |
| SigmaOS-hdd | Merge into canonical repo | HDD support; merge into `SigmaOS-drivers` |
| SigmaOS-ssd | Merge into canonical repo | SSD support; merge into `SigmaOS-drivers` |
| SigmaOS-input | Merge into canonical repo | Input device layer; merge into `SigmaOS-drivers` |
| SigmaOS-input-integration | Merge into canonical repo | Input integration; merge into `SigmaOS-drivers` |
| SigmaOS-xorg-server | Merge into canonical repo | Already optional; if X11 backend built-in merge into `SigmaOS-compositor` |
| SigmaOS-display-scripts | Merge into canonical repo | Display scripts; merge into `SigmaOS-display-manager` |
| SigmaOS-display-utility | Merge into canonical repo | Display utility; merge into `SigmaOS-display-manager` |
| SigmaOS-display-utility-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-display-manager` |
| SigmaOS-display-integration | Merge into canonical repo | Already optional; if overlap merge into `SigmaOS-display-manager` |
| SigmaOS-output-integration | Merge into canonical repo | Output integration; merge into `SigmaOS-display-manager` |
| SigmaOS-output-scripts | Merge into canonical repo | Output scripts; merge into `SigmaOS-display-manager` |
| SigmaOS-output-utility | Merge into canonical repo | Output utility; merge into `SigmaOS-display-manager` |
| SigmaOS-output-utility-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-display-manager` |
| SigmaOS-storage | Merge into canonical repo | Storage layer; merge into `SigmaOS-filesystem` |
| SigmaOS-volume-manager | Merge into canonical repo | Volume management; merge into `SigmaOS-filesystem` |
| SigmaOS-drive | Merge into canonical repo | Drive abstraction; merge into `SigmaOS-filesystem` |
| SigmaOS-nvme | Merge into canonical repo | NVMe; merge into `SigmaOS-filesystem` |
| SigmaOS-io | Merge into canonical repo | I/O abstraction; merge into `SigmaOS-filesystem` |
| SigmaOS-ipc | Merge into canonical repo | IPC layer; merge into `SigmaOS-core` |
| SigmaOS-interrupt | Merge into canonical repo | Interrupt handling; merge into `SigmaOS-kernel` |
| SigmaOS-bus | Merge into canonical repo | System bus; merge into `SigmaOS-core` |
| SigmaOS-runtime | Merge into canonical repo | Runtime alias; merge into `SigmaOS-core` |
| SigmaOS-branding | Merge into canonical repo | Branding assets; merge into `SigmaOS-brand-logo` |
| SigmaOS-brand | Merge into canonical repo | Brand alias; merge into `SigmaOS-brand-logo` |
| SigmaOS-shell | Merge into canonical repo | Shell meta; merge into `SigmaOS-zsh` or `SigmaOS-fish` depending on default |
| SigmaOS-shell-core | Merge into canonical repo | Shell core; merge into default shell package |
| SigmaOS-shell-integration | Merge into canonical repo | Shell integration; merge into default shell package |
| SigmaOS-shell-scripts | Merge into canonical repo | Shell scripts; merge into `SigmaOS-common` |
| SigmaOS-shell-themes | Merge into canonical repo | Shell themes; merge into `SigmaOS-theme` |
| SigmaOS-terminal-integration | Merge into canonical repo | Terminal integration; merge into `SigmaOS-terminal` |
| SigmaOS-terminal-scripts | Merge into canonical repo | Terminal scripts; merge into `SigmaOS-terminal` |
| SigmaOS-terminal-themes | Merge into canonical repo | Terminal themes; merge into `SigmaOS-theme` |
| SigmaOS-text-converter | Merge into canonical repo | Text converter; merge into `SigmaOS-text-editor` |
| SigmaOS-text-effects | Merge into canonical repo | Text effects; merge into `SigmaOS-terminal-text-effects` |
| SigmaOS-monitoring | Merge into canonical repo | Monitoring meta; merge into `SigmaOS-system-admin` |
| SigmaOS-monitoring-tools | Merge into canonical repo | Monitoring tools; merge into `SigmaOS-system-admin` |
| SigmaOS-diagnostics | Merge into canonical repo | Diagnostics; merge into `SigmaOS-troubleshooting-tool` |
| SigmaOS-diagnostic-tools | Merge into canonical repo | Diagnostic tools; merge into `SigmaOS-troubleshooting-tool` |
| SigmaOS-info-center | Merge into canonical repo | Info center; merge into `SigmaOS-control-center` |
| SigmaOS-system-info | Merge into canonical repo | System info; merge into `SigmaOS-control-center` |
| SigmaOS-system-monitor | Merge into canonical repo | System monitor; merge into `SigmaOS-system-admin` |
| SigmaOS-temperature-monitor | Merge into canonical repo | Temp monitor; merge into `SigmaOS-system-admin` |
| SigmaOS-task-manager | Merge into canonical repo | Task manager; merge into `SigmaOS-system-admin` |
| SigmaOS-metric-collector | Merge into canonical repo | Metric collector; merge into `SigmaOS-system-admin` |
| SigmaOS-log-viewer | Merge into canonical repo | Log viewer; merge into `SigmaOS-troubleshooting-tool` |
| SigmaOS-logging | Merge into canonical repo | Logging library; merge into `SigmaOS-core` |
| SigmaOS-logging-tools | Merge into canonical repo | Logging tools; merge into `SigmaOS-core` |
| SigmaOS-system-logger | Merge into canonical repo | System logger; merge into `SigmaOS-core` |
| SigmaOS-access-log | Merge into canonical repo | Access log; merge into `SigmaOS-security` |
| SigmaOS-cron-manager | Merge into canonical repo | Cron management; merge into `SigmaOS-system` |
| SigmaOS-job | Merge into canonical repo | Job abstraction; merge into `SigmaOS-system` |
| SigmaOS-job-manager | Merge into canonical repo | Job manager; merge into `SigmaOS-system` |
| SigmaOS-job-queue | Merge into canonical repo | Job queue; merge into `SigmaOS-system` |
| SigmaOS-job-runner | Merge into canonical repo | Job runner; merge into `SigmaOS-system` |
| SigmaOS-job-scheduler | Merge into canonical repo | Job scheduler; merge into `SigmaOS-system` |
| SigmaOS-job-stream | Merge into canonical repo | Job stream; merge into `SigmaOS-system` |
| SigmaOS-service | Merge into canonical repo | Service alias; merge into `SigmaOS-system` |
| SigmaOS-service-files | Merge into canonical repo | Service files; merge into `SigmaOS-system` |
| SigmaOS-services | Merge into canonical repo | Services plural alias; merge into `SigmaOS-system` |
| SigmaOS-system-daemon | Merge into canonical repo | System daemon; merge into `SigmaOS-system` |
| SigmaOS-system-daemon-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-system` |
| SigmaOS-system-daemon-3 | Merge into canonical repo | Duplicate; merge into `SigmaOS-system` |
| SigmaOS-system-daemons | Merge into canonical repo | Plural alias; merge into `SigmaOS-system` |
| SigmaOS-systemd-integration | Merge into canonical repo | systemd integration; merge into `SigmaOS-system` |
| SigmaOS-systemd-presets | Merge into canonical repo | systemd presets; merge into `SigmaOS-system` |
| SigmaOS-systemd-service-manager | Merge into canonical repo | systemd service manager GUI; merge into `SigmaOS-system` |
| SigmaOS-systemd-units | Merge into canonical repo | systemd unit files; merge into `SigmaOS-system` |
| SigmaOS-system-service | Merge into canonical repo | System service; merge into `SigmaOS-system` |
| SigmaOS-system-service-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-system` |
| SigmaOS-system-service-3 | Merge into canonical repo | Duplicate; merge into `SigmaOS-system` |
| SigmaOS-system-service-4 | Merge into canonical repo | Duplicate; merge into `SigmaOS-system` |
| SigmaOS-system-services | Merge into canonical repo | Plural alias; merge into `SigmaOS-system` |
| SigmaOS-user-daemon | Merge into canonical repo | User daemon; merge into `SigmaOS-session-manager` |
| SigmaOS-user-daemon-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-session-manager` |
| SigmaOS-user-daemons | Merge into canonical repo | Plural alias; merge into `SigmaOS-session-manager` |
| SigmaOS-user-service | Merge into canonical repo | User service; merge into `SigmaOS-session-manager` |
| SigmaOS-user-service-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-session-manager` |
| SigmaOS-user-service-3 | Merge into canonical repo | Duplicate; merge into `SigmaOS-session-manager` |
| SigmaOS-user-service-4 | Merge into canonical repo | Duplicate; merge into `SigmaOS-session-manager` |
| SigmaOS-user-services | Merge into canonical repo | Plural alias; merge into `SigmaOS-session-manager` |
| SigmaOS-doc-developer-guide | Merge into canonical repo | Developer guide; merge into `SigmaOS-docs` |
| SigmaOS-doc-installation-guide | Merge into canonical repo | Installation guide; merge into `SigmaOS-docs` |
| SigmaOS-doc-translation-guide | Merge into canonical repo | Translation guide; merge into `SigmaOS-docs` |
| SigmaOS-doc-troubleshooting-guide | Merge into canonical repo | Troubleshooting guide; merge into `SigmaOS-docs` |
| SigmaOS-documentation-wiki | Merge into canonical repo | Wiki docs; merge into `SigmaOS-docs` |
| SigmaOS-doc-user-guide | Merge into canonical repo | User guide; merge into `SigmaOS-docs` |
| SigmaOS-man-pages | Merge into canonical repo | Man pages; merge into `SigmaOS-docs` |
| SigmaOS-expanded-wiki | Merge into canonical repo | Expanded wiki; merge into `SigmaOS-docs` |
| SigmaOS-distro | Merge into canonical repo | Distro meta; merge into `SigmaOS-release` |
| SigmaOS-distro-inspirations | Merge into canonical repo | Inspiration notes; merge into `SigmaOS-docs` |
| SigmaOS-iso-root | Merge into canonical repo | ISO root; merge into `SigmaOS-release` |
| SigmaOS-rolling-release | Merge into canonical repo | Rolling release policy; merge into `SigmaOS-release` |
| SigmaOS-global-packages | Merge into canonical repo | Global package list; merge into `SigmaOS-package-manager` |
| SigmaOS-bin-packages | Merge into canonical repo | Binary packages; merge into `SigmaOS-package-manager` |
| SigmaOS-opt-packages | Merge into canonical repo | Optional packages; merge into `SigmaOS-package-manager` |
| SigmaOS-src-packages | Merge into canonical repo | Source packages; merge into `SigmaOS-package-manager` |
| SigmaOS-local-packages | Merge into canonical repo | Local packages; merge into `SigmaOS-package-manager` |
| SigmaOS-ports-packages | Merge into canonical repo | Ports packages; merge into `SigmaOS-package-manager` |
| SigmaOS-void-repos | Merge into canonical repo | Void repos; merge into `SigmaOS-repo-manager` |
| SigmaOS-usr-ports | Merge into canonical repo | BSD ports; merge into `SigmaOS-repo-manager` |
| SigmaOS-var-ports | Merge into canonical repo | Var ports; merge into `SigmaOS-repo-manager` |
| SigmaOS-config | Merge into canonical repo | Generic config alias; merge into `SigmaOS-core` |
| SigmaOS-etc-configs | Merge into canonical repo | /etc configs; merge into `SigmaOS-core` |
| SigmaOS-home-configs | Merge into canonical repo | Home dir configs; merge into `SigmaOS-dots-configuration` |
| SigmaOS-run-configs | Merge into canonical repo | Runtime configs; merge into `SigmaOS-core` |
| SigmaOS-tmp-configs | Merge into canonical repo | Temp configs; merge into `SigmaOS-core` |
| SigmaOS-var-configs | Merge into canonical repo | /var configs; merge into `SigmaOS-core` |
| SigmaOS-user-configs | Merge into canonical repo | User configs; merge into `SigmaOS-dots-configuration` |
| SigmaOS-system-configs | Merge into canonical repo | System configs; merge into `SigmaOS-core` |
| SigmaOS-misc-scripts | Merge into canonical repo | Misc scripts; merge into `SigmaOS-common` |
| SigmaOS-script-runner | Merge into canonical repo | Script runner; merge into `SigmaOS-common` |
| SigmaOS-bin | Merge into canonical repo | Binary helpers; merge into `SigmaOS-common` |
| SigmaOS-tool | Merge into canonical repo | Tool alias; merge into `SigmaOS-common` |
| SigmaOS-tools | Merge into canonical repo | Tools alias; merge into `SigmaOS-common` |
| SigmaOS-utility | Merge into canonical repo | Utility alias; merge into `SigmaOS-common` |
| SigmaOS-utility-collection | Merge into canonical repo | Utility collection; merge into `SigmaOS-common` |
| SigmaOS-console-utilities | Merge into canonical repo | Console utilities; merge into `SigmaOS-common` |
| SigmaOS-functions | Merge into canonical repo | Function library; merge into `SigmaOS-common` |
| SigmaOS-array | Merge into canonical repo | Array utility; merge into `SigmaOS-common` |
| SigmaOS-link | Merge into canonical repo | Link utility; merge into `SigmaOS-common` |
| SigmaOS-integration | Merge into canonical repo | Integration meta; merge into `SigmaOS-core` |
| SigmaOS-interface | Merge into canonical repo | Interface library; merge into `SigmaOS-core` |
| SigmaOS-protocol | Merge into canonical repo | Protocol library; merge into `SigmaOS-network` |
| SigmaOS-layer | Merge into canonical repo | Layer abstraction; merge into `SigmaOS-core` |
| SigmaOS-layer-2 | Merge into canonical repo | Duplicate layer; merge into `SigmaOS-core` |
| SigmaOS-layer-3 | Merge into canonical repo | Duplicate layer; merge into `SigmaOS-core` |
| SigmaOS-layer-4 | Merge into canonical repo | Duplicate layer; merge into `SigmaOS-core` |
| SigmaOS-layer-5 | Merge into canonical repo | Duplicate layer; merge into `SigmaOS-core` |
| SigmaOS-layer-6 | Merge into canonical repo | Duplicate layer; merge into `SigmaOS-core` |
| SigmaOS-layer-7 | Merge into canonical repo | Duplicate layer; merge into `SigmaOS-core` |
| SigmaOS-layer-8 | Merge into canonical repo | Duplicate layer; merge into `SigmaOS-core` |
| SigmaOS-framework | Merge into canonical repo | Framework meta; merge into `SigmaOS-core` |
| SigmaOS-framework-apis | Merge into canonical repo | Framework APIs; merge into `SigmaOS-core` |
| SigmaOS-framework-apis-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-core` |
| SigmaOS-framework-bridge | Merge into canonical repo | Framework bridge; merge into `SigmaOS-core` |
| SigmaOS-framework-call | Merge into canonical repo | Framework call; merge into `SigmaOS-core` |
| SigmaOS-library | Merge into canonical repo | Library alias; merge into `SigmaOS-common` |
| SigmaOS-library-2 | Merge into canonical repo | Duplicate library; merge into `SigmaOS-common` |
| SigmaOS-library-3 | Merge into canonical repo | Duplicate library; merge into `SigmaOS-common` |
| SigmaOS-library-4 | Merge into canonical repo | Duplicate library; merge into `SigmaOS-common` |
| SigmaOS-library-5 | Merge into canonical repo | Duplicate library; merge into `SigmaOS-common` |
| SigmaOS-library-6 | Merge into canonical repo | Duplicate library; merge into `SigmaOS-common` |
| SigmaOS-library-7 | Merge into canonical repo | Duplicate library; merge into `SigmaOS-common` |
| SigmaOS-library-8 | Merge into canonical repo | Duplicate library; merge into `SigmaOS-common` |
| SigmaOS-library-apis | Merge into canonical repo | Library APIs; merge into `SigmaOS-common` |
| SigmaOS-library-apis-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-common` |
| SigmaOS-library-call | Merge into canonical repo | Library call; merge into `SigmaOS-common` |
| SigmaOS-library-headers | Merge into canonical repo | Library headers; merge into `SigmaOS-common` |
| SigmaOS-library-tools | Merge into canonical repo | Library tools; merge into `SigmaOS-common` |
| SigmaOS-system-libs | Merge into canonical repo | System libs; merge into `SigmaOS-common` |
| SigmaOS-user-libs | Merge into canonical repo | User libs; merge into `SigmaOS-common` |
| SigmaOS-klib | Merge into canonical repo | Kernel lib; merge into `SigmaOS-kernel` |
| SigmaOS-system-frameworks | Merge into canonical repo | System frameworks; merge into `SigmaOS-core` |
| SigmaOS-user-frameworks | Merge into canonical repo | User frameworks; merge into `SigmaOS-session-manager` |
| SigmaOS-system-plugins | Merge into canonical repo | System plugins; merge into `SigmaOS-system` |
| SigmaOS-user-plugins | Merge into canonical repo | User plugins; merge into `SigmaOS-extension-manager` |
| SigmaOS-plugin-loader | Merge into canonical repo | Plugin loader; merge into `SigmaOS-extension-manager` |
| SigmaOS-tool-apis | Merge into canonical repo | Tool APIs; merge into `SigmaOS-common` |
| SigmaOS-tool-call | Merge into canonical repo | Tool call; merge into `SigmaOS-common` |
| SigmaOS-system-apis | Merge into canonical repo | System APIs; merge into `SigmaOS-core` |
| SigmaOS-system-apis-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-core` |
| SigmaOS-system-call | Merge into canonical repo | Syscall; merge into `SigmaOS-kernel` |
| SigmaOS-system-calls | Merge into canonical repo | Syscalls plural; merge into `SigmaOS-kernel` |
| SigmaOS-application-apis | Merge into canonical repo | App APIs; merge into `SigmaOS-app-framework` |
| SigmaOS-application-apis-2 | Merge into canonical repo | Duplicate; merge into `SigmaOS-app-framework` |
| SigmaOS-application-call | Merge into canonical repo | App call; merge into `SigmaOS-app-framework` |
| SigmaOS-kernel-call | Merge into canonical repo | Kernel call; merge into `SigmaOS-kernel` |
| SigmaOS-kernel-apis | Merge into canonical repo | Kernel APIs; merge into `SigmaOS-kernel` |
| SigmaOS-system-core | Merge into canonical repo | System core alias; merge into `SigmaOS-system` |
| SigmaOS-system-headers | Merge into canonical repo | System headers; merge into `SigmaOS-kernel` |
| SigmaOS-system-feature | Merge into canonical repo | System feature stub; merge into `SigmaOS-system` |
| SigmaOS-system-feature-2 through SigmaOS-system-feature-87 | Merge into canonical repo | Numbered feature stubs (87 entries); triage individually into relevant subsystem repos as concrete features are defined |
| SigmaOS-core-feature | Merge into canonical repo | Core feature stub; merge into `SigmaOS-core` |
| SigmaOS-core-feature-2 through SigmaOS-core-feature-87 | Merge into canonical repo | Numbered feature stubs (87 entries); triage individually into `SigmaOS-core` |
| SigmaOS-user-feature | Merge into canonical repo | User feature stub; merge into `SigmaOS-session-manager` or relevant app repo |
| SigmaOS-user-feature-2 through SigmaOS-user-feature-87 | Merge into canonical repo | Numbered feature stubs (87 entries); triage individually |
| SigmaOS-graphics | Merge into canonical repo | Graphics meta; merge into `SigmaOS-gpu` |
| SigmaOS-graphics-core | Merge into canonical repo | Graphics core; merge into `SigmaOS-gpu` |
| SigmaOS-graphic-effects | Merge into canonical repo | Graphic effects; merge into `SigmaOS-compositor` |
| SigmaOS-animation-tool | Merge into canonical repo | Animation tool; merge into `SigmaOS-compositor` |
| SigmaOS-3d-modeling | Merge into canonical repo | 3D modeling; merge into `SigmaOS-office` or standalone app |
| SigmaOS-3d-renderer | Merge into canonical repo | 3D renderer; merge into `SigmaOS-gpu` |
| SigmaOS-database | Merge into canonical repo | Database; merge into `SigmaOS-core` |
| SigmaOS-container | Merge into canonical repo | Container support; merge into `SigmaOS-system` |
| SigmaOS-containerization | Merge into canonical repo | Container system; merge into `SigmaOS-system` |
| SigmaOS-virtualization | Merge into canonical repo | Virtualization; merge into `SigmaOS-system` |
| SigmaOS-embedded | Merge into canonical repo | Embedded support; merge into `SigmaOS-kernel` |
| SigmaOS-server | Merge into canonical repo | Server profile; merge into `SigmaOS-workstation` |
| SigmaOS-mobile | Merge into canonical repo | Mobile profile; archive or defer — not in current scope |
| SigmaOS-iot-features | Merge into canonical repo | IoT features; defer until IoT target is defined |
| SigmaOS-cloud-features | Merge into canonical repo | Cloud features; defer until cloud target is defined |
| SigmaOS-linuxmint | Merge into canonical repo | Linux Mint upstream tracking; merge into `SigmaOS-docs` (inspirations) |
| SigmaOS-linux-aurora | Merge into canonical repo | Aurora tracking; merge into `SigmaOS-docs` |
| SigmaOS-linux-labs | Merge into canonical repo | Labs research; merge into `SigmaOS-docs` |
| SigmaOS-linux-live | Merge into canonical repo | Live Linux; merge into `SigmaOS-live-session` |
| SigmaOS-m1n1 | Merge into canonical repo | Apple M1 bootloader; merge into `SigmaOS-boot` when ARM target is active |
| SigmaOS-omadots | Merge into canonical repo | Omarchy dots; merge into `SigmaOS-dots-configuration` |
| SigmaOS-omafind | Merge into canonical repo | Omarchy find; merge into `SigmaOS-dev-tools` |
| SigmaOS-omamac | Merge into canonical repo | Omarchy mac config; merge into `SigmaOS-mac-run` |
| SigmaOS-omareel | Merge into canonical repo | Omarchy reel; merge into `SigmaOS-reel` |
| SigmaOS-omasnap | Merge into canonical repo | Omarchy snap; merge into `SigmaOS-screen-shot` |
| SigmaOS-omarchy-core | Merge into canonical repo | Omarchy core; merge into `SigmaOS-core` (inspirations) |
| SigmaOS-opal | Merge into canonical repo | Opal library; merge into `SigmaOS-common` |
| SigmaOS-compliance | Merge into canonical repo | Compliance docs; merge into `SigmaOS-security` |
| SigmaOS-herdr-runtime-live-coding-agents | Merge into canonical repo | Live coding agent runtime; merge into `SigmaOS-herdr-runtime` |
| SigmaOS-jsework | Merge into canonical repo | JS work helper; merge into `SigmaOS-JS-interpreter` |
| SigmaOS-jargonaut | Merge into canonical repo | Jargon tool; merge into `SigmaOS-docs` |
| SigmaOS-klib | Merge into canonical repo | Kernel lib; already listed above |
| SigmaOS-loader | Merge into canonical repo | Loader; merge into `SigmaOS-boot` |
| SigmaOS-modules | Merge into canonical repo | Modules meta; merge into `SigmaOS-kernel` |
| SigmaOS-module | Merge into canonical repo | Module; merge into `SigmaOS-kernel` |
| SigmaOS-module-manager | Merge into canonical repo | Module manager; merge into `SigmaOS-kernel` |
| SigmaOS-compatibility | Merge into canonical repo | Compatibility layer; merge into `SigmaOS-core` |
| SigmaOS-governance | Merge into canonical repo | Governance doc repo; content moved into `.agents/tasks/governance/` |
| SigmaOS-legal | Merge into canonical repo | Legal docs; merge into `SigmaOS-docs` |
| SigmaOS-research | Merge into canonical repo | Research notes; merge into `SigmaOS-docs` |
| SigmaOS-contributing | Merge into canonical repo | Contributing guide; merge into `SigmaOS-docs` |
| SigmaOS-community | Merge into canonical repo | Community resources; merge into `SigmaOS-docs` |
| SigmaOS-skill-management | Merge into canonical repo | Skill management; merge into `SigmaOS-docs` |
| SigmaOS-skills | Merge into canonical repo | Skills repo; merge into `SigmaOS-docs` |
| SigmaOS-omarchy-architecture | Merge into canonical repo | Architecture notes; merge into `SigmaOS-docs` |
| SigmaOS-omarchy-philosophy | Merge into canonical repo | Philosophy docs; merge into `SigmaOS-docs` |
| SigmaOS-omarchy-philosophy-docs | Merge into canonical repo | Duplicate philosophy; merge into `SigmaOS-docs` |
| SigmaOS-bar-widgets | Merge into canonical repo | Bar widgets; merge into `SigmaOS-panel` |
| SigmaOS-bar_widgets | Merge into canonical repo | Snake-case alias; merge into `SigmaOS-panel` |
| SigmaOS-browser | Merge into canonical repo | Browser meta; merge into `SigmaOS-chromium` or standalone |
| SigmaOS-browser-handoff | Merge into canonical repo | Browser handoff; merge into `SigmaOS-desktop-portal` |
| SigmaOS-browser_handoff | Merge into canonical repo | Snake-case alias; merge into `SigmaOS-desktop-portal` |
| SigmaOS-api-documentation | Merge into canonical repo | API docs; merge into `SigmaOS-docs` |
| SigmaOS-software-documentation | Merge into canonical repo | Software docs; merge into `SigmaOS-docs` |
| SigmaOS-software-collections | Merge into canonical repo | Software collections; merge into `SigmaOS-software-manager` |
| SigmaOS-apps | Merge into canonical repo | Apps meta-package; merge into `SigmaOS-software-manager` |
| SigmaOS-package | Merge into canonical repo | Package alias; merge into `SigmaOS-package-manager` |
| SigmaOS-dependency-resolver | Merge into canonical repo | Dependency resolver; merge into `SigmaOS-package-manager` |
| SigmaOS-debugging-tools | Merge into canonical repo | Debug tools; merge into `SigmaOS-dev-tools` |
| SigmaOS-performance-tools | Merge into canonical repo | Performance tools; merge into `SigmaOS-performance` |
| SigmaOS-profiling-tools | Merge into canonical repo | Profiling; merge into `SigmaOS-performance` |
| SigmaOS-cpu | Merge into canonical repo | CPU meta; merge into `SigmaOS-cpu-profile` |
| SigmaOS-allocators | Merge into canonical repo | Memory allocators; merge into `SigmaOS-kernel` |
| SigmaOS-system-cleaner | Merge into canonical repo | System cleaner; merge into `SigmaOS-system-admin` |
| SigmaOS-uninstall-service | Merge into canonical repo | Uninstall service; merge into `SigmaOS-software-manager` |
| SigmaOS-removal-service | Merge into canonical repo | Removal service; merge into `SigmaOS-software-manager` |
| SigmaOS-restore | Merge into canonical repo | Restore tool; merge into `SigmaOS-backup-tool` |
| SigmaOS-export-tool | Merge into canonical repo | Export tool; merge into `SigmaOS-backup-tool` |
| SigmaOS-clone-manager | Merge into canonical repo | Clone manager; merge into `SigmaOS-backup-tool` |
| SigmaOS-system-backup | Merge into canonical repo | System backup alias; merge into `SigmaOS-backup-tool` |
| SigmaOS-backup | Merge into canonical repo | Backup alias; merge into `SigmaOS-backup-tool` |
| SigmaOS-report-generator | Merge into canonical repo | Report generator; merge into `SigmaOS-docs` |
| SigmaOS-ml | Merge into canonical repo | Machine learning meta; merge into `SigmaOS-docs` (future) |
| SigmaOS-mm | Merge into canonical repo | Memory management alias; merge into `SigmaOS-kernel` |
| SigmaOS-repair | Merge into canonical repo | Repair tool; merge into `SigmaOS-troubleshooting-tool` |
| SigmaOS-rollback | Merge into canonical repo | Rollback tool; merge into `SigmaOS-update` |
| SigmaOS-thingy | Merge into canonical repo | Placeholder; merge into `SigmaOS-common` |
| SigmaOS-stack | Merge into canonical repo | Stack abstraction; merge into `SigmaOS-core` |
| SigmaOS-component | Merge into canonical repo | Component meta; merge into `SigmaOS-core` |
| SigmaOS-etc | Merge into canonical repo | /etc alias; merge into `SigmaOS-core` |
| SigmaOS-elsewhen | Merge into canonical repo | Placeholder; merge into `SigmaOS-docs` |
| SigmaOS-hype | Merge into canonical repo | Marketing placeholder; merge into `SigmaOS-website` |
| SigmaOS-signed | Merge into canonical repo | Signed package alias; merge into `SigmaOS-sign` |
| SigmaOS-task | Merge into canonical repo | Task alias; merge into `SigmaOS-system` |
| SigmaOS-custom | Merge into canonical repo | Custom config; merge into `SigmaOS-dots-configuration` |
| SigmaOS-customization | Merge into canonical repo | Customization meta; merge into `SigmaOS-control-center` |
| SigmaOS-config | Merge into canonical repo | Already noted; merge into `SigmaOS-core` |
| SigmaOS-system-feature-10 through SigmaOS-system-feature-87 | Merge into canonical repo | See numbered stubs note above |
| SigmaOS-core-feature-10 through SigmaOS-core-feature-87 | Merge into canonical repo | See numbered stubs note above |
| SigmaOS-user-feature-10 through SigmaOS-user-feature-87 | Merge into canonical repo | See numbered stubs note above |
| **— Prototype / optional —** | | |
| Sigma | Prototype / optional | Top-level name placeholder; no defined scope yet |
| SigmaOS | Prototype / optional | Root monorepo placeholder; scope TBD |
| SigmaOS-accessibility | Prototype / optional | Accessibility layer; prototype until desktop is stable |
| SigmaOS-3d-modeling | Prototype / optional | 3D modeling app; experimental, not on release critical path |
| SigmaOS-3d-renderer | Prototype / optional | 3D renderer; experimental |
| SigmaOS-ai | Prototype / optional | AI integration; experimental |
| SigmaOS-animation-tool | Prototype / optional | Animation tool; experimental |
| SigmaOS-automation | Prototype / optional | Automation framework; experimental |
| SigmaOS-caligula | Prototype / optional | Placeholder name; no scope |
| SigmaOS-captain | Prototype / optional | Placeholder; no scope |
| SigmaOS-creative | Prototype / optional | Creative tools meta; experimental |
| SigmaOS-creative-tools | Prototype / optional | Creative tools; experimental |
| SigmaOS-education | Prototype / optional | Education meta; experimental |
| SigmaOS-embedded | Prototype / optional | Embedded target; not in current release scope |
| SigmaOS-futuristic | Prototype / optional | Futuristic feature set; experimental |
| SigmaOS-graphics | Prototype / optional | Graphics meta; experimental until GPU is stable |
| SigmaOS-iot-features | Prototype / optional | IoT; no defined target yet |
| SigmaOS-cloud-features | Prototype / optional | Cloud; no defined target yet |
| SigmaOS-kde-plasma | Prototype / optional | KDE Plasma session; experimental DE alternative |
| SigmaOS-mate | Prototype / optional | MATE DE; experimental |
| SigmaOS-mate-desktop | Prototype / optional | MATE desktop; experimental |
| SigmaOS-mate-packages-betsy | Prototype / optional | MATE Betsy packages; experimental |
| SigmaOS-mobile | Prototype / optional | Mobile target; not in current scope |
| SigmaOS-music | Prototype / optional | Music app; not yet defined |
| SigmaOS-opal | Prototype / optional | Opal library; experimental |
| SigmaOS-productivity | Prototype / optional | Productivity meta; experimental |
| SigmaOS-server | Prototype / optional | Server profile; experimental |
| SigmaOS-simulation | Prototype / optional | Simulation tool; experimental |
| SigmaOS-virtualization | Prototype / optional | Virtualization; experimental until kernel stable |
| SigmaOS-workstation | Prototype / optional | Workstation profile; experimental until release |
| SigmaOS-herdr | Prototype / optional | Herdr root; experimental |
| SigmaOS-launch-ready | Prototype / optional | Launch readiness tool; experimental |
| SigmaOS-maximizer | Prototype / optional | Window maximizer helper; experimental |
| SigmaOS-minimizer | Prototype / optional | Window minimizer helper; experimental |
| SigmaOS-ml | Prototype / optional | ML integration; no defined scope |
| SigmaOS-omarchy-sub-repo-creator | Prototype / optional | Sub-repo scaffolding tool; experimental |
| SigmaOS-omarchy-sub-repos | Prototype / optional | Sub-repos management; experimental |
| SigmaOS-omarchy-subsystem | Prototype / optional | Subsystem definition; experimental |
| SigmaOS-opal | Prototype / optional | Opal; experimental |
| SigmaOS-poc-repos-created | Prototype / optional | POC repo tracker; internal tooling only |
| SigmaOS-quickpanel | Prototype / optional | Quick panel; prototype |
| SigmaOS-rolling-release | Prototype / optional | Rolling release policy; experimental |
| SigmaOS-runtime | Prototype / optional | Runtime alias; experimental until core stable |
| SigmaOS-skills | Prototype / optional | Skills management; experimental |
| SigmaOS-stack | Prototype / optional | Stack abstraction; experimental |
| SigmaOS-stream | Prototype / optional | Streaming; experimental |
| SigmaOS-training | Prototype / optional | Training data/docs; experimental |
| SigmaOS-university | Prototype / optional | University learning module; experimental |
| SigmaOS-window-management-library | Prototype / optional | Window management library (phase4); needs desktop to be stable first |
| **— Archive —** | | |
| SigmaOS--Dante | Archive | Double-dash typo; orphaned |
| SigmaOS-ab | Archive | Two-letter placeholder; no scope defined |
| SigmaOS-abduco | Archive | abduco session manager clone; superseded by SigmaOS-terminal |
| SigmaOS-abiword | Archive | AbiWord fork; superseded by SigmaOS-office |
| SigmaOS-abs | Archive | Two-letter placeholder; no scope |
| SigmaOS-ac | Archive | Two-letter placeholder; no scope |
| SigmaOS-accelerate | Archive | Acceleration placeholder; no scope |
| SigmaOS-acceleration | Archive | Acceleration duplicate; no scope |
| SigmaOS-accerciser | Archive | GNOME Accerciser clone; not on roadmap |
| SigmaOS-access | Archive | Generic access placeholder; no scope |
| SigmaOS-access-control | Archive | Access control; superseded by SigmaOS-auth |
| SigmaOS-accessory | Archive | Accessory placeholder; no scope |
| SigmaOS-accomplishment | Archive | Accomplishment placeholder; no scope |
| SigmaOS-accountability | Archive | Accountability placeholder; no scope |
| SigmaOS-acct | Archive | Accounting placeholder; no scope |
| SigmaOS-achievement | Archive | Achievement system; no scope |
| SigmaOS-acronis | Archive | Acronis clone; no scope |
| SigmaOS-activeandroid | Archive | Android ActiveRecord; not relevant |
| SigmaOS-activedirectory | Archive | Active Directory; enterprise scope not defined |
| SigmaOS-activemq | Archive | ActiveMQ; not relevant |
| SigmaOS-activity-log | Archive | Activity log; superseded by SigmaOS-security |
| SigmaOS-activitypub | Archive | ActivityPub; no scope |
| SigmaOS-ad | Archive | Two-letter placeholder |
| SigmaOS-ada | Archive | Ada language support; no scope |
| SigmaOS-adaptability | Archive | Placeholder; no scope |
| SigmaOS-adaptation | Archive | Placeholder; no scope |
| SigmaOS-adaptive-server | Archive | Adaptive server; no scope |
| SigmaOS-adaware | Archive | Adaware clone; superseded by SigmaOS-antivirus |
| SigmaOS-adium | Archive | Adium clone; no scope |
| SigmaOS-adler32 | Archive | Hash function; no standalone scope |
| SigmaOS-adobe-fonts | Archive | Adobe fonts; licensing issues; no scope |
| SigmaOS-adventure | Archive | Adventure game; no scope |
| SigmaOS-aether.nvim | Archive | Neovim theme; superseded by SigmaOS-nvim |
| SigmaOS-airbnb | Archive | Airbnb clone; not relevant |
| SigmaOS-alchemy | Archive | Placeholder; no scope |
| SigmaOS-alert-system | Archive | Alert system; superseded by SigmaOS-notification |
| SigmaOS-arch | Archive | Arch placeholder; use arch-specific repos instead |
| SigmaOS-arch-am through SigmaOS-arch-wm | Archive | Arch sub-repos with short placeholder names (100+ entries); superseded by canonical repos; archive all |
| SigmaOS-omarchy-am through SigmaOS-omarchy-wm | Archive | Omarchy sub-repos with short placeholder names; superseded by canonical repos; archive all |
| SigmaOS-fedora-log-rotator | Archive | Fedora clone; not in scope |
| SigmaOS-fedora-package-manager | Archive | Fedora clone; superseded by SigmaOS-package-manager |
| SigmaOS-fedora-session-logger | Archive | Fedora clone; not in scope |
| SigmaOS-fedora-software-center | Archive | Fedora clone; superseded by SigmaOS-software-manager |
| SigmaOS-fedora-system-monitor | Archive | Fedora clone; superseded by SigmaOS-system-admin |
| SigmaOS-fedora-update-manager | Archive | Fedora clone; superseded by SigmaOS-update |
| SigmaOS-fedora-update-notifier | Archive | Fedora clone; not in scope |
| SigmaOS-mint-accessibility | Archive | Mint clone; superseded by SigmaOS-accessibility |
| SigmaOS-mint-antivirus | Archive | Mint clone; superseded by SigmaOS-security |
| SigmaOS-mint-appstream | Archive | Mint clone; superseded by SigmaOS-software-manager |
| SigmaOS-mint-artwork | Archive | Mint clone; superseded by SigmaOS-brand-logo |
| SigmaOS-mint-artwork-share | Archive | Mint clone; superseded by SigmaOS-brand-logo |
| SigmaOS-mint-audio | Archive | Mint clone; superseded by SigmaOS-audio |
| SigmaOS-mint-audio-manager | Archive | Mint clone; superseded by SigmaOS-audio |
| SigmaOS-mint-audio-recorder | Archive | Mint clone; superseded by SigmaOS-audio |
| SigmaOS-mint-auto-mounter | Archive | Mint clone; superseded by SigmaOS-filesystem |
| SigmaOS-mint-bluetooth | Archive | Mint clone; superseded by SigmaOS-bluetooth |
| SigmaOS-mint-bluetooth-manager | Archive | Mint clone; superseded by SigmaOS-bluetooth-tool |
| SigmaOS-mint-bluetooth-setup | Archive | Mint clone; superseded by SigmaOS-bluetooth |
| SigmaOS-mint-boot-manager | Archive | Mint clone; superseded by SigmaOS-boot |
| SigmaOS-mint-bugtracker | Archive | Mint clone; use GitHub issues |
| SigmaOS-mint-calculator | Archive | Mint clone; superseded by SigmaOS-calculator |
| SigmaOS-mint-calendar | Archive | Mint clone; superseded by SigmaOS-clock-global |
| SigmaOS-mint-call-manager | Archive | Mint clone; no scope |
| SigmaOS-mint-code-editor | Archive | Mint clone; superseded by SigmaOS-text-editor |
| SigmaOS-mint-colorschemes | Archive | Mint clone; superseded by SigmaOS-theme |
| SigmaOS-mint-command-processor | Archive | Mint clone; superseded by SigmaOS-terminal |
| SigmaOS-mint-community | Archive | Mint clone; superseded by SigmaOS-docs |
| SigmaOS-mint-context-switcher | Archive | Mint clone; superseded by SigmaOS-desktop |
| SigmaOS-mint-crypto-wallet | Archive | Mint clone; no scope defined |
| SigmaOS-mint-customize | Archive | Mint clone; superseded by SigmaOS-control-center |
| SigmaOS-mint-date-time | Archive | Mint clone; superseded by SigmaOS-clock-global |
| SigmaOS-mint-debugger | Archive | Mint clone; superseded by SigmaOS-dev-tools |
| SigmaOS-mint-dependency-resolver | Archive | Mint clone; superseded by SigmaOS-package-manager |
| SigmaOS-mint-dependency-tracker | Archive | Mint clone; superseded by SigmaOS-package-manager |
| SigmaOS-mint-desktop-config-importer | Archive | Mint clone; superseded by SigmaOS-dots-configuration |
| SigmaOS-mint-device-manager | Archive | Mint clone; superseded by SigmaOS-hardware |
| SigmaOS-mint-dev-pkg | Archive | Mint clone; superseded by SigmaOS-dev-tools |
| SigmaOS-mint-dev-tools | Archive | Mint clone; superseded by SigmaOS-dev-tools |
| SigmaOS-mint-diagnostic-tool | Archive | Mint clone; superseded by SigmaOS-troubleshooting-tool |
| SigmaOS-mint-directory-manager | Archive | Mint clone; superseded by SigmaOS-file-manager-system |
| SigmaOS-mint-disk-cleanup | Archive | Mint clone; superseded by SigmaOS-system-admin |
| SigmaOS-mint-disk-utility | Archive | Mint clone; superseded by SigmaOS-disk-utility |
| SigmaOS-mint-display-manager | Archive | Mint clone; superseded by SigmaOS-display-manager |
| SigmaOS-mint-doc | Archive | Mint clone; superseded by SigmaOS-docs |
| SigmaOS-mint-drawing | Archive | Mint clone; superseded by SigmaOS-draw |
| SigmaOS-mint-drivers | Archive | Mint clone; superseded by SigmaOS-drivers |
| SigmaOS-mint-drivers-detector | Archive | Mint clone; superseded by SigmaOS-drivers |
| SigmaOS-mint-drivers-extra | Archive | Mint clone; superseded by SigmaOS-drivers |
| SigmaOS-mint-drivers-pack | Archive | Mint clone; superseded by SigmaOS-drivers |
| SigmaOS-mint-editor | Archive | Mint clone; superseded by SigmaOS-text-editor |
| SigmaOS-mint-encryption-tool | Archive | Mint clone; superseded by SigmaOS-security |
| SigmaOS-mint-event-handler | Archive | Mint clone; superseded by SigmaOS-system |
| SigmaOS-mint-executor | Archive | Mint clone; superseded by SigmaOS-system |
| SigmaOS-mint-faq | Archive | Mint clone; superseded by SigmaOS-docs |
| SigmaOS-mint-faq-system | Archive | Mint clone; superseded by SigmaOS-docs |
| SigmaOS-mint-file-browser | Archive | Mint clone; superseded by SigmaOS-file-manager-system |
| SigmaOS-mint-file-indexer | Archive | Mint clone; superseded by SigmaOS-file-manager-system |
| SigmaOS-mint-file-manager | Archive | Mint clone; superseded by SigmaOS-file-manager-system |
| SigmaOS-mint-firewall | Archive | Mint clone; superseded by SigmaOS-firewall |
| SigmaOS-mint-flatpak-manager | Archive | Mint clone; superseded by SigmaOS-software-manager |
| SigmaOS-mint-font-installer | Archive | Mint clone; superseded by SigmaOS-font-manager |
| SigmaOS-mint-fonts | Archive | Mint clone; superseded by SigmaOS-font-manager |
| SigmaOS-mint-graphics-renderer | Archive | Mint clone; superseded by SigmaOS-gpu |
| SigmaOS-mint-graphing-calculator | Archive | Mint clone; superseded by SigmaOS-calculator |
| SigmaOS-mint-help-center | Archive | Mint clone; superseded by SigmaOS-docs |
| SigmaOS-mint-helpdesk | Archive | Mint clone; superseded by SigmaOS-docs |
| SigmaOS-mint-help-tips | Archive | Mint clone; superseded by SigmaOS-docs |
| SigmaOS-mint-icon-packer | Archive | Mint clone; superseded by SigmaOS-icon-theme |
| SigmaOS-mint-icon-set | Archive | Mint clone; superseded by SigmaOS-icon-theme |
| SigmaOS-mint-image-viewer | Archive | Mint clone; superseded by SigmaOS-image-viewer |
| SigmaOS-mint-installer | Archive | Mint clone; superseded by SigmaOS-installer |
| SigmaOS-mint-interrupt-handler | Archive | Mint clone; superseded by SigmaOS-kernel |
| SigmaOS-mint-iso-builder | Archive | Mint clone; superseded by SigmaOS-release |
| SigmaOS-mint-job-scheduler | Archive | Mint clone; superseded by SigmaOS-system |
| SigmaOS-mint-job-tracker | Archive | Mint clone; superseded by SigmaOS-system |
| SigmaOS-mint-kernel-builder | Archive | Mint clone; superseded by SigmaOS-kernel |
| SigmaOS-mint-kernel-manager | Archive | Mint clone; superseded by SigmaOS-kernel |
| SigmaOS-mint-keyring-manager | Archive | Mint clone; superseded by SigmaOS-auth |
| SigmaOS-mint-knowledge-base | Archive | Mint clone; superseded by SigmaOS-docs |
| SigmaOS-mint-launcher | Archive | Mint clone; superseded by SigmaOS-launcher |
| SigmaOS-mint-link-manager | Archive | Mint clone; superseded by SigmaOS-common |
| SigmaOS-mint-live-session | Archive | Mint clone; superseded by SigmaOS-live-session |
| SigmaOS-mint-load-balancer | Archive | Mint clone; no scope |
| SigmaOS-mint-log-rotator | Archive | Mint clone; superseded by SigmaOS-system |
| SigmaOS-mint-log-viewer | Archive | Mint clone; superseded by SigmaOS-troubleshooting-tool |
| SigmaOS-mint-media-player | Archive | Mint clone; superseded by SigmaOS-player |
| SigmaOS-mint-memory-manager | Archive | Mint clone; superseded by SigmaOS-kernel |
| SigmaOS-mint-mirrors | Archive | Mint clone; superseded by SigmaOS-repo-manager |
| SigmaOS-mint-modem-manager | Archive | Mint clone; superseded by SigmaOS-network |
| SigmaOS-mint-module-compiler | Archive | Mint clone; superseded by SigmaOS-kernel |
| SigmaOS-mint-modules-loader | Archive | Mint clone; superseded by SigmaOS-kernel |
| SigmaOS-mint-network-configurator | Archive | Mint clone; superseded by SigmaOS-network |
| SigmaOS-mint-network-manager | Archive | Mint clone; superseded by SigmaOS-network |
| SigmaOS-mint-network-tools | Archive | Mint clone; superseded by SigmaOS-network |
| SigmaOS-mint-package-installer | Archive | Mint clone; superseded by SigmaOS-software-manager |
| SigmaOS-mint-package-lister | Archive | Mint clone; superseded by SigmaOS-package-manager |
| SigmaOS-mint-package-manager | Archive | Mint clone; superseded by SigmaOS-package-manager |
| SigmaOS-mint-package-querier | Archive | Mint clone; superseded by SigmaOS-package-manager |
| SigmaOS-mint-packages | Archive | Mint clone; superseded by SigmaOS-package-manager |
| SigmaOS-mint-package-verify | Archive | Mint clone; superseded by SigmaOS-package-manager |
| SigmaOS-mint-password-manager | Archive | Mint clone; superseded by SigmaOS-auth |
| SigmaOS-mint-path-monitor | Archive | Mint clone; superseded by SigmaOS-filesystem |
| SigmaOS-mint-path-resolver | Archive | Mint clone; superseded by SigmaOS-filesystem |
| SigmaOS-mint-pdf-viewer | Archive | Mint clone; superseded by SigmaOS-xreader |
| SigmaOS-mint-performance-analyzer | Archive | Mint clone; superseded by SigmaOS-performance |
| SigmaOS-mint-performance-monitor | Archive | Mint clone; superseded by SigmaOS-performance |
| SigmaOS-mint-permission-manager | Archive | Mint clone; superseded by SigmaOS-auth |
| SigmaOS-mint-power-manager | Archive | Mint clone; superseded by SigmaOS-power-manager |
| SigmaOS-mint-power-profiles | Archive | Mint clone; superseded by SigmaOS-power-policy |
| SigmaOS-mint-power-supplier | Archive | Mint clone; superseded by SigmaOS-power-manager |
| SigmaOS-mint-printer-manager | Archive | Mint clone; superseded by SigmaOS-printing |
| SigmaOS-mint-process-handler | Archive | Mint clone; superseded by SigmaOS-system |
| SigmaOS-mint-process-manager | Archive | Mint clone; superseded by SigmaOS-system |
| SigmaOS-mint-profiler | Archive | Mint clone; superseded by SigmaOS-performance |
| SigmaOS-mint-reader | Archive | Mint clone; superseded by SigmaOS-xreader |
| SigmaOS-mint-recovery-tool | Archive | Mint clone; superseded by SigmaOS-troubleshooting-tool |
| SigmaOS-mint-repo-manager | Archive | Mint clone; superseded by SigmaOS-repo-manager |
| SigmaOS-mint-repositories | Archive | Mint clone; superseded by SigmaOS-repo-manager |
| SigmaOS-mint-repository-browser | Archive | Mint clone; superseded by SigmaOS-software-manager |
| SigmaOS-mint-repo-sync | Archive | Mint clone; superseded by SigmaOS-repo-manager |
| SigmaOS-mint-resource-manager | Archive | Mint clone; superseded by SigmaOS-system-admin |
| SigmaOS-mint-resource-monitor | Archive | Mint clone; superseded by SigmaOS-system-admin |
| SigmaOS-mint-scanner | Archive | Mint clone; superseded by SigmaOS-simple-scan |
| SigmaOS-mint-scheduler | Archive | Mint clone; superseded by SigmaOS-scheduler |
| SigmaOS-mint-screensavers | Archive | Mint clone; superseded by SigmaOS-screen-saver |
| SigmaOS-mint-search-index | Archive | Mint clone; superseded by SigmaOS-launcher |
| SigmaOS-mint-security | Archive | Mint clone; superseded by SigmaOS-security |
| SigmaOS-mint-security-auditor | Archive | Mint clone; superseded by SigmaOS-security |
| SigmaOS-mint-service-deployer | Archive | Mint clone; superseded by SigmaOS-system |
| SigmaOS-mint-service-manager | Archive | Mint clone; superseded by SigmaOS-system |
| SigmaOS-mint-service-wrapper | Archive | Mint clone; superseded by SigmaOS-system |
| SigmaOS-mint-session-logger | Archive | Mint clone; superseded by SigmaOS-session-manager |
| SigmaOS-mint-session-manager | Archive | Mint clone; superseded by SigmaOS-session-manager |
| SigmaOS-mint-shell | Archive | Mint clone; superseded by SigmaOS-zsh/fish |
| SigmaOS-mint-signal-handler | Archive | Mint clone; superseded by SigmaOS-kernel |
| SigmaOS-mint-snap-store | Archive | Mint clone; superseded by SigmaOS-software-manager |
| SigmaOS-mint-software-center | Archive | Mint clone; superseded by SigmaOS-software-manager |
| SigmaOS-mint-software-manager | Archive | Mint clone; superseded by SigmaOS-software-manager |
| SigmaOS-mint-storage-manager | Archive | Mint clone; superseded by SigmaOS-filesystem |
| SigmaOS-mint-support | Archive | Mint clone; superseded by SigmaOS-docs |
| SigmaOS-mint-system-adjustments | Archive | Mint clone; superseded by SigmaOS-control-center |
| SigmaOS-mint-system-backup | Archive | Mint clone; superseded by SigmaOS-backup-tool |
| SigmaOS-mint-system-configurator | Archive | Mint clone; superseded by SigmaOS-control-center |
| SigmaOS-mint-system-info | Archive | Mint clone; superseded by SigmaOS-control-center |
| SigmaOS-mint-system-look | Archive | Mint clone; superseded by SigmaOS-theme |
| SigmaOS-mint-system-monitor | Archive | Mint clone; superseded by SigmaOS-system-admin |
| SigmaOS-mint-system-optimizer | Archive | Mint clone; superseded by SigmaOS-performance |
| SigmaOS-mint-system-security | Archive | Mint clone; superseded by SigmaOS-security |
| SigmaOS-mint-system-tracer | Archive | Mint clone; superseded by SigmaOS-dev-tools |
| SigmaOS-mint-system-updater | Archive | Mint clone; superseded by SigmaOS-update |
| SigmaOS-mint-system-updater-v2 | Archive | Mint clone; superseded by SigmaOS-update |
| SigmaOS-mint-task-manager | Archive | Mint clone; superseded by SigmaOS-system-admin |
| SigmaOS-mint-task-scheduler | Archive | Mint clone; superseded by SigmaOS-system |
| SigmaOS-mint-terminal | Archive | Mint clone; superseded by SigmaOS-terminal |
| SigmaOS-mint-terminal-emulator | Archive | Mint clone; superseded by SigmaOS-terminal |
| SigmaOS-mint-terminal-text-effects | Archive | Mint clone; superseded by SigmaOS-terminal-text-effects |
| SigmaOS-mint-theme | Archive | Mint clone; superseded by SigmaOS-theme |
| SigmaOS-mint-theme-collection | Archive | Mint clone; superseded by SigmaOS-theme-registry |
| SigmaOS-mint-theme-installer | Archive | Mint clone; superseded by SigmaOS-theme-registry |
| SigmaOS-mint-theme-settings | Archive | Mint clone; superseded by SigmaOS-control-center |
| SigmaOS-mint-themes-manager | Archive | Mint clone; superseded by SigmaOS-theme-registry |
| SigmaOS-mint-tools | Archive | Mint clone; superseded by SigmaOS-common |
| SigmaOS-mint-transaction-manager | Archive | Mint clone; superseded by SigmaOS-package-manager |
| SigmaOS-mint-translations | Archive | Mint clone; superseded by SigmaOS-translations |
| SigmaOS-mint-uninstaller | Archive | Mint clone; superseded by SigmaOS-software-manager |
| SigmaOS-mint-update-manager | Archive | Mint clone; superseded by SigmaOS-update |
| SigmaOS-mint-update-manager-v2 | Archive | Mint clone; superseded by SigmaOS-update |
| SigmaOS-mint-update-notifier | Archive | Mint clone; superseded by SigmaOS-update |
| SigmaOS-mint-updater-scheduler | Archive | Mint clone; superseded by SigmaOS-update |
| SigmaOS-mint-user-guide | Archive | Mint clone; superseded by SigmaOS-docs |
| SigmaOS-mint-video | Archive | Mint clone; superseded by SigmaOS-player |
| SigmaOS-mint-voice-manager | Archive | Mint clone; no scope defined |
| SigmaOS-mint-volume-control | Archive | Mint clone; superseded by SigmaOS-indicator-sound |
| SigmaOS-mint-volume-controller | Archive | Mint clone; superseded by SigmaOS-indicator-sound |
| SigmaOS-mint-wallpaper | Archive | Mint clone; superseded by SigmaOS-wallpaper-package |
| SigmaOS-mint-wallpaper-setup | Archive | Mint clone; superseded by SigmaOS-wallpaper-package |
| SigmaOS-mint-wifi-manager | Archive | Mint clone; superseded by SigmaOS-wifi |
| SigmaOS-mint-xfce | Archive | Mint clone; XFCE not in scope |
| SigmaOS-omarchy-* (all remaining) | Archive | Omarchy sub-repos not mapped to canonical repos above; superseded by SigmaOS canonical repos |
| SigmaOS-arch-* (all remaining) | Archive | Arch sub-repos not mapped to canonical repos above; superseded by SigmaOS canonical repos |
| SigmaOS-ab | Archive | Placeholder; no scope |
| SigmaOS-abduco | Archive | abduco clone; no scope |
| SigmaOS-adventure | Archive | Game placeholder; no scope |
| SigmaOS-airbnb | Archive | Not relevant |
| SigmaOS-alchemy | Archive | Placeholder |
| SigmaOS-bike | Archive | Placeholder; no scope |
| SigmaOS-boat | Archive | Placeholder; no scope |
| SigmaOS-building | Archive | Placeholder; no scope |
| SigmaOS-car | Archive | Placeholder; no scope |
| SigmaOS-cinema | Archive | Placeholder; no scope |
| SigmaOS-conference | Archive | Placeholder; no scope |
| SigmaOS-cooking | Archive | Placeholder; no scope |
| SigmaOS-cookbook | Archive | Placeholder; no scope |
| SigmaOS-crafting | Archive | Placeholder; no scope |
| SigmaOS-cuisine | Archive | Placeholder; no scope |
| SigmaOS-dance | Archive | Placeholder; no scope |
| SigmaOS-diet | Archive | Placeholder; no scope |
| SigmaOS-drink | Archive | Placeholder; no scope |
| SigmaOS-dyeing | Archive | Placeholder; no scope |
| SigmaOS-enchanting | Archive | Placeholder; no scope |
| SigmaOS-event | Archive | Placeholder; no scope |
| SigmaOS-farming | Archive | Placeholder; no scope |
| SigmaOS-festival | Archive | Placeholder; no scope |
| SigmaOS-fitness | Archive | Placeholder; no scope |
| SigmaOS-food | Archive | Placeholder; no scope |
| SigmaOS-forestry | Archive | Placeholder; no scope |
| SigmaOS-gadget | Archive | Placeholder; no scope |
| SigmaOS-gardening | Archive | Placeholder; no scope |
| SigmaOS-gathering | Archive | Placeholder; no scope |
| SigmaOS-grocery | Archive | Placeholder; no scope |
| SigmaOS-guesthouse | Archive | Placeholder; no scope |
| SigmaOS-helicopter | Archive | Placeholder; no scope |
| SigmaOS-hospitality | Archive | Placeholder; no scope |
| SigmaOS-hostel | Archive | Placeholder; no scope |
| SigmaOS-hotel | Archive | Placeholder; no scope |
| SigmaOS-job | Archive | Placeholder generic; superseded by SigmaOS-system |
| SigmaOS-laboratory | Archive | Placeholder; no scope |
| SigmaOS-leatherworking | Archive | Placeholder; no scope |
| SigmaOS-lodging | Archive | Placeholder; no scope |
| SigmaOS-mall | Archive | Placeholder; no scope |
| SigmaOS-meeting | Archive | Placeholder; no scope |
| SigmaOS-mining | Archive | Placeholder; no scope |
| SigmaOS-nutrition | Archive | Placeholder; no scope |
| SigmaOS-outlet | Archive | Placeholder; no scope |
| SigmaOS-pharmacy | Archive | Placeholder; no scope |
| SigmaOS-plane | Archive | Placeholder; no scope |
| SigmaOS-platformer | Archive | Game placeholder; no scope |
| SigmaOS-racing | Archive | Game placeholder; no scope |
| SigmaOS-recipe | Archive | Placeholder; no scope |
| SigmaOS-resort | Archive | Placeholder; no scope |
| SigmaOS-rpg | Archive | Game placeholder; no scope |
| SigmaOS-school | Archive | Placeholder; no scope |
| SigmaOS-ship | Archive | Placeholder; no scope |
| SigmaOS-shooter | Archive | Game placeholder; no scope |
| SigmaOS-shop | Archive | Placeholder; no scope |
| SigmaOS-shopping | Archive | Placeholder; no scope |
| SigmaOS-simulation | Archive | Placeholder; no scope |
| SigmaOS-smithing | Archive | Placeholder; no scope |
| SigmaOS-spaceship | Archive | Placeholder; no scope |
| SigmaOS-spinning | Archive | Placeholder; no scope |
| SigmaOS-sports | Archive | Placeholder; no scope |
| SigmaOS-strategy | Archive | Game placeholder; no scope |
| SigmaOS-submarine | Archive | Placeholder; no scope |
| SigmaOS-supermarket | Archive | Placeholder; no scope |
| SigmaOS-tailoring | Archive | Placeholder; no scope |
| SigmaOS-theater | Archive | Placeholder; no scope |
| SigmaOS-tourism | Archive | Placeholder; no scope |
| SigmaOS-train | Archive | Placeholder; no scope |
| SigmaOS-transportation | Archive | Placeholder; no scope |
| SigmaOS-travel | Archive | Placeholder; no scope |
| SigmaOS-university | Archive | Placeholder; no scope |
| SigmaOS-weaving | Archive | Placeholder; no scope |
| SigmaOS-we-can-fix-everything | Archive | Marketing slogan; not a real repo scope |
| SigmaOS-caligula | Archive | Unclear scope; no active development |
| SigmaOS-captain | Archive | Unclear scope; no active development |
| SigmaOS-continue-check | Archive | CI utility placeholder; no active development |
| SigmaOS-lumon.nvim | Archive | Neovim theme; superseded by SigmaOS-nvim |
| SigmaOS-aether.nvim | Archive | Neovim theme; superseded by SigmaOS-nvim |
| SigmaOS-boat | Archive | Placeholder |
| SigmaOS-caligula | Archive | Placeholder |
| SigmaOS-elsewhen | Archive | Placeholder; unclear scope |
| SigmaOS-etc | Archive | /etc placeholder; merge was noted above |
| SigmaOS-herdr | Archive | Herdr bare placeholder; superseded by SigmaOS-herdr-runtime |
| SigmaOS-hype | Archive | Marketing; no code scope |
| SigmaOS-jargonaut | Archive | Jargon tool; no active dev |
| SigmaOS-jsework | Archive | JS work placeholder; unclear scope |
| SigmaOS-launch-ready | Archive | CI readiness placeholder |
| SigmaOS-location | Archive | Location service; no scope defined |
| SigmaOS-lumon.nvim | Archive | Neovim theme |
| SigmaOS-mit-editor | Archive | Typo/alias for text editor; superseded |
| SigmaOS-poc-repos-created | Archive | Internal tracking; not a real repo |
| SigmaOS-thingy | Archive | No scope |
| SigmaOS-try | Archive | Try-before-install; experimental and unclear |
| SigmaOS-we-can-fix-everything | Archive | Marketing; not a real repo |
| SigmaOS-Github-Action | Archive | Duplicate capitalization variant; superseded by SigmaOS-github-action |
| SigmaOS-github-actions | Archive | Plural alias; superseded by SigmaOS-github-action |
| SigmaOS-Node.js-Binary | Archive | Dot in name; superseded by SigmaOS-nodejs-binary |
| SigmaOS-GTK+ | Archive | + in name (invalid identifier); superseded by SigmaOS-GTK |
| SigmaOS-cpuid | Archive | CPUID tool; merge into SigmaOS-cpu-profile |
| SigmaOS-extended-distro | Archive | Unclear; no scope |
| SigmaOS-extra-plugins | Archive | Plugin dump; superseded by SigmaOS-plugin-registry |
| SigmaOS-distro-inspirations | Archive | Already listed under merge into docs |
| SigmaOS-disk-utility | Keep active but optional | Already classified above |
| SigmaOS-omasnap | Archive | Omarchy snap; already noted above |

---

## Summary

| Classification | Count (approx.) |
|---|---|
| Keep active and production | 42 (30 core + 12 performance) |
| Keep active but optional | ~110 |
| Merge into canonical repo | ~500+ |
| Prototype / optional | ~40 |
| Archive | ~900+ (mint clones, arch sub-repos, omarchy sub-repos, placeholders) |

> **Note:** The numbered feature stubs (`SigmaOS-system-feature-1` through `-87`, `SigmaOS-core-feature-1` through `-87`, `SigmaOS-user-feature-1` through `-87`) account for ~261 entries classified as "Merge into canonical repo." The mint-*, arch-*, omarchy-* clone families account for the majority of archive entries. All merge targets follow the mandatory merge rules table at the top.
