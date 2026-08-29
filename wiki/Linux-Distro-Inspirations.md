# Linux and BSD Distro Inspirations

SigmaOS is built on the shoulders of giants. We have carefully studied the Unix philosophy and the evolution of both Linux and BSD distributions to synthesize the best ideas into a modern, Rust-based operating system.

## Linux Distros
- **Arch Linux**: The rolling release model ensures software is always up-to-date. `sigpkg` borrows concepts from pacman and the AUR (Arch User Repository) for community-driven packaging.
- **Ubuntu/Debian**: The robust dependency resolution of APT and the `dpkg` trigger system inspired how we handle post-install hooks.
- **Fedora/RHEL**: Integration of advanced security policies (like SELinux) and atomic, image-based updates inspired by `rpm-ostree`.
- **Gentoo**: The Portage system's USE flags are integrated into `sigpkg`, allowing users to compile source packages with custom feature flags.
- **NixOS**: Concept of declarative, reproducible environments and atomic rollbacks through the `nix-store`.
- **openSUSE**: YaST-style centralized configuration management and native support for filesystem snapshots.
- **Manjaro**: User-friendly wrappers for complex systems (like AUR helpers).
- **Alpine Linux**: Extreme minimalism, use of lightweight libraries, and fast `apk` package management, ideal for containerization.
- **Clear Linux**: The concept of a stateless OS, separating user configuration from system defaults.
- **Void Linux**: The lightweight `runit` init system and the speedy, no-nonsense `xbps` package manager.

## BSD Derivatives
- **FreeBSD**: The Jails subsystem inspired our container isolation primitives. The ports system influenced our source-based package tree.
- **OpenBSD**: Uncompromising focus on security. The `pledge` and `unveil` system calls inspired the Sentinel security subsystem.
- **NetBSD**: Focus on extreme portability across different hardware architectures (`pkgsrc`).
