# Distro Absorption: Ubuntu

## Overview

Ubuntu is the most widely deployed Linux distribution for desktops, cloud instances, and IoT devices. Its success is driven by excellent hardware detection, cloud-init integration, Snap packaging, and the Debian package ecosystem.

## Key Principles Absorbed

### Cloud-Init & First-Boot Provisioning

- Ubuntu's `cloud-init` system for automatic instance configuration is absorbed into SigmaOS's init subsystem.
- Machine identity, network configuration, and package installation are declaratively specified.
- SigmaOS replaces YAML-based cloud-init with native typed configuration structs.

### Hardware Detection & Driver Management

- Ubuntu's `ubuntu-drivers` tool for automatic GPU and peripheral detection is absorbed into the `sigma_drv` subsystem.
- Hardware fingerprinting and driver binding occur at boot without external tooling.

### Snap / AppArmor Integration

- Ubuntu's Snap confinement model (based on AppArmor profiles) is fully absorbed into `sigma_security`.
- `SigmaProfile` provides path-based read/write/exec restrictions natively.
- The `sigma_containers` `SandboxManager` replaces Snap's squashfs sandbox.

### APT / dpkg Package Management

- SigmaOS's `sigpkg` provides a superset of APT's dependency resolution.
- The `tools/sigma_apt_compat_mesh.rs` compatibility layer enables migration from `.deb` packages.

### Multipass & LXD

- Ubuntu's lightweight VM/container tools are absorbed via `sigma_containers`.
- `MicroVMEngine` replaces Multipass; `ContainerRuntime` replaces LXD.

## Displaced Technologies

| Ubuntu Component | SigmaOS Replacement |
| --- | --- |
| cloud-init | Native typed init config |
| ubuntu-drivers | `sigma_drv` hardware binding |
| Snap + AppArmor | `sigma_security::SigmaProfile` + `sigma_containers::SandboxManager` |
| APT / dpkg | `sigpkg` declarative resolver |
| Multipass | `sigma_containers::MicroVMEngine` |
| LXD | `sigma_containers::ContainerRuntime` |
| Netplan | `sigma_net` native network config |

## Status

**Core Absorbed** — AppArmor profile model, container sandboxing, and package management are implemented. Cloud-init typed config and Netplan absorption are in progress.
