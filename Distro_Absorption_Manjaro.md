# Distro Absorption: Manjaro Linux

> **Status**: 📋 Planned | **Source Paradigm**: Manjaro | **Target Shard**: `SigmaOS Hardware Detection`

---

## 1. Executive Summary

Manjaro brings the bleeding-edge software of Arch Linux to a broader audience by introducing user-friendly hardware detection and multiple-kernel management. 

SigmaOS absorbs the **MHWD (Manjaro Hardware Detection)** philosophy, providing users with a GUI and CLI tool that automatically identifies PCI/USB IDs and seamlessly provisions the correct proprietary or open-source drivers.

---

## 2. Key Features to Absorb

### 2.1 Automated Hardware Provisioning (`sigma-hw`)

Instead of hunting for forum posts to install a Wi-Fi driver, SigmaOS scans the hardware at boot or on command, cross-references a verified hardware database, and safely installs the required kernel modules.

```bash
$ sigma hw auto-install
Σ [HW] Scanning PCI/USB buses...
  Detected: NVIDIA Corporation TU117M [GeForce GTX 1650 Mobile / Max-Q]
  Selected: sigma-driver-nvidia-proprietary (v535)
  Detected: Realtek Semiconductor RTL8822CE 802.11ac PCIe Wireless Network Adapter
  Selected: sigma-driver-rtw88

Σ [HW] Installing drivers and rebuilding initramfs...
```

### 2.2 Kernel Switching GUI

Manjaro allows users to easily swap between Linux 5.15 LTS, 6.1 LTS, and 6.6 via a simple GUI. SigmaOS takes this further. Because SigmaOS manages boot parameters declaratively, the UI allows users to install multiple kernel versions and explicitly assign them to different boot profiles in `sigma-boot`.

---

## 3. References & Standards

- Manjaro Linux — `manjaro.org`
- mhwd source (GPL)
