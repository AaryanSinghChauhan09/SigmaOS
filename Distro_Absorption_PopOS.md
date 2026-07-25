# Distro Absorption: Pop!_OS — Creator-Focused Workstation OS

> **Status**: 📋 Planned | **Source Paradigm**: System76 Pop!_OS | **Target Shard**: `SigmaOS Zenith Desktop & Power Management`

---

## 1. Executive Summary

Pop!_OS by System76 is an Ubuntu-based Linux distribution highly optimized for creators, developers, and gamers. It is known for its excellent out-of-the-box NVIDIA graphics support, the COSMIC desktop environment (which features hybrid tiling/floating window management), and intelligent power profiles.

SigmaOS absorbs Pop!_OS's **auto-tiling window manager**, **seamless hybrid graphics switching**, and **power profile daemon** into the Zenith Desktop experience.

---

## 2. Key Features to Absorb

### 2.1 Auto-Tiling Window Management (COSMIC-inspired)

Zenith Desktop includes a toggleable "Auto-Tiling" mode that automatically organizes windows into a non-overlapping grid, maximizing screen real estate while retaining the ability to float specific windows (like modal dialogs or calculators).

```bash
$ sigma desktop tiling enable
Σ [DESKTOP] Auto-tiling enabled.
  Shortcut: Super+Y to toggle.
  Exceptions: floating-only classes applied.
```

### 2.2 System76-Power Inspired Power Management

`sigma-power` manages CPU frequency scaling, PCI device states, and active cooling. It seamlessly integrates hybrid graphics, allowing users to launch specific applications on the discrete GPU while the rest of the OS runs on integrated graphics.

```bash
$ sigma power profile performance
Σ [POWER] Profile set to PERFORMANCE:
  CPU governor:   performance
  Turbo Boost:    ENABLED
  dGPU:           AWAKE

$ sigma-gpu-launch /usr/bin/blender
Σ [POWER] Launching Blender on discrete GPU (NVIDIA RTX).
```

### 2.3 Built-in Recovery Partition

Pop!_OS includes a recovery partition that allows the OS to be refreshed or repaired without external USB media. SigmaOS `sigma-recovery` creates an A/B boot partition layout for similar bulletproof updates and in-place repairs.

---

## 3. References & Standards

- Pop!_OS — `pop.system76.com` (GPL-3.0)
- COSMIC Desktop — `github.com/pop-os/cosmic-epoch`
