# Distro Absorption: Solus

> **Status**: 📋 Planned | **Source Paradigm**: Solus Linux | **Target Shard**: `SigmaOS Curated Update Channels`

---

## 1. Executive Summary

Solus is an independent rolling-release Linux distribution that balances the latest software releases with stability through a curated rolling model and its custom desktop environment (Budgie).

SigmaOS absorbs Solus's **Curated Rolling Release model**, ensuring developers receive weekly software updates that have undergone automated testing, rather than continuous, raw upstream changes.

---

## 2. Key Features to Absorb

### 2.1 Curated Rolling Updates

Instead of continuous daily updates (like Arch) that can break systems, SigmaOS updates roll out on a predictable weekly cadence (`sigma-pkg update`).

Every package update in the testing channel must successfully pass the automated testing suite (`sigma-test`) before being merged into the stable user channel, ensuring users have the absolute newest software with none of the typical rolling-release instability.

### 2.2 Budgie-Inspired Modular UI

Solus’s Budgie desktop prioritizes simplicity and integration. SigmaOS's Zenith UI inherits this mindset, designing widget and status panels as lightweight, sandboxed WASM modules that can be dragged, dropped, and configured dynamically without restarting the compositor.

---

## 3. References & Standards

- Solus Project — `getsol.us`
- Budgie Desktop — `buddiesofbudgie.org` (GPL-2.0)
