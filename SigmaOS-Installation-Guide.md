# SigmaOS Bare-Metal Installation Guide

Welcome to the **SovereignInstaller** guide. The SigmaOS installer is a bare-metal native C++ tool built directly into the kernel source tree, designed to ensure perfect reliability, security, and integration with our atomic rollback systems.

## 🚀 Installation Modes

You can run the installer using the `sigma_install_cli` wizard from the live environment.

### 1. Guided Installation (Recommended)

The guided installation uses our enterprise-grade defaults. It is highly recommended for 95% of users.

```bash
sigma_install_cli --guided /dev/nvme0n1
```

**What this does:**
1. **Wipes the target disk** safely.
2. **Creates A/B Partitions:** Sets up `/sigma_root_A`, `/sigma_root_B`, and a persistent `/sigma_var` partition for NixOS/Silverblue-style atomic rollbacks.
3. **LUKS Encryption:** Automatically encrypts the root volume.
4. **Btrfs:** Uses Btrfs for the filesystem to support native COW (Copy-On-Write) snapshots.
5. **Secure Boot:** Enrolls TPM 2.0 keys for measured boot.

### 2. Advanced Installation

For power users (HPC deployments, specialized server rigs), you can define exact parameters.

```bash
sigma_install_cli --advanced /dev/sda
```

The advanced wizard will prompt you for:
- Disabling/Enabling LUKS and LVM.
- File system choice (ext4, Btrfs, ZFS).
- Bypassing A/B partitions (Not recommended unless using a very space-constrained IoT device).

## 🛡️ The Architecture Advantage

Unlike Calamares or Anaconda (which rely heavily on Python and massive dependency trees), the `SovereignInstaller` is a statically linked C++ binary. 

This guarantees:
- **Zero crashes** due to missing Python libraries on the live USB.
- **Flawless integration** with the `SovereignAtomicEngine` for identical live and installed environments.
- **Minimal RAM usage**, allowing installation on deeply embedded edge devices.

## Post-Installation

Upon rebooting, your system will launch into the **Zenith Desktop Environment**. 
If you are migrating from Ubuntu or Windows, you can now launch the **SigmaOS Migration Assistant** from the Control Center to automatically port your files and configs.
