# SigmaOS Distro Absorption: NixOS (Declarative System Engine)

## 1. Overview
SigmaOS incorporates NixOS's core paradigm: total declarative system configuration, reproducible state graphs, zero-drift system profiles, and atomic generation rollbacks.

## 2. Architecture & Components
- **Declarative Manifest (`/etc/sigmaos/config.sig`)**: Single declarative specification for all kernel parameters, system services, user accounts, and installed applications.
- **Generation Store (`/sigma/store`)**: Immutable content-addressed storage for all system binaries, libraries, and configuration files.
- **Atomic Switch (`sigctl switch`)**: Atomic symlink swapping between system generations without requiring reboots.
- **Rollback Engine (`sigctl rollback`)**: Instant hardware-enforced rollback to any previous system state.

## 3. Configuration Specification
```hcl
system "sigma-sovereign" {
  kernel {
    modules = ["kvm", "nvme", "wireguard"]
    params  = ["intel_iommu=on", "hugepages=1024"]
  }
  
  services {
    sshd    = { enable = true, port = 22 }
    matrix  = { enable = true, domain = "sigma.internal" }
    siginit = { autostart = true, parallel = true }
  }

  packages = [
    "sigpkg:gcc",
    "sigpkg:neovim",
    "sigpkg:htop",
    "sigpkg:wireshark"
  ]
}
```
