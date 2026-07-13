# Distro Absorption: Qubes OS

> **Status**: 📋 Planned | **Source Paradigm**: Qubes OS | **Target Shard**: `SigmaOS Sovereign Compartmentalization`

---

## 1. Executive Summary

Qubes OS is a security-oriented operating system that implements "Security by Compartmentalization" using the Xen hypervisor. Instead of relying on a monolithic kernel to isolate processes, Qubes runs different user tasks in completely isolated Virtual Machines (qubes), with strict, policy-enforced communication between them.

SigmaOS absorbs the **compartmentalization GUI model** and **secure peripheral proxying**, utilizing `sigma-vm` (Firecracker/KVM) rather than Xen, yielding faster boot times while maintaining hardware-level isolation.

---

## 2. Key Features to Absorb

### 2.1 Hardware-Isolated Compartments (Shards)

In SigmaOS, users can define distinct "Domains" (e.g., Work, Personal, Banking, Untrusted). Each Domain runs in a separate Firecracker microVM.

```bash
$ sigma domain create banking --template secure-minimal --net restricted
Σ [DOMAIN] Created domain "banking"
  Isolation: Hardware VT-x (KVM)
  Network  : Whitelisted (only banking URLs allowed)
  Storage  : Encrypted volume (AES-256-XTS)
```

In the GUI (Zenith Compositor), windows from different domains have colored borders to immediately indicate their security context. A compromised browser in the "Untrusted" domain cannot access the SSH keys in the "Work" domain because they run on entirely different virtual kernels.

### 2.2 Secure Peripheral Routing

In a monolithic OS, a malicious USB device can compromise the entire kernel. SigmaOS isolates hardware buses into `sys-usb` and `sys-net` domains.

```
┌────────────────────────────────────────────────────────┐
│               HARDWARE ISOLATION LAYER                 │
│                                                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ sys-net      │  │ sys-usb      │  │ app-work     │  │
│  │ (IOMMU pass- │  │ (IOMMU pass- │  │ (No direct   │  │
│  │  through of  │  │  through of  │  │  hardware)   │  │
│  │  WiFi card)  │  │  USB hubs)   │  │              │  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  │
│         │                 │                 │          │
│         ▼                 ▼                 ▼          │
│  ┌──────────────────────────────────────────────────┐  │
│  │            SIGMA MICROKERNEL (KVM)               │  │
│  └──────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────┘
```

If you plug in a USB drive, it appears in `sys-usb`. You must explicitly route it to `app-work` via the secure IPC bus.

```bash
$ sigma device attach sys-usb:Kingston_DataTraveler app-work
Σ [IPC] Attached block device to app-work.
```

### 2.3 Split GPG / Split SSH

Private keys never leave an offline, network-disconnected domain (`vault`). When an application in the `work` domain needs to sign a commit or authenticate via SSH, it sends a request over IPC to the `vault`. The user is prompted to approve the operation, and only the signature is returned.

---

## 3. References & Standards

- Qubes OS Architecture — `qubes-os.org` (GPL-2.0)
- Xen Hypervisor (source inspiration)
- Split GPG architecture
