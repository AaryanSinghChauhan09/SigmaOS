# Distro Absorption: Chrome OS / ChromiumOS

> **Status**: 📋 Planned | **Source Paradigm**: Chrome OS / ChromiumOS | **Target Shard**: `SigmaOS Verified Boot & Web Runtime`

---

## 1. Executive Summary

Chrome OS is Google's Linux-based operating system designed around the web browser. Its standout engineering features are **Verified Boot** (a cryptographic chain-of-trust from firmware to OS), **stateless root filesystem** (rootfs is read-only; user data lives in a separate encrypted partition), and **container-based Linux environment** (Crostini).

SigmaOS absorbs Chrome OS's **Verified Boot chain**, **stateless root design**, and **containerized application runtime** to provide an operating system that is intrinsically tamper-evident at every boot.

---

## 2. Key Features to Absorb

### 2.1 Verified Boot Chain

Inspired by Chrome OS's Verified Boot spec, SigmaOS implements a hardware root-of-trust where each stage verifies the next before transferring control.

```mermaid
graph LR
    FW[Firmware RO] -->|Verify hash| RW_FW[Firmware RW]
    RW_FW -->|Verify Dilithium5| Kernel[sigma-kernel.signed]
    Kernel -->|dm-verity| RootFS[/ read-only]
    RootFS -->|Decrypt + Mount| UserData[/home encrypted]
```

If any layer fails verification, the system refuses to boot and drops into recovery mode.

### 2.2 Stateless Read-Only Root Filesystem

The SigmaOS rootfs (`/sigma/base`) is mounted read-only and verified by dm-verity. All system state (package installations, config) lives in a separate read-write overlay mounted at `/etc/sigma`.

```bash
$ sigma rootfs verify
Σ [ROOTFS] dm-verity check:
  Hash tree:  valid
  Root hash:  blake3:a1b2c3d4...
  Signature:  Dilithium5 valid (pkg.sigma.dev key)
  Status:     TRUSTED ✓
```

### 2.3 Isolated Application Runtime (sigma-crostini)

Like Chrome OS's Crostini, SigmaOS ships a lightweight LXC-based Linux environment where users can run standard Linux applications without compromising the host OS integrity.

---

## 3. References & Standards

- Chrome OS — `chromium.org/chromium-os` (BSD/Apache-2.0)
- Verified Boot — `chromium.org/chromium-os/chromiumos-design-docs/verified-boot`
