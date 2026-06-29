# Zenith Desktop

The **SigmaOS Zenith Sovereign Lattice Desktop** is the flagship desktop environment demonstrating what SigmaOS can do when all platform features are assembled together. It runs entirely in the browser — no Electron, no GTK, no Qt — using the SigmaOS web shell as its compositor.

---

## Overview

Zenith is experimental and forward-looking. It showcases features that most operating systems don't have yet:

- **Post-quantum silicon attestation** (Kyber-1024) — every session is cryptographically bound to the hardware.
- **Persistent shard matrix** — storage is organized as a distributed lattice of encrypted shards, not a flat filesystem.
- **Neural UI Engine** — the dock and widget placement adapt in real time based on your usage patterns, powered by the local `sigmad-ai` inference daemon.
- **Reactive system events** — the desktop automatically responds to events like low battery, network changes, or storage pressure without user intervention.
- **Dynamic theme engine** — themes update live based on time of day, wallpaper color palette, and system state.

---

## Architecture

```
┌──────────────────────────────────────────────────────┐
│              Zenith Shell (React + Svelte)            │
│  Dock · Taskbar · Notification Center · Lock Screen  │
│  Workspace Switcher · Status Bar · Spotlight Search  │
├──────────────────────────────────────────────────────┤
│               Neural UI Engine                        │
│  sigmad-ai → /v1/predict → dock layout suggestions   │
│  responds to sigma-update-dock-suggestions events     │
├──────────────────────────────────────────────────────┤
│             Sovereign Lattice Storage                  │
│  Persistent shard matrix (encrypted, distributed)    │
│  Atomic snapshot differential engine                  │
├──────────────────────────────────────────────────────┤
│           Silicon Attestation Layer                   │
│  Kyber-1024 keypair bound to hardware fingerprint    │
│  Session tokens required for cross-shard comms        │
└──────────────────────────────────────────────────────┘
```

---

## Silicon Attestation (Kyber-1024)

On boot, Zenith generates a fresh **Kyber-1024** keypair. Kyber-1024 is a post-quantum key encapsulation mechanism — it is resistant to attacks from both classical and quantum computers.

The keypair is bound to the current hardware fingerprint (TPM measurement or CPU serial). This means:
- A shard encrypted on this machine cannot be decrypted on a different machine.
- Even if the raw shard data is copied off-device, it is unreadable without the original hardware.

```
Boot
  │
  ├─ Generate Kyber-1024 keypair (hardware-bound)
  ├─ Derive session key from keypair + hardware fingerprint
  ├─ Decrypt shard index using session key
  └─ Mount virtual filesystem from decrypted shard index
```

---

## Persistent Shard Matrix

Instead of a traditional flat filesystem, Zenith organizes persistent storage as a **shard matrix** — a grid of fixed-size, independently encrypted blocks. Benefits:

- **Atomic snapshots**: Capturing a point-in-time snapshot is O(1) — just record which shard versions are current.
- **Differential sync**: Only modified shards are transferred during cloud sync, minimizing bandwidth.
- **Crash recovery**: A crash corrupts at most one shard. All others remain readable.

The shard list is tracked in `SHARDS.manifest`. Each entry contains:

```json
{
  "id": "shard-0042",
  "hash": "sha256:abc123...",
  "encrypted": true,
  "size": 4096,
  "last_modified": 1719400000
}
```

---

## Neural UI Engine

The Zenith dock is not static. It listens for `sigma-update-dock-suggestions` events dispatched by the Neural UI Engine, which queries `sigmad-ai` at `localhost:17392/v1/predict` with a feature vector describing recent app usage.

The engine predicts which apps you are likely to need next and reorders or highlights them in the dock accordingly.

```js
// Neural UI Engine flow
const prediction = await navigator.sigmaos.ai.predict({
  model: "sigma-ui-v1",
  features: encodeUsageVector(recentAppLaunches, currentTime, batteryState)
});

// Dispatch to dock
window.dispatchEvent(new CustomEvent("sigma-update-dock-suggestions", {
  detail: { suggestions: prediction.ranked_apps }
}));
```

If `sigmad-ai` is unavailable, the engine falls back to a static frequency-based ordering and shows a small status indicator.

---

## Reactive System Events

Zenith subscribes to system events from the SigmaOS daemons and reacts automatically:

| Event | Automatic response | 
| --- | --- | 
| Battery < 15% | Switch to dark theme, reduce animation rate, notify user | 
| Battery < 5% | Freeze all non-critical background apps, prompt to save work | 
| Network disconnected | Show sync-status as "Disconnected", pause cloud sync | 
| Storage > 90% full | Highlight the largest shards in the Storage Manager widget | 
| New USB device | Show hotplug notification with mount/open/eject options | 
| IDS alert from sigma_zerotrust | Flash red status in the security widget, log to audit view | 

---

## Dynamic Theme Engine (`SovereignThemeEngine`)

Themes in Zenith are **hardware-accelerated CSS variables** updated in real time. The theme engine:

1. Extracts the dominant color palette from the current wallpaper using a fast k-means pass.
2. Derives complementary UI colors (accent, surface, text) using OKLCH color space for perceptual uniformity.
3. Applies the palette as CSS custom properties on `document.documentElement`.
4. Transitions smoothly between themes when the wallpaper changes or the time of day crosses a threshold (e.g., automatic dark mode at sunset).

```css
/* Applied by SovereignThemeEngine */
:root {
  --sigma-accent:    oklch(65% 0.2 260);
  --sigma-surface:   oklch(15% 0.01 260);
  --sigma-text:      oklch(95% 0.01 260);
  --sigma-glow:      oklch(65% 0.2 260 / 30%);
}
```

---

## Zenith Desktop Components

| Component | Description | 
| --- | --- | 
| Dock | Bottom launcher with AI-suggested app ordering | 
| Taskbar | Top bar with workspace switcher, clock, status icons | 
| Notification Center | Slide-out panel with timestamped notifications and badge counter | 
| Lock Screen | Full-screen overlay (Ctrl+L) with clock and PIN/credential unlock | 
| Spotlight Search | Cmd+Space natural language search powered by `sigmad-ai` | 
| Window Manager | In-page draggable/resizable iframes (no `window.open()`) | 
| Control Center | Quick-access panel for WiFi, Bluetooth, brightness, volume | 
| Storage Manager | Visual shard matrix browser with size and encryption status | 
| Security Dashboard | Live view of zero-trust policy decisions and audit log | 

---

*See also: [Architecture Overview](Architecture-Overview) · [Security Model](Security-Model) · [API Reference](API-Reference)*
