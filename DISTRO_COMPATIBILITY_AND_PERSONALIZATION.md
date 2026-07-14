# Distro Compatibility & Personalization Engines

This document outlines the design and integration of the Distro Streamer and declarative Personalizer engines inside SigmaOS userland.

## 1. Distro Stream Compatibility (`sigma_distro_streamer.rs`)

To absorb capabilities from other Linux distributions (such as Ubuntu, Debian, Fedora, and Arch) without running full virtual machine emulators, SigmaOS implements userland syscall redirection and filesystem shimming.

- **Syscall Mapping Matrix**: The distro streamer maintains an translation lookup table mapping key Linux syscall numbers (e.g. `sys_fork`, `sys_execve`, `sys_clone`, `sys_open`) to their corresponding SigmaOS capability calls.

- **Rootfs Containerization**: Unpacks and mounts root filesystem streams (squashfs or tarballs) of target distros under structured namespaces.

## 2. Declarative Personalizer (`sigma_personalizer.rs`)

Inspired by NixOS and Home Manager, SigmaOS supports declarative system state configurations.

- **Declarative Package Management**: Evaluates lists of desired packages and synchronizes them using the local package manager daemon.

- **Declarative User Env**: Manages theme configuration, font sizes, and layout templates dynamically through centralized configurations.

### Last Updated: July 2026
