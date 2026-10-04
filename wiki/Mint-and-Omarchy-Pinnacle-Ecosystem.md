# Mint & Omarchy Pinnacle Ecosystem

The **Mint & Omarchy Pinnacle Ecosystem Suite** ([`src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs)) represents the state-of-the-art fusion of Linux Mint's user data sovereignty and biometric desktop security with Omarchy's bleeding-edge Proton gaming prefixes and 1:1 kinetic gesture engine.

---

## 1. Overview & Architectural Superiority

| Functional Domain | Linux Mint (Cinnamon/Ubuntu) | Omarchy (Hyprland/Arch) | **SigmaOS Pinnacle Ecosystem** |
| :--- | :--- | :--- | :--- |
| **Personal Backup & Migration** | Python `mintbackup` (tar-based) | Manual dotfiles / rsync | **Content-addressed manifest with explicit package export** |
| **Proton / Wine Gaming Engine** | Generic Steam client | Manual Lutris / Heroic scripts | **Automated prefix lifecycle with FSR 3.1 & Anti-Lag+ pacing** |
| **Screen Locker & Bio-Auth** | `cinnamon-screensaver` (PAM only)| Swaylock / Hyprlock (PAM only) | **Multi-factor biometrics (Fingerprint, IR Face, FIDO2) & media controls** |
| **Multi-Touch Gestures** | Basic libinput 3-finger swipe | Hyprland touch gestures | **1:1 kinetic spring physics for workspaces & window expose** |

---

## 2. Subsystem Specifications

### A. Sovereign Mint Backup Engine (`SovereignMintBackupEngine`)
*Source: [`src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs#L34-L122)*

Reimplements and extends `mintbackup`:
* **Tri-Tier Backup Modes**:
  * `PersonalDataOnly`: User home directory documents, media, and configurations.
  * `SoftwareSelectionOnly`: Manifest listing explicitly installed packages across `sigma-core`, `flathub`, and `aur`.
  * `FullSystemSnapshot`: Complete root and userland state snapshot.
* **Portable Migration Manifest**: Exports software selection manifests into deterministic records (`SoftwareSelectionRecord`) allowing one-command reinstallation on newly provisioned machines.
* **Integrity Guarantee**: Content-addressed SHA-256 manifest hash calculation with zero data loss.

### B. Sovereign Omarchy Gaming Prefix Manager (`SovereignOmarchyGamingPrefixManager`)
*Source: [`src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs#L124-L215)*

Direct integration for Proton and Wine prefixes:
* **Proton Runner Switcher**: Seamless switching between `ProtonGeLatest`, `ProtonExperimental`, `WineStaging`, and `NativeVulkanDirect`.
* **Real-Time Graphics Upscaling**: Per-game automated toggles for `FSR1Spatial`, `FSR2Temporal`, `FSR3FrameGeneration`, and `NisNvidiaImageScaling`.
* **Ultra-Low Latency Frame Pacing**: NVIDIA Reflex and AMD Anti-Lag+ equivalent frame submission pacing (`LatencyMitigationMode::UltraLowLatencyBoost`), eliminating input queue latency.
* **Shader Pre-Caching**: Pre-compiles SPIR-V and DirectX bytecode shaders prior to game execution to guarantee zero shader-compilation stutter.

### C. Sovereign Cinnamon Screen Lock & Bio-Auth (`SovereignCinnamonScreenLockAndBioAuth`)
*Source: [`src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs#L217-L295)*

* **Biometric Multi-Factor Authentication**:
  * `FingerprintBiometric` (`fprintd` zero-latency driver hook).
  * `IrFaceRecognition` (howdy-compatible IR camera facial verification).
  * `Fido2HardwareKey` (hardware cryptographic security key challenge-response).
  * `Passphrase` fallback with Argon2id KDF.
* **Integrated Lockscreen Media Widget**: Displays live playback track metadata, position, and controls without unlocking the desktop session.
* **Emergency Console Bypass**: Prevents lockscreen deadlocks through secure supervisor escalation.

### D. Sovereign Omarchy Gesture Engine (`SovereignOmarchyGestureEngine`)
*Source: [`src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs#L297-L380)*

* **1:1 Smooth Kinetic Interpolation**:
  * 3-Finger Horizontal Swipe: Workspace transition following finger motion with spring physics.
  * 3-Finger Vertical Swipe: Smooth window expose / overview display.
  * 4-Finger Pinch: Instant show desktop gesture.
  * Touchscreen Edge Swipe: Quick settings and notification center drawer.
* **High-Precision Velocity Tracking**: Dynamically computes swipe momentum to execute natural completion or bounce-back animations.

---

## 3. Source Code Reference

* Implementation: [`src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs)
* Registration: [`src/distro/mod.rs`](file:///home/aaryansinghchauhan/SigmaOS/src/distro/mod.rs#L399)

---

## 4. AI Agent Maintenance Instructions

> **For AI Agents Maintaining This Page:**
> - Source: `src/distro/sovereign_mint_omarchy_pinnacle_ecosystem.rs`
> - When new Proton runner versions or upscaling technologies (e.g. FSR 4) are added, update the gaming configuration table.
> - Ensure biometric authentication timeout and security policies conform to ISO 27001 standards.
> - Cross-reference with [Omarchy-Gaming-Performance-Suite](Omarchy-Gaming-Performance-Suite.md) and [Security](07-Security.md).
