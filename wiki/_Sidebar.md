### [SigmaOS Wiki](./Home.md)
**The Sovereign Linux & BSD Parity Distribution**

---

### [Core Kernel & Hardware](./Home.md#1-core-kernel--hardware)
- [Kernel Core](./Kernel-Core-Component-Guide.md)
- [Scheduler](./Scheduler-Component-Guide.md)
- [Memory Management](./Memory-Management-Component-Guide.md)
- [PCIe Bus Enumeration](./Pci-Express-Bus-Enumeration-Component-Guide.md)
- [IOMMU & DMA Engine](./Dma-Iommu-Direct-Memory-Access-Component-Guide.md)
- [ACPI Power & AML](./Acpi-Power-Resource-Parser-Component-Guide.md)
- [CPU Frequency Scaling](./Cpu-Frequency-Scaling-Governor-Component-Guide.md)
- [Thermal Cooling](./Thermal-Cooling-Component-Guide.md)
- [Hardware HAL Drivers](./Hardware-Hal-Drivers-Component-Guide.md)
- [USB & HID Subsystem](./Usb-Subsystem-Hid-Mass-Storage-Component-Guide.md)
- [Interrupts & APIC](./Interrupt-Apic-Component-Guide.md)
- [Timekeeping](./Timekeeping-Component-Guide.md)
- [Firmware & Microcode](./Firmware-Microcode-Updater-Component-Guide.md)
- [Sysctl Kernel Tunables](./Sysctl-Kernel-Tunables-Component-Guide.md)
- [Udev & Devd Hotplug](./Udev-Devd-Hotplug-Device-Component-Guide.md)
- [Hardware Watchdog](./Watchdog-Hardware-Recovery-Component-Guide.md)

---

### [Storage & Filesystems](./Home.md#2-storage-volumes--filesystems)
- [Virtual Filesystem (VFS)](./Filesystem-Vfs-Component-Guide.md)
- [ZFS & HAMMER2 FS](./Zfs-Hammer2-Transactional-Fs-Component-Guide.md)
- [Device Mapper & LUKS](./Device-Mapper-Lvm-Cryptsetup-Component-Guide.md)
- [ZRAM Swap Compression](./Swap-Zram-Compression-Component-Guide.md)
- [FUSE Userspace FS](./Fuse-Userspace-Filesystem-Component-Guide.md)
- [Storage Engine](./Storage-Component-Guide.md)
- [SMART Disk Health](./Smart-Disk-Health-Component-Guide.md)
- [Crash Dump Core](./Crash-Dump-Core-Component-Guide.md)

---

### [Init & Service Supervisors](./Home.md#3-init-supervisors--system-services)
- [Init Supervisor](./Init-Supervisor-Component-Guide.md)
- [Init Targets State](./Init-Targets-State-Component-Guide.md)
- [Systemd & SysV RC Scripts](./Systemd-Sysv-Rc-Init-Scripts-Component-Guide.md)
- [Cgroups Resource Governor](./Cgroups-Resource-Governor-Component-Guide.md)
- [System Monitoring](./System-Monitoring-Component-Guide.md)
- [System Logging](./Logging-Component-Guide.md)
- [IPC & Messaging](./Ipc-Component-Guide.md)
- [Dynamic Linker Loader](./Dynamic-Linker-Loader-Component-Guide.md)
- [System Calls ABI](./System-Calls-Abi-Component-Guide.md)

---

### [Security & Cryptography](./Home.md#4-security-sandboxing--cryptography)
- [Security Architecture](./Security-Component-Guide.md)
- [MAC (SELinux & AppArmor)](./Selinux-Apparmor-Mac-Component-Guide.md)
- [Namespaces & Jails](./Namespaces-Isolation-Jails-Component-Guide.md)
- [Seccomp-BPF Sandbox](./Seccomp-Bpf-Syscall-Filter-Component-Guide.md)
- [TPM 2.0 & Measured Boot](./Hardware-Security-Module-Tpm2-Component-Guide.md)
- [Cryptography](./Cryptography-Component-Guide.md)
- [Keyring Secret Service](./Keyring-Secret-Service-Component-Guide.md)
- [Firewall Security](./Firewall-Security-Component-Guide.md)
- [System Security Audit](./System-Security-Audit-Component-Guide.md)
- [System Tracing (eBPF)](./System-Tracing-Ebpf-Component-Guide.md)

---

### [Networking & Wireless](./Home.md#5-networking--wireless-communications)
- [Networking Stack](./Networking-Stack-Component-Guide.md)
- [NetworkManager & IWD](./Network-Manager-Iwd-Component-Guide.md)
- [Wireless WiFi](./Wireless-Wifi-Component-Guide.md)
- [Bluetooth Stack](./Bluetooth-Stack-Component-Guide.md)
- [Dynamic DNS Resolver](./Dynamic-Dns-Resolver-Component-Guide.md)
- [NSSwitch & DNSSEC](./Dnssec-Nsswitch-Name-Service-Component-Guide.md)
- [Location (GeoClue)](./Location-Geoclue-Component-Guide.md)

---

### [Desktop & Wayland](./Home.md#6-desktop-environment--wayland-compositor)
- [Wayland Compositor](./Wayland-Compositor-Component-Guide.md)
- [Hyprland Tiling](./Hyprland-Tiling-Component-Guide.md)
- [Desktop Environment](./Desktop-Environment-Component-Guide.md)
- [Session Manager (UWSM)](./Session-Manager-Uwsm-Component-Guide.md)
- [Waybar Status Bar](./Waybar-Status-Bar-Component-Guide.md)
- [Waybar Custom CSS](./Waybar-Custom-Css-Component-Guide.md)
- [Walker Launcher](./Walker-Launcher-Component-Guide.md)
- [Wofi App Menu](./Wofi-Application-Menu-Component-Guide.md)
- [Quickshell Widgets](./Quickshell-Widgets-Component-Guide.md)
- [SwayNC Notifications](./Swaync-Notification-Component-Guide.md)
- [Waypaper Wallpaper](./Waypaper-Wallpaper-Component-Guide.md)
- [Hyprpaper Daemon](./Hyprpaper-Wallpaper-Daemon-Component-Guide.md)
- [Hyprlock Shield](./Hyprlock-Screen-Shield-Component-Guide.md)
- [Hypridle Governor](./Hypridle-Timeout-Component-Guide.md)
- [Wlogout Session](./Wlogout-Session-Component-Guide.md)
- [Clipboard (Cliphist)](./Clipboard-Manager-Component-Guide.md)
- [Night Light & Gamma](./Night-Light-Gamma-Component-Guide.md)
- [Brightness & Backlight](./Brightness-Backlight-Component-Guide.md)
- [Theme Omakase](./Theme-Omakase-Component-Guide.md)

---

### [Multimedia & Audio](./Home.md#7-multimedia--audio-subsystem)
- [Audio Subsystem](./Audio-Subsystem-Component-Guide.md)
- [Wireplumber Audio Policy](./Wireplumber-Audio-Policy-Component-Guide.md)
- [Hardware Media Codecs](./Media-Codec-Hardware-Component-Guide.md)
- [Grimblast Screenshot](./Grimblast-Screenshot-Component-Guide.md)
- [WF-Recorder Screencast](./Wf-Recorder-Screencast-Component-Guide.md)
- [Graphics GPU Driver](./Graphics-Gpu-Component-Guide.md)

---

### [Terminal & Shell](./Home.md#8-terminal-shell--command-line)
- [Alacritty & Kitty](./Alacritty-Kitty-Terminal-Component-Guide.md)
- [Starship Prompt](./Starship-Prompt-Component-Guide.md)
- [Shells (Bash, Zsh, Fish)](./Bash-Zsh-Fish-Shell-Component-Guide.md)
- [Coreutils Parity](./Coreutils-Gnu-Bsd-Parity-Component-Guide.md)
- [Serial TTY & PTY](./Serial-Tty-Pty-Virtual-Console-Component-Guide.md)
- [Internationalization (i18n)](./Internationalization-I18n-Component-Guide.md)
- [Input Accessibility](./Input-Accessibility-Component-Guide.md)
- [Screen Reader](./Accessibility-Screen-Reader-Component-Guide.md)

---

### [Gaming & Virtualization](./Home.md#9-gaming-acceleration--virtualization)
- [GameMode Daemon](./Gamemode-Daemon-Component-Guide.md)
- [MangoHud Overlay](./Mangohud-Gaming-Component-Guide.md)
- [Lutris & Heroic Gaming](./Lutris-Heroic-Gaming-Component-Guide.md)
- [VKD3D-Proton Vulkan](./Vkd3d-Proton-Vulkan-Component-Guide.md)
- [DXVK Direct3D 11](./Dxvk-D3d11-Translation-Component-Guide.md)
- [Virtualization](./Virtualization-Component-Guide.md)
- [Containers (OCI/Podman)](./Container-Runtime-Component-Guide.md)
- [Rootless Podman](./Container-Podman-Rootless-Component-Guide.md)
- [PCI Passthrough (VFIO)](./Pci-Passthrough-Vfio-Component-Guide.md)

---

### [Packages & Maintenance](./Home.md#10-package-management--maintenance)
- [Package Management](./Package-Management-Component-Guide.md)
- [Package Build Ports](./Package-Build-Ports-Component-Guide.md)
- [Delta & Reproducible Builds](./Package-Delta-Reproducible-Build-Component-Guide.md)
- [Live ISO Calamares](./Live-Iso-Installer-Calamares-Component-Guide.md)
- [Bootloader](./Bootloader-Component-Guide.md)
- [Power Management](./Power-Management-Component-Guide.md)
- [Printing (CUPS)](./Printing-Cups-Component-Guide.md)
- [System Automation](./System-Automation-Component-Guide.md)
- [AI Maintenance Policy](./AI-Agent-Maintenance-Instructions.md)
- [Open Source OS Inspiration V58 PR Suite](./36-Open-Source-OS-Inspiration-V58-PR-Suite.md)
- [Open Source OS Inspiration V59 PR Suite](./37-Open-Source-OS-Inspiration-V59-PR-Suite.md)
