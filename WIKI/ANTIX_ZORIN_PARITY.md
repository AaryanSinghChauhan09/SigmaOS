# antiX Linux & Zorin OS Parity Features in SigmaOS

## Executive Overview

SigmaOS incorporates architectural paradigms inspired by **antiX Linux** (systemd-free, ultra-lightweight init and service management for low-resource hardware) and **Zorin OS** (adaptive desktop layout switching between Windows 11, Windows Classic, macOS, GNOME, and Ubuntu interfaces).

---

## 1. antiX Linux Lightweight Init (`AntiXSysVInitEngine`)
- **Systemd-Free Init Manager**: `src/distro/antix_zorin_innovations.rs`
- **Subsystem Mode**: `DistroSubsystemMode::LinuxAntiX`
- **Capabilities**:
  - Lightweight SysVinit & Runit service lifecycle controls (`Stopped`, `Running`, `Disabled`).
  - Zero-overhead init execution for low-RAM and legacy hardware profiles.

---

## 2. Zorin OS Appearance Switcher (`ZorinAppearanceSwitcher`)
- **Desktop Layout Switcher**: `src/distro/antix_zorin_innovations.rs`
- **Subsystem Mode**: `DistroSubsystemMode::LinuxZorin`
- **Layout Profiles**:
  - `Windows11`: Modern centered taskbar & start menu overlay.
  - `WindowsClassic`: Traditional left-aligned taskbar.
  - `MacOs`: Top menu bar + bottom dock container.
  - `GnomeStandard`: Top panel + full-screen application grid.
  - `UbuntuUnity`: Left vertical launcher panel.
