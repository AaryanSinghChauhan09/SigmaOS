# Distro Absorption: Manjaro — Accessible Arch Linux

> **Status**: 📋 Planned | **Source Paradigm**: Manjaro Linux | **Target Shard**: `SigmaOS Zenith Hardware Detection & Rolling Release`

---

## 1. Executive Summary

Manjaro is an Arch Linux-based distribution that aims to make Arch's rolling-release model accessible to everyday users. Its defining features are **MHWD (Manjaro Hardware Detection)** for automatic proprietary driver installation, and **staged rolling releases** (delaying Arch updates to test for stability).

SigmaOS absorbs Manjaro's **automatic hardware configuration tool** and **staged rolling release channels** to provide bleeding-edge software without the typical breakage of a pure rolling release.

---

## 2. Key Features to Absorb

### 2.1 Sigma Hardware Detection (`sigma-hwd`)

Modeled after Manjaro's MHWD, `sigma-hwd` automatically detects system hardware (especially graphics cards and network adapters) and can install the optimal drivers—including proprietary ones like NVIDIA—with a single command.

```bash
$ sigma-hwd auto --video
Σ [HWD] Detected NVIDIA RTX 4070.
  Installing: sigma-driver-nvidia-proprietary (550.x)
  Configuring Zenith Desktop for hybrid graphics...
  Done. Reboot required.
```

### 2.2 Staged Rolling Release

While SigmaOS's `sigma.next` channel is a pure rolling release, the `sigma.beta` channel acts like Manjaro's stable branch: updates from `sigma.next` are held in testing for a week. If significant breakage is reported, the packages are held back until fixed, insulating normal users from upstream regressions.

---

## 3. References & Standards

- Manjaro Linux — `manjaro.org`
