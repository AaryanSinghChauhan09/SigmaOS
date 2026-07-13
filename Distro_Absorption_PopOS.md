# Distro Absorption: Pop!_OS

> **Status**: 📋 Planned | **Source Paradigm**: Pop!_OS (System76) | **Target Shard**: `SigmaOS Creator Profile`

---

## 1. Executive Summary

Pop!_OS, developed by hardware vendor System76, is acclaimed for its out-of-the-box support for hybrid graphics, deep learning frameworks, and an incredibly productive, keyboard-driven auto-tiling workflow (Cosmic).

SigmaOS absorbs the **System76-Scheduler** logic and **Hybrid Graphics Switching** to provide maximum battery life without sacrificing GPU horsepower when needed by professionals.

---

## 2. Key Features to Absorb

### 2.1 Context-Aware CPU Scheduling

Standard Linux schedulers treat all processes relatively equally. SigmaOS integrates a Rust-based scheduler daemon (inspired by `system76-scheduler`) that dynamically adjusts CPU affinities and nice values based on the active GUI window.

If the user focuses on a heavy compile job, background rendering tasks are instantly deprioritized.

### 2.2 Seamless Hybrid Graphics

Managing Nvidia Optimus / Hybrid graphics on Linux is historically painful. SigmaOS implements a native UI toggle and IPC bus that handles the `prime-run` offloading transparently.

```bash
$ sigma gpu set --mode hybrid
Σ [GPU] Hybrid mode enabled. 
  Integrated GPU (Intel) handling compositor.
  Discrete GPU (Nvidia) powered down (D3Cold state).
  Launch apps with `sigma run --gpu` to wake discrete graphics.
```

### 2.3 Auto-Tiling Zenith

The SigmaOS Zenith compositor includes a native, toggleable auto-tiling mode inspired by Pop!_OS Cosmic, allowing power users to navigate entirely via the keyboard without manually arranging floating windows.

---

## 3. References & Standards

- Pop!_OS — `pop.system76.com`
- system76-scheduler (GPL-3.0)
