# 🔒🖥️ SIGMAOS SCREENSAVER & SCREENLOCKER DEVELOPMENT BLUEPRINT
## Comprehensive Linux & BSD-Inspired Architecture, Security Framework, and Implementation Specification for https://github.com/AaryanSinghChauhan09/SigmaOS

---

## EXECUTIVE SUMMARY & ARCHITECTURAL VISION

In desktop operating systems, the screensaver and screen locker subsystem serves two critical roles:
1. **Security & Privacy Protection**: Locking system sessions during inactivity, requiring PAM/biometric authentication, and preventing unauthorized visual inspection or local input access.
2. **Display Power & Hardware Management**: Transitioning displays through VESA DPMS (Display Power Management Signaling) states (On, Standby, Suspend, Off) to conserve energy and prevent OLED/CRT image burn-in, while maintaining smooth hardware-accelerated animations when active.

SigmaOS adopts a hybrid architecture drawing inspiration from **Linux** (`xscreensaver`, `gnome-screensaver`, `kscreenlocker`, `swaylock`/`hyprlock` with `ext-session-lock-v1`, and the `org.freedesktop.ScreenSaver` DBus API) and **BSD distributions** (FreeBSD/OpenBSD PAM authentication, secure memory wipes via `explicit_bzero`, and VESA DPMS kernel state machines).

This blueprint establishes the master development plan for the SigmaOS ScreenSaver Engine (`src/desktop/screensaver.rs`).

---

## PART 1: COMPARATIVE GAP ANALYSIS & DISTRO INSPIRATION

### 1. Linux Distro Inspirations
- **`xscreensaver` (Jamie Zawinski / Classic UNIX)**: Modular architecture separating display animations from the lock engine; strict client-server separation; screen blanking and DPMS coordination.
- **Wayland `ext-session-lock-v1` Protocol (Swaylock / Hyprlock / KDE KScreenLocker)**: Security-first protocol guaranteeing that when locked, the compositor renders ONLY the screen locker's surfaces. Direct client input is isolated, preventing malicious background apps from spying on keystrokes or bypassing the lock screen.
- **`org.freedesktop.ScreenSaver` DBus Protocol**: Industry-standard IPC interface allowing web browsers (Chromium, Firefox), media players (mpv, VLC), and presentation engines to temporarily inhibit screensaver/DPMS activation via `Inhibit(app_name, reason)` and `Uninhibit(cookie)`.

### 2. BSD Distro Inspirations
- **FreeBSD / OpenBSD PAM & BSD Auth**: Pluggable Authentication Modules allowing secure local passphrase verification, post-quantum cryptographic key checking, and multi-factor hardware tokens (YubiKey / FIDO2).
- **Secure Memory Sanitization (`explicit_bzero` / Rust zeroize)**: Passphrase entry buffers in memory are zeroed out immediately after authentication attempts to prevent cold-boot memory dumps or swap page exposure.
- **Kernel DPMS Hardware Interface**: Direct interaction with DRM/KMS connector power states to power down backlight controllers cleanly without corrupting display timing modes.

---

## PART 2: CORE ARCHITECTURAL PILLARS FOR SIGMAOS

```
                    +------------------------------------------+
                    |  SIGMAOS SCREENSAVER & LOCKER DAEMON     |
                    +------------------------------------------+
                                         |
     +-------------------+---------------+---------------+-------------------+
     |                   |               |               |                   |
     v                   v               v               v                   v
🎮 ANIMATION       🔒 WAYLAND LOCK   ⚡ DPMS POWER   🛑 DBUS INHIBITOR   🔑 AUTHENTICATION
  ENGINE             PROTOCOL          CONTROLLER       MANAGER             ENGINE
  • Matrix Rain      • ext-session     • On (0s)        • Chrome/Firefox    • PAM Verification
  • Starfield          lock-v1         • Standby        • mpv / VLC         • Passphrase Hash
  • Custom Canvas    • Input Isolation • Suspend        • Cookie Track      • Buffer Zeroing
  • Capped FPS       • Ring 3 Sandbox  • Off (DPMS)     • Auto-Release      • Anti-Bruteforce
```

---

## PART 3: 4-PHASE DEVELOPMENT ROADMAP

### PHASE 1: Core Daemon, Idle Tracking & DPMS State Machine
- **Idle Time Monitoring**: Integrate system-wide user activity events (keyboard keypresses, mouse movements, touchscreen taps) into a centralized idle timer.
- **DPMS Cascading Power States**: Implement configurable threshold timers for transitioning displays from `On` -> `Standby` -> `Suspend` -> `Off`.
- **Configurable Timeouts**: Default 5 min screensaver, 10 min lock screen, 15 min DPMS Standby, 20 min DPMS Suspend, 30 min DPMS Off.

### PHASE 2: Wayland `ext-session-lock-v1` Protocol & PAM Authentication
- **Compositor Surface Lock**: Implement Wayland `ext-session-lock-v1` protocol client shims ensuring locked states block all desktop surface drawing and input event routing to background processes.
- **PAM Authentication Integration**: Perform secure password authentication with `hashed_passphrase` comparisons and strict rate-limiting / lockout policies on repeated failed attempts.
- **Zero-Copy Memory Scrubbing**: Ensure passphrase string buffers are wiped from memory immediately after verification.

### PHASE 3: Hardware-Accelerated Animations & Low-Power Rendering
- **Animation Render Modes**: Support `Blank`, `MatrixRain`, `Starfield`, `ColorCycles`, and `Custom` shader modes.
- **Frame Rate Capping**: Automatically cap animation frame rate (e.g. 30 FPS / 60 FPS) to avoid unnecessary CPU/GPU power consumption during idle screensaver state.
- **Dynamic Clock & Notification Display**: Optional minimal lock screen clock and privacy-respecting notification counters.

### PHASE 4: DBus Inhibitor Spec (`org.freedesktop.ScreenSaver`) & Media Integrations
- **Inhibit / Uninhibit IPC**: Implement `Inhibit(app_name, reason) -> cookie` and `Uninhibit(cookie)` methods.
- **Media Player Integration**: Automatically detect active media playback (e.g., video playing via PipeWire / mpv) to suspend idle timers automatically.
- **Zenith Desktop Integration**: Provide quick shortcuts (`Ctrl+Alt+L` / `Super+L`) and system tray controls for manual locking and screensaver mode selection.

---

## PART 4: VERIFICATION BENCHMARK & TEST CRITERIA

1. **Idle Activation Unit Tests**: Verify that idle time updates trigger screensaver mode, lock state, and DPMS state transitions at configured second thresholds.
2. **Authentication Safety Unit Tests**: Validate correct passphrase verification, failed attempt lockouts, and buffer memory zeroing.
3. **Inhibitor Unit Tests**: Ensure `Inhibit` blocks screensaver activation despite user idle time exceeding configured thresholds until `Uninhibit` is issued.
4. **Wayland Lock Isolation**: Guarantee that session lock prevents input leaking to underlying Zenith desktop components.

---
*End of SigmaOS ScreenSaver Development Blueprint Specification.*
