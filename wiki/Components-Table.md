# SigmaOS Components

| Component | Module Path | Status | Description | Linux Equivalent | BSD Equivalent | Implementation File |
|---|---|---|---|---|---|---|
| Microkernel | `src/kernel/` | 🚧 | Core kernel logic | `vmlinux` | `kernel` | `src/kernel/main.rs` |
| HAL | `src/hal/` | 🚧 | Hardware abstraction layer | `arch/` | `sys/arch/` | `src/hal/mod.rs` |
| Memory Management | `src/memory/` | 🚧 | Paging, allocation, VMM | `mm/` | `sys/vm/` | `src/memory/vmm.rs` |
| Security Subsystem | `src/security/` | 🚧 | Sentinel capability-based security | SELinux/AppArmor | Capsicum/pledge | `src/security/sentinel.rs` |
| Virtual Filesystem | `src/filesystem/` | 🚧 | VFS layer and mount points | `fs/` | `sys/vfs/` | `src/filesystem/vfs.rs` |
| Networking Stack | `src/networking/` | 🚧 | TCP/IP stack implementation | `net/` | `sys/net/` | `src/networking/tcpip.rs` |
| Container Runtime | `src/containers/`, `src/virtualization/` | 🚧 | OCI containers, namespaces | LXC/Docker | Jails | `src/containers/runtime.rs` |
| Package Manager | `src/package_manager/` | 🚧 | sigpkg universal package manager | apt/pacman/portage | pkg | `src/package_manager/sigpkg.rs` |
| Desktop Environment | `src/desktop/` | 🚧 | Palette GUI/compositor | Wayland/GNOME | X11/Lumina | `src/desktop/palette.rs` |
| Audio System | `src/audio/` | 🚧 | Bolt low-latency audio | ALSA/PulseAudio | OSS | `src/audio/bolt.rs` |
| GPU/Graphics | `src/gpu/` | 🚧 | Graphics drivers (DRM/KMS) | `drivers/gpu/` | `sys/dev/drm/` | `src/gpu/kms.rs` |
| Shell/Terminal | `src/shell/` | 🚧 | Default system shell | bash/zsh | sh/tcsh | `src/shell/sigma_sh.rs` |
| Init System | `src/init/` | 🚧 | System initialization (PID 1) | systemd/runit | init | `src/init/init.rs` |
| Device Drivers | `src/drivers/` | 🚧 | Core peripheral drivers | `drivers/` | `sys/dev/` | `src/drivers/mod.rs` |
| Bootloader | `src/boot/` | 🚧 | Boot protocol implementation | GRUB/systemd-boot | loader | `src/boot/bootloader.rs` |
| Process Scheduler | `src/scheduler/` | 🚧 | CPU scheduling algorithms | CFS | ULE scheduler | `src/scheduler/cfs.rs` |
| IPC System | `src/ipc/` | 🚧 | Inter-process communication | D-Bus/Binder | kqueue/Sockets | `src/ipc/message.rs` |
| Logging System | `src/logging/` | 🚧 | Centralized logging | journald/syslog | syslogd | `src/logging/syslog.rs` |
| Remote Desktop | `src/remote/` | 🚧 | Screen sharing and remote control | VNC/RDP | VNC | `src/remote/vnc.rs` |
| AI Inference Engine | `src/ai/` | 🚧 | Local AI capabilities | Ollama/Llama.cpp | - | `src/ai/inference.rs` |
| Cryptography | `src/crypto/` | 🚧 | Crypto primitives | Kernel Crypto API | OpenCrypto | `src/crypto/mod.rs` |
| USB Stack | `src/usb/` | 🚧 | USB device support | `drivers/usb/` | `sys/dev/usb/` | `src/usb/host.rs` |
| ACPI | `src/acpi/` | 🚧 | Power management | `drivers/acpi/` | `sys/dev/acpica/` | `src/acpi/mod.rs` |
| PCI System | `src/pci/` | 🚧 | PCI bus enumeration | `drivers/pci/` | `sys/dev/pci/` | `src/pci/bus.rs` |
| Distro Compatibility | `src/distros/` | 🚧 | Linux/BSD interop layer | WSL/Linuxulator | Linuxulator | `src/distros/compat.rs` |
| Build System | `Makefile`, `build.rs` | 🚧 | Build instructions | Kbuild | bsd.prog.mk | `build.rs` |
| CI/CD | `.github/workflows/` | 🚧 | Automated testing and checks | - | - | `.github/workflows/ci.yml` |
