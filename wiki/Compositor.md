# Compositor

**Capability state: Proposed.** No verified graphical boot or compositor-to-display path exists. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

Desktop configuration and graphics models do not establish a running compositor, Wayland server, DRM/KMS backend, GPU acceleration, or window-management session. No latency, frame-rate, or power comparison with Linux Mint, Omarchy, or other desktops has been measured.

## Design references

Study Wayland's client/server boundaries, Linux DRM/KMS integration, Omarchy's keyboard-first defaults, and Mint's familiar session behavior. Keep input, rendering, window policy, and display backends independently testable.

## Roadmap

1. Bring up a text/serial boot and a framebuffer on one QEMU target.
2. Add a minimal display server and input path with clear lifecycle and error reporting.
3. Implement keyboard-operable focus and window actions before adding optional effects.
4. Validate resolution changes, multiple outputs, session failure, and recovery on named devices.

**Completion evidence:** the booted system starts a session, accepts input, displays and manages a client, and recovers from backend/client failures on documented configurations. See [Desktop](08-Desktop.md) and [Audio and Graphics](Audio-and-Graphics.md).
