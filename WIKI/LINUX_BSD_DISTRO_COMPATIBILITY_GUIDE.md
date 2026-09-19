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
