# SigmaOS Linux & BSD Distro Compatibility & Ecosystem Guide

SigmaOS implements a unified, zero-overhead compatibility story for software originating from major Linux and BSD distribution ecosystems.

## 1. Native Syscall & ABI Translation (`UniversalSyscallAbiShim`)

SigmaOS directly translates foreign System V x86_64 ABI syscalls into native microkernel capabilities without requiring virtual machine emulation:

* **Linux x86_64 ABI:** Full dispatch support for standard Linux syscalls including `read(0)`, `write(1)`, `open(2)`, `close(3)`, `mmap(9)`, `brk(12)`, and `exit(60)`.
* **FreeBSD ABI:** Direct translation for FreeBSD kernel syscalls (`read`, `write`, `open`, `exit`).
* **OpenBSD & NetBSD ABI:** Built-in translation for OpenBSD `pledge(2)` / `unveil(2)` access control policies and NetBSD `rump(3)` hypercalls.

## 2. Multi-Format Package Bridge (`MultiFormatPackageBridge`)

Native transpilation and runtime conversion of foreign package formats into native `.sigpkg` bundles:

| Distro Origin | Package Format | Conversion Mechanism |
|---|---|---|
| **Debian / Ubuntu** | `.deb` | `control.tar.xz` metadata & dependency mapper |
| **Arch Linux** | `PKGBUILD` / `.pkg.tar.zst` | Plaintext recipe sandbox parser & AUR adapter |
| **Fedora / RHEL** | `.rpm` | DNF shared lock & RPM tag parser |
| **FreeBSD** | `+MANIFEST` / `.pkg` | FreeBSD pkg manifest reader & Capsicum rights generator |
| **Alpine Linux** | `.apk` | Musl libc safety auditor & APK world file sync |

## 3. POSIX Shared Memory & Event Multiplexing (`PosixSharedMemoryIpcBridge`)

* **Shared Memory:** POSIX `/dev/shm` shared memory allocation with zero-copy memory mapping.
* **Event Loops:** Multiplexing adapter bridging FreeBSD `kqueue` / `kevent` filters and Linux `epoll` instances into native microkernel event queues.

## 4. Media & Sound Engine (`LinuxBsdDistroMediaSuite`)

* **Ubuntu / PipeWire:** GStreamer multimedia pipelines with automatic PipeWire / PulseAudio sink routing.
* **FreeBSD / OpenBSD:** mpv zero-copy media player with native BSD `sndio` audio daemon integration.
* **Arch / Fedora:** FFmpeg zero-copy hardware encoding pipeline supporting Intel VAAPI, NVIDIA NVENC, and AMD AMF.
* **Linux Mint:** VLC SSA/ASS rich subtitle rendering and multi-track audio selector.

## 5. Universal Subsystem Cross-Distro Matrix (`SovereignUniversalDistroBridge`)

`SovereignUniversalDistroBridge` orchestrates seamless interoperability across 32 core SigmaOS subsystems and 21 Linux & BSD distro subsystem modes (`LinuxArch`, `LinuxDebian`, `LinuxAlpine`, `LinuxNix`, `LinuxGentoo`, `LinuxFedora`, `LinuxVoid`, `LinuxOpenSuse`, `LinuxSolus`, `LinuxClear`, `LinuxSlackware`, `FreeBsd`, `OpenBsd`, `NetBsd`, `DragonFlyBsd`, `SolarisIllumos`, `SmartOs`, `BedrockLinux`, `LinuxPopOs`, `LinuxTails`, `LinuxGuix`):

1. **`init`**: Service supervision dispatching (`systemd`, `OpenRC`, `runit`, `Shepherd`, `Dinit`, `SysVInit`, `SMF`, `RCD`).
2. **`package`**: Multi-format specifier translation (`.deb`, `.rpm`, `.apk`, `PKGBUILD`, `.ebuild`, `.nix`, `.scm`, `.xbps`, `.eopkg`, `.bundle`, `.pkg`, `.tgz`, `.p5p`, `.stratum`, `.hpkg`).
3. **`vfs`**: Transparent filesystem path translation (`/etc`, `/var/lib/pkg`, `/proc`, `/sys`, `/dev`, `/tmp`).
4. **`security`**: Security isolation enforcement (`AppArmor`, `SELinux`, `Capsicum`, `Landlock`, `pledge`/`unveil`, `RetGuard`).
5. **`storage`**: Storage filesystem CoW self-healing and snapshots (`Btrfs`, `HAMMER2`, `ZFS`, `GEOM`, `Soft Updates`).
6. **`kernel`**: Task scheduler registration (`EEVDF`, `BORE`, `MLFQ`, `SCHED_EXT`, `NUMA`, `eBPF`).
7. **`network`**: Network stack dispatch (`XDP` zero-copy sockmap, `VNET`, `Crossbow` VNIC/Etherstub, `PF` stateful filtering, `Netplan`).
8. **`graphics`**: Atomic DRM/KMS modesetting and Gamescope Wayland layer-shell composition.
9. **`power`**: System76 power profile governor and DVFS frequency capping.
10. **`ipc`**: Zero-copy splice channels and POSIX `/dev/shm` IPC bridges.
11. **`auth`**: `systemd-homed` PAM and `doas`/`sudo` authorization gates.
12. **`audit`**: Dynamic `DTrace` probe auditing and eBPF event tracking.
13. **`boot`**: Multi-architecture boot chain configuration (`Multiboot2`, `UEFI`, `SigmaBoot`, `Grub`, `Limine`).
14. **`container`**: Cross-distro isolated container management (`Toolbx`, `FreeBSD Jails`, `Solaris Zones`, `Qubes`).
15. **`virtualization`**: Hypervisor guest launch (`SovereignVMM`/`KVM`, `bhyve`, OpenBSD `vmm`, Illumos `Brand`).
16. **`audio`**: Zero-latency PipeWire, PulseAudio, `sndio`, and ALSA audio routing.
17. **`input`**: USB HID and Evdev multi-touch input event mapping.
18. **`thermal`**: Thermal governor trip-point monitoring and fan control.
19. **`memory`**: KARL/W^X randomized memory page allocation, CMA, and zram compression.
20. **`syscall`**: Multi-arch syscall translation and pledge sandbox dispatch.
21. **`device`**: Dynamic devfs and udev auto-probe hardware binding.
22. **`crypto`**: Post-quantum cryptographic signing (`Dilithium-5`, `Falcon`) and CSPRNG entropy management.
23. **`ai`**: Local agentic LLM KV-cache inference execution.
24. **`monitoring`**: Structured `journald` binary storage and system health telemetry queries.
25. **`desktop`**: Desktop compositor environment adaptation (`GNOME`/Adwaita, `KDE`/Breeze, `COSMIC`, `Budgie`, `XFce`/Lumina, `Hyprland`).
26. **`compiler`**: Compiler chroot sandboxing (`Makepkg`, `Sbuild`, `Poudriere`, `Portage` EAPI8, `XBPS-src`).
27. **`i18n`**: Input method candidate selection and multi-locale keyboard mappings.
28. **`bluetooth`**: Wireless bluetooth audio stream routing via BSD Netgraph or BlueZ DBus.
29. **`firewall`**: Stateful packet filtering via OpenBSD `PF`, Solaris `IPFilter`, or Linux `nftables`/`firewalld`.
30. **`diagnostics`**: Automated crash diagnostics via `ABRT`, `DTrace` probe audit, and eBPF forensics.
31. **`recovery`**: Atomic system recovery snapshot rollback (`Snapper`, `ZFS`/`HAMMER2` Boot Environments, `TimeTravel`).
32. **`time`**: High-precision time synchronization (`CLOCK_MONOTONIC_RAW`/Chrony vs `CLOCK_MONOTONIC_PRECISE`/ntpd).
