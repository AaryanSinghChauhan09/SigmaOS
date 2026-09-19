# SigmaOS Distro Subsystem Interoperability

## Overview

The `SovereignUniversalDistroBridge` provides SigmaOS with runtime compatibility and
interoperability across 21 Linux/BSD distribution subsystem modes spanning 32 core kernel subsystems.

## Supported Distribution Modes

### Linux Distributions
| Mode | Package Manager | Init System | Key Features |
|---|---|---|---|
| `LinuxArch` | pacman/AUR | systemd | rolling release, AUR, PKGBUILD |
| `LinuxDebian` | apt/dpkg | systemd | multi-arch, dpkg triggers, pbuilder |
| `LinuxAlpine` | apk | OpenRC | musl libc, minimal, world file |
| `LinuxNix` | nix | systemd | functional, immutable, flakes |
| `LinuxGentoo` | portage | OpenRC | USE flags, EAPI 8, ebuild slots |
| `LinuxFedora` | dnf5 | systemd | SELinux MLS, rpm-ostree, COPR |
| `LinuxVoid` | xbps | runit | musl or glibc, xbps-src |
| `LinuxOpenSuse` | zypper | systemd | YaST2, Snapper, btrfs |
| `LinuxSolus` | eopkg | systemd | Budgie, eopkg delta packages |
| `LinuxClear` | swupd | systemd | stateless (/usr defaults), ISA-optimized |
| `LinuxSlackware` | pkgtool | SysVinit | SlackBuilds, oldest actively maintained |
| `LinuxPopOs` | apt/flatpak | systemd | COSMIC desktop, System76 |
| `LinuxTails` | apt | systemd | amnesic memory scrubbing, Tor |
| `LinuxGuix` | guix | shepherd | functional, transactional, Scheme |

### BSD Systems
| Mode | Package Manager | Key Features |
|---|---|---|
| `FreeBsd` | pkg/ports | jails, VNET, ZFS, GEOM |
| `OpenBsd` | pkg | pledge/unveil, pf, W^X |
| `NetBsd` | pkgsrc | rump kernel, portability |
| `DragonFlyBsd` | dports | HAMMER2, vkernel |
| `SolarisIllumos` | IPS/pkgsend | Zones, DTrace, ZFS |
| `SmartOs` | pkgsrc | hyperzone, KVM, ZFS |
| `BedrockLinux` | any | strata, cross-distro |

## 32 Core System Subsystems

The bridge implements interoperability across all 32 kernel subsystems:

1. **Package Management** - Universal package format translation
2. **Service Supervision** - systemd/OpenRC/runit/shepherd/dinit/s6
3. **Security Model** - SELinux/AppArmor/pledge/unveil/Landlock
4. **Filesystem** - ext4/btrfs/zfs/hammer2/ufs/xfs/nilfs
5. **Network Stack** - NetBSD pf/OpenBSD pf/Linux nftables/IPFW
6. **Memory Management** - huge pages, ZRAM, swap, overcommit policies
7. **Process Scheduling** - CFS/BFS/EEVDF/BSD ULE/BORE
8. **Initramfs** - dracut/mkinitcpio/genkernel/initrd
9. **Boot Loader** - GRUB/systemd-boot/rEFInd/OpenBSD bootloader
10. **User Management** - useradd/adduser/pw/user(8)
11. **Logging** - journald/syslog-ng/metalog/alogd
12. **Container Runtime** - Docker/Podman/jails/zones/vmd
13. **Display Server** - Wayland/X11/direct framebuffer
14. **Audio** - PipeWire/ALSA/OSS/sndiod
15. **Bluetooth** - BlueZ/BSD bluetooth
16. **Wireless** - wpa_supplicant/iwd/ifconfig
17. **Power Management** - acpid/tlp/powerd/apm
18. **Hardware Detection** - udev/devd/hotplugd
19. **Time Synchronization** - chrony/ntpd/openntpd
20. **Firewall** - nftables/iptables/pf/ipfw
21. **DNS Resolver** - systemd-resolved/unbound/dnsmasq
22. **NTP** - chrony/ntpd/openntpd
23. **Cryptography** - OpenSSL/LibreSSL/BoringSSL/NSS
24. **Kernel Modules** - modprobe/kldload/modfind
25. **Archive Tools** - tar/pax/cpio/zip
26. **Locale** - glibc locale/musl locale/iconv
27. **Terminal Emulator** - xterm/alacritty/kitty/st
28. **Shell** - bash/zsh/dash/ksh/tcsh
29. **Compiler Toolchain** - GCC/Clang/TinyCC/LLVM
30. **Debug Tools** - gdb/lldb/dtrace/ktrace/truss
31. **Benchmark** - Phoronix/UnixBench/stress-ng
32. **Virtualization** - KVM/bhyve/xhyve/QEMU/VMware

## Key Innovations Implemented

### Clear Linux Stateless Architecture (`ClearLinuxStatelessEngine`)
Separates vendor defaults (`/usr/share/defaults/`) from user overrides (`/etc/`).
Allows factory reset without losing user config.

### Tails Amnesic Memory Scrubbing (`TailsAmnesicScrubEngine`)
Implements deterministic memory zeroing on session end to prevent forensic recovery.

### Alpine APK World File (`AlpineApkWorldFileEngine`)
Declarative package specification: a single file listing desired packages;
system auto-syncs to match this specification exactly.

### Void Linux Ed25519 Package Signatures (`VoidXbpsEd25519SigVerifier`)
Verifies XBPS package signatures using Ed25519 elliptic curve signatures.

### OpenBSD Unveil Audit Sentinel (`OpenBsdUnveilAuditSentinel`)
Tracks pledge/unveil policy violations for kernel-enforced filesystem sandboxing.

### DragonFly HAMMER2 Emergency CoW (`DragonFlyHammer2EmergencyCowEngine`)
Copy-on-Write filesystem primitives for ACID-compliant block transactions.

## See Also
- [Multi-Architecture HAL](Multi-Architecture-HAL.md)
- [Package Management](package-manager.md)
- [Security Model](security.md)
