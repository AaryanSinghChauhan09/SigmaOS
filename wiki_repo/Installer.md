# Installer Roadmap & Specification

## 1. GUI Installer Design & Zenith UI
SigmaOS features a graphical installer built directly on top of the Zenith Wayland compositor. It utilizes the native Rust UI layout engine, maintaining a small footprint (less than 20MB installer payload).

### Core Installer Modules
- **Welcome & Locale**: Auto-detects keyboard layout and location.
- **Partitioning**: Configures Btrfs/SigmaFS subvolumes, creating isolated active and passive staging subvolumes.
- **User Provisioning**: System config for first login and administrative token keys.

## 2. Security Defaults
- **TPM2 Full Disk Encryption**: Encrypts home directories (`/home`) and system targets using LUKS2, binding recovery keys to TPM2 measurements.
- **Secure Boot Integration**: Enrolls custom owner keys (MOK) during installation to enforce signed boot execution.
- **Dual-Boot Coexistence**: Safely parses active EFI System Partitions (ESP) to configure boot configurations side-by-side with Windows or Linux.

## 3. Enterprise Automated Installation
For automated deployments, the installer accepts a headless provisioning file (`autoinstall.yaml`):
```yaml
version: 1
partitioning:
  disk: /dev/nvme0n1
  encryption: tpm2_luks2
  subvolumes:
    - name: root
    - name: staging
    - name: home
user:
  username: admin
  ssh_key: ssh-ed25519 AAAAC3Nza...
packages:
  - sigma-dev-sdk
  - zenith-desktop
```

## 4. Roadmap Phases
- **Phase 1 (0–3m)**: Terminal-based partitioning installer and Btrfs setup helpers.
- **Phase 2 (3–6m)**: Graphical Zenith-based setup wizard interface and autoinstall parser.
- **Phase 3 (6–9m)**: Dual-boot setup logic and UEFI boot menu manager.
- **Phase 4 (9–12m)**: Full-Disk Encryption integration (TPM2 + LUKS2) and automatic key enrollment.

## 5. Contributor Guidelines
- Keep UI components responsive, adhering to high contrast accessibility targets.
- Ensure automated installers are thoroughly tested in headless QEMU runs.
