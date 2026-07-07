# SigmaOS Installer & First-Boot UX Specification

## 1. Vision
The SigmaOS installer draws inspiration from Calamares but is built entirely on the native Zenith Wayland compositor using our lightweight UI toolkit. The goal is to provide a frictionless, 3-click installation path for enterprise and daily-driver use cases while strictly enforcing secure-by-default postures.

## 2. Core Installation Flow
1. **Welcome & Language:** Auto-detected via UEFI locale hints if available.
2. **Security Posture (The "Sigma" Difference):**
   - Prompt: "Enable Full Disk Encryption (LUKS2 + TPM2)?" -> Default: **YES**
   - Prompt: "Enforce Strict Secure Boot?" -> Default: **YES**
3. **Partitioning:**
   - Atomic Subvolumes: Root (`/`), Home (`/home`), and Staging (`/staging`).
   - Uses SigmaFS / BTRFS out of the box to support atomic `sigpkg` rollbacks.
4. **User Creation:** Standard user provisioning. The first user is added to the `wheel` equivalent group, but `sudo` requires hardware token/TPM attestation by default.

## 3. First-Boot Experience (OOBE)
- **Silent Boot:** A flicker-free boot experience leveraging the DRM/KMS layer directly into the Zenith Wayland compositor.
- **MicroVM Pre-warming:** Essential dev sandboxes and AI runtimes are pre-warmed during the first login to ensure instantaneous opening of critical apps.
- **Hardware Telemetry Check:** Opt-in prompt for anonymous driver crash reporting to support the Driver Bounty Program.

## 4. Enterprise Imaging
The installer supports headless YAML-based provisioning (Kickstart/Ignition equivalent).
```yaml
sigmaos:
  version: "1.0.0"
  security:
    tpm2_enroll: true
    fde: true
  packages:
    - sigma-dev-toolchain
```
