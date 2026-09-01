# ❓ Frequently Asked Questions (FAQ)

### What is SigmaOS?
SigmaOS is an AI-native, microkernel-based operating system written in safe Rust. It incorporates technical innovations from premier Linux distributions (Fedora, Arch, Debian, openSUSE, NixOS, Linux Mint) and BSD systems (FreeBSD, OpenBSD, DragonFly BSD).

### Is SigmaOS POSIX compliant?
Yes. SigmaOS implements a POSIX system call matrix (`src/memory/low_level.rs`) and ELF loader (`src/loader/elf/elf_loader.rs`) supporting Glibc and Musl C library profiles.

### Which package formats are supported?
Through the `sigpkg` universal adapter (`src/sigpkg/universal_adapter.rs`), SigmaOS can install, convert, and sandbox `.sigpkg`, `.deb`, `.rpm`, `PKGBUILD`, `.apk`, `.xbps`, `Flatpak`, and `Snap` packages.

### How do I run the automated test suite?
Run `./run_sigma_tests.sh` to execute the 40 atomic hardware/container tests and the 212 core algorithm unit tests.

### How do I run the Quality Gate checks?
Run `./scripts/sigma_quality_check.sh` to verify zero open TODO stubs, zero credential hits, and 1:1 wiki synchronization.
