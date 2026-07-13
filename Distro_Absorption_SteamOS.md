# Distro Absorption: SteamOS

> **Status**: 📋 Planned | **Source Paradigm**: SteamOS (Valve) | **Target Shard**: `SigmaOS Graphics & Gaming Layer`

---

## 1. Executive Summary

SteamOS (v3+) is a gaming-focused Linux distribution built by Valve. It features an immutable read-only root filesystem, flatpak-centric userland software updates, and a custom micro-compositor (Gamescope) optimized for gaming performance and frame pacing.

SigmaOS absorbs SteamOS's **nested session compositing (Gamescope model)** and **developer overlays for read-only rootfs**, enabling maximum graphical performance for isolated graphical sessions.

---

## 2. Key Features to Absorb

### 2.1 Nested Micro-Compositor (Zenith-Gamescope)

In standard Linux, running a game inside a desktop environment introduces compositor latency and frame drops. SigmaOS implements a nested Wayland compositor mode. When a game launches, Zenith spawns a lightweight micro-compositor specifically for that process.

```bash
$ sigma run --game legacy-game.exe
Σ [ZENITH] Launching game-specific micro-compositor...
  Frame pacing: Hardware-locked (VRR active)
  Resolution scaling: FSR 2.1 (AMD FidelityFX)
  DirectX translation: DXVK (Direct3D 11 → Vulkan)
```

The game is rendered to a virtual container frame, allowing it to bypass desktop window-manager latency entirely and communicate directly with the GPU display queues.

### 2.2 Developer Overlays (`sigma-overlay`)

While the core OS partition is read-only (ensuring stability), developers can temporarily mount a read-write overlay (`overlayfs` style) to modify system-level libraries for debugging without changing the base image.

```bash
$ sigma developer-mode enable --rw
Σ [SYSTEM] Dev-Mode Active: Read-write overlay mounted on /usr
  Changes will persist until dev-mode is disabled.
```

---

## 3. References & Standards

- SteamOS — `valvesoftware.com/steamos`
- Gamescope — `github.com/ValveSoftware/gamescope` (BSD-2-Clause)
