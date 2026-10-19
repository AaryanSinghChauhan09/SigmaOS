# SigmaOS Official Documentation & Architecture Wiki

> **Welcome to the SigmaOS Documentation Wiki.**  
> SigmaOS is a next-generation sovereign operating system combining microkernel performance, memory safety, and full ecosystem parity with **Linux**, **FreeBSD**, **OpenBSD**, and **Omarchy Linux**. This wiki serves as the authoritative technical reference, architecture guide, and subsystem catalog.

---

## 🏛️ Subsystem Portals (100 Canonical Component Guides)

| Portal | Subsystems | Key Highlights |
| :--- | :--- | :--- |
| [Core Kernel & Hardware](#1-core-kernel--hardware) | 16 Subsystems | Preemptive Scheduler, Paging, APIC, PCIe AER, IOMMU DMA, ACPI AML, cpufreq, Sysctl, Udev, Watchdog |
| [Storage & Filesystems](#2-storage-volumes--filesystems) | 8 Subsystems | POSIX VFS, ZFS / HAMMER2 Transactional FS, Device Mapper, LUKS2, ZRAM Swap, FUSE Userspace FS, SMART |
| [Init & Service Supervisors](#3-init-supervisors--system-services) | 9 Subsystems | Dependency DAG init, SysV/BSD rc.d parity, cgroups v2 resource quotas, system monitoring, dynamic linker |
| [Security & Cryptography](#4-security-sandboxing--cryptography) | 10 Subsystems | Mandatory Access Control (SELinux/AppArmor), Namespaces & Jails, Seccomp-BPF, TPM 2.0, eBPF Tracing |
| [Networking & Wireless](#5-networking--wireless-communications) | 7 Subsystems | Zero-copy TCP/IP, NetworkManager / IWD, 802.11 WiFi, Bluetooth, Dynamic DNS, NSSwitch / DNSSEC, GeoClue |
| [Desktop & Wayland Ecosystem](#6-desktop-environment--wayland-compositor) | 19 Subsystems | Wayland Compositor, Hyprland Tiling, UWSM Session, Waybar, Walker, SwayNC, Omakase Themes, Night Light |
| [Multimedia & Audio](#7-multimedia--audio-subsystem) | 6 Subsystems | PipeWire / ALSA audio, Wireplumber ducking policies, VA-API/VDPAU, Grimblast, WF-Recorder |
| [Terminal, Shell & CLI](#8-terminal-shell--command-line) | 8 Subsystems | Alacritty/Kitty GPU terminal, Starship prompt, Bash/Zsh/Fish parity, GNU/BSD Coreutils, Serial PTY |
| [Gaming & Acceleration](#9-gaming-acceleration--virtualization) | 9 Subsystems | GameMode scheduler, MangoHud HUD, VKD3D-Proton Vulkan, DXVK, MicroVMs, VFIO Passthrough |
| [Package & Distro Engineering](#10-package-management--maintenance) | 8 Subsystems | `sigpkg` binary packages, Ports build system, Delta Reproducible Builds, Calamares installer, UKI boot |

---

### 1. Core Kernel & Hardware
- [Kernel Core Component Guide](./Kernel-Core-Component-Guide.md) &ndash; Sovereign microkernel core, bootstrap, and multi-core AP bring-up.
- [Operating System Structure Component Guide](./Operating-System-Structure-Component-Guide.md) &ndash; Modular hybrid topology, microkernel isolation, and lifecycle traits.
- [Kernel Scheduler Component Guide](./Kernel-Scheduler-Component-Guide.md) &ndash; CFS / EEVDF fair scheduling, real-time deadlines, and Linux/BSD comparisons.
- [Scheduler Component Guide](./Scheduler-Component-Guide.md) &ndash; Multi-queue CFS / BORE low-latency preemptive task scheduler.
- [Process Management Component Guide](./Process-Management-Component-Guide.md) &ndash; Process lifecycle, PCB state machine, cloning, and zombie reaping.
- [Memory Management Component Guide](./Memory-Management-Component-Guide.md) &ndash; Paging, slab allocator, NUMA memory tiers, and page cache.
- [Virtual Memory Manager Component Guide](./Virtual-Memory-Manager-Component-Guide.md) &ndash; Demand paging, Copy-on-Write fork, ASLR, and mmap sealing.
- [Concurrency & Deadlocks Component Guide](./Concurrency-And-Deadlocks-Component-Guide.md) &ndash; Hierarchical spinlocks, witness cycle detection, and lockless RCU.
- [Syscall Interface Component Guide](./Syscall-Interface-Component-Guide.md) &ndash; Fast syscall entry dispatch, vDSO time page, and seccomp filters.
- [Tty Management Component Guide](./Tty-Management-Component-Guide.md) &ndash; Terminal line disciplines, PTY /dev/ptmx multiplexing, and virtual consoles.
- [PCI Express Bus Enumeration Component Guide](./Pci-Express-Bus-Enumeration-Component-Guide.md) &ndash; PCIe root complex enumeration, MSI/MSI-X, and AER error logging.
- [DMA & IOMMU Direct Memory Access Component Guide](./Dma-Iommu-Direct-Memory-Access-Component-Guide.md) &ndash; Zero-copy scatter-gather DMA memory manager and IOMMU domain translation.
- [ACPI Power Resource Parser Component Guide](./Acpi-Power-Resource-Parser-Component-Guide.md) &ndash; ACPI AML bytecode interpreter, thermal trip points, and S0-S5 power states.
- [CPU Frequency Scaling Governor Component Guide](./Cpu-Frequency-Scaling-Governor-Component-Guide.md) &ndash; Dynamic P-state scaling across schedutil, performance, and powersave.
- [Thermal Cooling Component Guide](./Thermal-Cooling-Component-Guide.md) &ndash; Passive and active fan curves, throttling thresholds, and temperature telemetry.
- [Hardware HAL Drivers Component Guide](./Hardware-Hal-Drivers-Component-Guide.md) &ndash; Unified driver model and device tree bus abstractions.
- [USB Subsystem & HID Storage Component Guide](./Usb-Subsystem-Hid-Mass-Storage-Component-Guide.md) &ndash; xHCI/EHCI host controllers, HID input events, and USB Mass Storage class.
- [Interrupt APIC Component Guide](./Interrupt-Apic-Component-Guide.md) &ndash; Local APIC, I/O APIC, MSI routing, and interrupt vector arbitration.
- [Timekeeping Component Guide](./Timekeeping-Component-Guide.md) &ndash; TSC / HPET clock sources, high-resolution timers, and monotonic timekeeping.
- [Firmware & Microcode Updater Component Guide](./Firmware-Microcode-Updater-Component-Guide.md) &ndash; CPU microcode patch loader and vendor firmware verification.
- [Sysctl Kernel Tunables Component Guide](./Sysctl-Kernel-Tunables-Component-Guide.md) &ndash; Runtime kernel parameter tuning, /proc/sys hierarchy, and network buffers.
- [Udev & Devd Hotplug Device Component Guide](./Udev-Devd-Hotplug-Device-Component-Guide.md) &ndash; Dynamic /dev node synthesizer, netlink uevent dispatcher, and persistent symlinks.
- [Watchdog Hardware Recovery Component Guide](./Watchdog-Hardware-Recovery-Component-Guide.md) &ndash; Hardware watchdog timers, ping heartbeat daemon, and panic recovery reset.
- [CachyOS BORE Scheduler Component Guide](./Cachyos-Bore-Component-Guide.md) &ndash; Interactive desktop responsiveness and low-latency scheduling.

---

### 2. Storage, Volumes & Filesystems
- [Filesystem VFS Component Guide](./Filesystem-Vfs-Component-Guide.md) &ndash; Virtual Filesystem switch, inode cache, and POSIX path resolution.
- [ZFS & HAMMER2 Transactional FS Component Guide](./Zfs-Hammer2-Transactional-Fs-Component-Guide.md) &ndash; Copy-on-Write (CoW) transactional storage, snapshots, and checksum verification.
- [Device Mapper, LVM & Cryptsetup Component Guide](./Device-Mapper-Lvm-Cryptsetup-Component-Guide.md) &ndash; Linear/striped volume mapping, LUKS2 AES-XTS encryption, and dm-verity.
- [Swap ZRAM Compression Component Guide](./Swap-Zram-Compression-Component-Guide.md) &ndash; In-memory compressed swap devices using Zstandard and LZ4 compression.
- [FUSE Userspace Filesystem Component Guide](./Fuse-Userspace-Filesystem-Component-Guide.md) &ndash; Kernel /dev/fuse request multiplexer for unprivileged user filesystems.
- [Storage Component Guide](./Storage-Component-Guide.md) &ndash; Block I/O scheduler, NVMe queues, AHCI SATA, and asynchronous request dispatch.
- [SMART Disk Health Component Guide](./Smart-Disk-Health-Component-Guide.md) &ndash; Disk telemetry monitoring, wear-level statistics, and thermal health reporting.
- [Crash Dump Core Component Guide](./Crash-Dump-Core-Component-Guide.md) &ndash; Kernel panic crash dump captures, kdump memory reservations, and stack traces.

---

### 3. Init Supervisors & System Services
- [Init Supervisor Component Guide](./Init-Supervisor-Component-Guide.md) &ndash; PID 1 process supervision, service graph dependencies, and zombie reaping.
- [Init Targets State Component Guide](./Init-Targets-State-Component-Guide.md) &ndash; System runlevels, boot milestones, emergency shell, and target synchronization.
- [Systemd & SysV RC Scripts Component Guide](./Systemd-Sysv-Rc-Init-Scripts-Component-Guide.md) &ndash; Socket activation, dependency DAGs, and SysV / BSD rc.d compatibility.
- [Cgroups Resource Governor Component Guide](./Cgroups-Resource-Governor-Component-Guide.md) &ndash; Cgroups v2 controllers for CPU shares, memory caps, and I/O throttling.
- [System Monitoring Component Guide](./System-Monitoring-Component-Guide.md) &ndash; Real-time CPU, RAM, disk, and network instrumentation telemetry.
- [Logging Component Guide](./Logging-Component-Guide.md) &ndash; Structured circular journal buffer, syslog compatibility, and log rotation.
- [IPC Component Guide](./Ipc-Component-Guide.md) &ndash; Shared memory, Unix domain sockets, pipes, and message queues.
- [Dynamic Linker Loader Component Guide](./Dynamic-Linker-Loader-Component-Guide.md) &ndash; ELF shared object loader, symbol resolution, and lazy binding.
- [System Calls ABI Component Guide](./System-Calls-Abi-Component-Guide.md) &ndash; POSIX syscall interface, register conventions, and error translation.

---

### 4. Security, Sandboxing & Cryptography
- [Security Component Guide](./Security-Component-Guide.md) &ndash; Defense-in-depth system security architecture and privilege separation.
- [Mandatory Access Control (MAC) Component Guide](./Selinux-Apparmor-Mac-Component-Guide.md) &ndash; SELinux, AppArmor, and FreeBSD MAC framework security enforcement.
- [Namespaces & Jails Isolation Component Guide](./Namespaces-Isolation-Jails-Component-Guide.md) &ndash; PID/mount/net/IPC namespaces, FreeBSD Jails, and OpenBSD pledge/unveil.
- [Seccomp-BPF Syscall Filter Component Guide](./Seccomp-Bpf-Syscall-Filter-Component-Guide.md) &ndash; Deterministic BPF system call boundary filter reducing attack surfaces.
- [Hardware Security Module (TPM 2.0) Component Guide](./Hardware-Security-Module-Tpm2-Component-Guide.md) &ndash; Measured boot verification and cryptographic key unsealing via TPM 2.0.
- [Cryptography Component Guide](./Cryptography-Component-Guide.md) &ndash; Hardware-accelerated AES-NI, ChaCha20-Poly1305, SHA-256/512, and secure RNG.
- [Keyring Secret Service Component Guide](./Keyring-Secret-Service-Component-Guide.md) &ndash; User secret service, DBus secret API, and kernel keyring keys.
- [Firewall Security Component Guide](./Firewall-Security-Component-Guide.md) &ndash; Stateful packet filtering, NAT, connection tracking, and nftables/pf parity.
- [System Security Audit Component Guide](./System-Security-Audit-Component-Guide.md) &ndash; Real-time audit events, syscall logging, and file integrity validation.
- [System Tracing (eBPF) Component Guide](./System-Tracing-Ebpf-Component-Guide.md) &ndash; In-kernel verified bytecode filters, kprobes, and dynamic tracepoints.
- [FreeBSD Capsicum Sandboxing Component Guide](./Freebsd-Capsicum-Component-Guide.md) &ndash; Object-capability rights masks and capability mode sandbox.

---

### 5. Networking & Wireless Communications
- [Networking Stack Component Guide](./Networking-Stack-Component-Guide.md) &ndash; Zero-copy TCP/UDP/IPv4/IPv6 socket stack with high-throughput ring buffers.
- [NetworkManager & IWD Component Guide](./Network-Manager-Iwd-Component-Guide.md) &ndash; Connection lifecycle manager, DHCP client, and fast roaming daemon.
- [Linux Mint Warpinator Component Guide](./Linuxmint-Warpinator-Component-Guide.md) &ndash; Local peer-to-peer secure network file transfer engine with TLS/gRPC discovery.
- [Wireless WiFi Component Guide](./Wireless-Wifi-Component-Guide.md) &ndash; IEEE 802.11 mac80211 wireless driver model, WPA3 SAE, and scan engine.
- [Bluetooth Stack Component Guide](./Bluetooth-Stack-Component-Guide.md) &ndash; HCI host controller interface, BlueZ protocol compatibility, and BLE GATT services.
- [Dynamic DNS Resolver Component Guide](./Dynamic-Dns-Resolver-Component-Guide.md) &ndash; Asynchronous caching DNS resolver with DNS-over-TLS/HTTPS support.
- [NSSwitch & DNSSEC Component Guide](./Dnssec-Nsswitch-Name-Service-Component-Guide.md) &ndash; Name service switch dispatcher and DNSSEC cryptographic validation.
- [Location (GeoClue) Component Guide](./Location-Geoclue-Component-Guide.md) &ndash; D-Bus location service, WiFi beacon geolocation, and GPS telemetry.

---

### 6. Desktop Environment & Wayland Compositor
- [Wayland Compositor Component Guide](./Wayland-Compositor-Component-Guide.md) &ndash; Core Wayland display server, wlroots protocol support, and hardware surfaces.
- [Hyprland Tiling Component Guide](./Hyprland-Tiling-Component-Guide.md) &ndash; Dynamic tiling Wayland compositor with fluid physics, gestures, and window rules.
- [Desktop Environment Component Guide](./Desktop-Environment-Component-Guide.md) &ndash; Integrated sovereign desktop shell, wallpaper manager, and panel dispatch.
- [Linux Mint Slick Greeter Component Guide](./Linuxmint-Slick-Greeter-Component-Guide.md) &ndash; Elegant, secure Display Manager greeter with PAM authentication and guest sessions.
- [Session Manager (UWSM) Component Guide](./Session-Manager-Uwsm-Component-Guide.md) &ndash; Universal Wayland Session Manager with systemd user unit wrapping.
- [Waybar Status Bar Component Guide](./Waybar-Status-Bar-Component-Guide.md) &ndash; Modular multi-monitor status bar with dynamic battery, net, and workspace modules.
- [Waybar Custom CSS Component Guide](./Waybar-Custom-Css-Component-Guide.md) &ndash; Pixel-perfect themes, gradients, border radiuses, and Omarchy aesthetic styling.
- [Walker Launcher Component Guide](./Walker-Launcher-Component-Guide.md) &ndash; Sub-millisecond application launcher with fuzzy matching and plugin runner.
- [Wofi Application Menu Component Guide](./Wofi-Application-Menu-Component-Guide.md) &ndash; Fallback dmenu/rofi-compatible Wayland launcher and command prompt.
- [Quickshell Widgets Component Guide](./Quickshell-Widgets-Component-Guide.md) &ndash; Reactive QML-based desktop overlays, HUDs, and system flyouts.
- [SwayNC Notification Component Guide](./Swaync-Notification-Component-Guide.md) &ndash; Wayland notification daemon with interactive widget drawer and do-not-disturb.
- [Waypaper Wallpaper Component Guide](./Waypaper-Wallpaper-Component-Guide.md) &ndash; GUI wallpaper selector frontend supporting animated and multi-monitor backdrops.
- [Hyprpaper Wallpaper Daemon Component Guide](./Hyprpaper-Wallpaper-Daemon-Component-Guide.md) &ndash; Zero-overhead Wayland wallpaper rendering engine via DMA-BUF textures.
- [Hyprlock Screen Shield Component Guide](./Hyprlock-Screen-Shield-Component-Guide.md) &ndash; Fast, secure Wayland lock screen with PAM authentication and aesthetic styling.
- [Hypridle Timeout Component Guide](./Hypridle-Timeout-Component-Guide.md) &ndash; Multi-tier idle detection daemon managing screen dimming, lock, and suspend.
- [Wlogout Session Component Guide](./Wlogout-Session-Component-Guide.md) &ndash; Wayland logout, reboot, suspend, and power-off prompt interface.
- [Clipboard Manager (Cliphist) Component Guide](./Clipboard-Manager-Component-Guide.md) &ndash; Wayland clipboard history ring supporting text, images, and fuzzy search.
- [Night Light & Gamma Component Guide](./Night-Light-Gamma-Component-Guide.md) &ndash; Smooth color temperature transitions matching sunrise/sunset times.
- [Brightness & Backlight Component Guide](./Brightness-Backlight-Component-Guide.md) &ndash; Exponential PWM backlight brightness control and DDC/CI monitor adjustment.
- [Theme Omakase Component Guide](./Theme-Omakase-Component-Guide.md) &ndash; Curated, synchronized dynamic theme switcher across terminal, GTK, and Waybar.

---

### 7. Multimedia & Audio Subsystem
- [Audio Subsystem Component Guide](./Audio-Subsystem-Component-Guide.md) &ndash; Low-latency PipeWire / ALSA audio architecture with graph-based routing.
- [Wireplumber Audio Policy Component Guide](./Wireplumber-Audio-Policy-Component-Guide.md) &ndash; Policy engine for automatic stream ducking, role routing, and device switching.
- [Media Codec Hardware Component Guide](./Media-Codec-Hardware-Component-Guide.md) &ndash; Hardware-accelerated decode/encode (H.264, HEVC, AV1, VP9) via VA-API/VDPAU.
- [Grimblast Screenshot Component Guide](./Grimblast-Screenshot-Component-Guide.md) &ndash; Low-latency region, active window, and full-screen Wayland capture utility.
- [WF-Recorder Screencast Component Guide](./Wf-Recorder-Screencast-Component-Guide.md) &ndash; Hardware-accelerated Wayland screen recorder via DMA-BUF and FFmpeg.
- [Graphics GPU Component Guide](./Graphics-Gpu-Component-Guide.md) &ndash; DRM/KMS kernel graphics driver, GEM/TTM memory manager, and Vulkan runtime.

---

### 8. Terminal, Shell & Command Line
- [Alacritty & Kitty Terminal Component Guide](./Alacritty-Kitty-Terminal-Component-Guide.md) &ndash; GPU-accelerated terminals with truecolor, ligature, and sixel graphics.
- [Starship Prompt Component Guide](./Starship-Prompt-Component-Guide.md) &ndash; Ultra-fast cross-shell prompt with git status, runtime versions, and exit codes.
- [Bash, Zsh & Fish Shell Component Guide](./Bash-Zsh-Fish-Shell-Component-Guide.md) &ndash; Full POSIX shell compliance with autocompletion, syntax highlighting, and history.
- [Coreutils GNU & BSD Parity Component Guide](./Coreutils-Gnu-Bsd-Parity-Component-Guide.md) &ndash; Sovereign implementations of essential userland utilities (`ls`, `cp`, `grep`).
- [Serial TTY & Virtual Terminal Component Guide](./Serial-Tty-Pty-Virtual-Console-Component-Guide.md) &ndash; POSIX termios line discipline, /dev/ptmx multiplexer, and UART serial console.
- [Internationalization (i18n) Component Guide](./Internationalization-I18n-Component-Guide.md) &ndash; UTF-8 locale engine, timezone database, and multibyte font rendering.
- [Input Accessibility Component Guide](./Input-Accessibility-Component-Guide.md) &ndash; Sticky keys, bounce keys, mousekeys, and on-screen virtual keyboard support.
- [Accessibility Screen Reader Component Guide](./Accessibility-Screen-Reader-Component-Guide.md) &ndash; AT-SPI2 accessibility bridge, text-to-speech synthesis, and braille output.

---

### 9. Gaming, Acceleration & Virtualization
- [GameMode Daemon Component Guide](./Gamemode-Daemon-Component-Guide.md) &ndash; Dynamic CPU governor optimizer, process renicing, and GPU boost mode.
- [MangoHud Gaming Component Guide](./Mangohud-Gaming-Component-Guide.md) &ndash; Real-time Vulkan/OpenGL gaming overlay showing FPS, frame timings, and temperatures.
- [Lutris & Heroic Gaming Component Guide](./Lutris-Heroic-Gaming-Component-Guide.md) &ndash; Game library manager and runner for native, Proton, and wineprefix games.
- [VKD3D-Proton Vulkan Component Guide](./Vkd3d-Proton-Vulkan-Component-Guide.md) &ndash; Direct3D 12 to Vulkan translation layer for modern sovereign gaming.
- [DXVK Direct3D 11 Component Guide](./Dxvk-D3d11-Translation-Component-Guide.md) &ndash; Direct3D 9/10/11 translation to Vulkan with pipeline caching.
- [Virtualization Component Guide](./Virtualization-Component-Guide.md) &ndash; KVM-compatible hardware hypervisor virtualization and MicroVM scheduler.
- [Container Runtime Component Guide](./Container-Runtime-Component-Guide.md) &ndash; OCI-compliant container execution engine with overlayfs and seccomp filtering.
- [Container Podman Rootless Component Guide](./Container-Podman-Rootless-Component-Guide.md) &ndash; Rootless container execution using user namespaces and slirp4netns.
- [PCI Passthrough (VFIO) Component Guide](./Pci-Passthrough-Vfio-Component-Guide.md) &ndash; Direct PCIe device assignment and IOMMU domain isolation for guest VMs.

---

### 10. Package Management & Maintenance
- [Arch ALPM Pacman Component Guide](./Arch-Alpm-Pacman-Component-Guide.md) &ndash; Sovereign ALPM package database synchronization, dependency resolution, and pacman parity.
- [Arch Mkinitcpio Component Guide](./Arch-Mkinitcpio-Component-Guide.md) &ndash; Modular early userspace initramfs and Unified Kernel Image (UKI) builder.
- [Arch Install Scripts Component Guide](./Arch-Install-Scripts-Component-Guide.md) &ndash; System bootstrap (pacstrap), declarative fstab (genfstab), and isolated arch-chroot.
- [Package Management Component Guide](./Package-Management-Component-Guide.md) &ndash; `sigpkg` sovereign package manager with cryptographic signing and rollback.
- [Package Build Ports Component Guide](./Package-Build-Ports-Component-Guide.md) &ndash; BSD-style ports tree and PKGBUILD recipe build infrastructure.
- [Package Delta & Reproducible Build Component Guide](./Package-Delta-Reproducible-Build-Component-Guide.md) &ndash; Bit-for-bit deterministic builds and zsync binary delta updates.
- [Live ISO Installer (Calamares) Component Guide](./Live-Iso-Installer-Calamares-Component-Guide.md) &ndash; Graphical system installer with partition resizing and locale selection.
- [Bootloader Component Guide](./Bootloader-Component-Guide.md) &ndash; UEFI / BIOS multi-boot loader supporting unified kernel images (UKIs).
- [Power Management Component Guide](./Power-Management-Component-Guide.md) &ndash; Sleep state transitions, suspend-to-idle, and battery saver profiles.
- [Printing (CUPS) Component Guide](./Printing-Cups-Component-Guide.md) &ndash; IPP printing daemon, raster filters, and driverless AirPrint discovery.
- [System Automation Component Guide](./System-Automation-Component-Guide.md) &ndash; Cron scheduler, system event listeners, and unattended maintenance triggers.
- [Linux Mint & Omarchy Complete Gap Closure](./Mint-and-Omarchy-Complete-Gap-Closure-Components.md) &ndash; 14-subsystem parity engine achieving 100% gap closure against Linux Mint 22 (X-Apps, Hypnotix, WebApps, Drivers) and Omarchy Linux (Swww, Wallust, Quickshell, Ananicy, Walker).
- [Omarchy Linux Desktop Parity & Distro Gap Closure Roadmap](./43-Omarchy-Linux-Desktop-Parity-And-Distro-Gap-Closure-Roadmap.md) &ndash; Complete desktop environment parity roadmap covering Hyprpaper, Hypridle, Swaync, Waypaper, Wallust, Wlogout, and Wofi.
- [Linux Mint & Omarchy Coexistence & Interoperability](./Mint-and-Omarchy-Coexistence-and-Interoperability-Components.md) &ndash; Containerized application bridge, dynamic Hyprland/Waybar configuration hot-reload, and High-Assurance Niche Governor.
- [Linux Mint & Omarchy Federated Desktop](./Mint-and-Omarchy-Federated-Desktop-Components.md) &ndash; Warpinator line-rate P2P protocol, Backup selection manifests, Hyprland master-stack tiling, and Swww IPC socket engine.

---

## 📚 Maintenance, Governance & Architecture
- [Why SigmaOS Cannot Defeat Linux Mint & Omarchy Linux](./Why-SigmaOS-Cannot-Defeat-Linux-Mint-and-Omarchy.md) &ndash; Principled engineering post-mortem examining Gall's Law, Worse is Better, the Ninety-Ninety Rule, hardware quirk mass, and ecosystem scale.
- [Future Development Roadmap: BSD, Linux & Craft Ecosystem](./Future-Development-Roadmap-BSD-Craft-Linux.md) &ndash; Strategic roadmap integrating DragonFly/Free/OpenBSD, Arch/Cachy/Debian Linux, and the 21-pillar Sovereign Craft suite.
- [AI Development Agent Operating Manual](./AI-Development-Agent-Operating-Manual.md) &ndash; Binding 5-step operational lifecycle, branch & PR merging via 'Theirs', and OOPS/SOLID programming principles.
- [AI Agent Maintenance Instructions](./AI-Agent-Maintenance-Instructions.md) &ndash; Canonical rules, standard library authorization, and One-File-Per-Component invariant.
- [Contributing Guidelines](./CONTRIBUTING.md) &ndash; Submitting patches, test requirements, and coding conventions.
- [Main Repository Codebase](https://github.com/AaryanSinghChauhan09/SigmaOS) &ndash; Official SigmaOS GitHub repository.
